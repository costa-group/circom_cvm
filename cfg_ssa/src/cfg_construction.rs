use std::collections::{HashMap, HashSet};

use crate::{ast::ASTNode, types::{Atomic, Expression, Parameter}, OperatorOrPhi, Statement, Value, CFG};

/// Enum to represent the possible uses: In a statement (line in the block) or in a condition of
/// the block
enum Use {
    InDeclaration(String),
    InStmt(usize),
    InCondition,
}

/// A SSA-based CFG builder
pub struct CfgConstructor<'a> {
    cfg: &'a mut CFG,

    /// Save for each block the definition of the variables it contains (the new name)
    definitions: Vec<HashMap<String, String>>,

    /// Current SSA values: name -> concrete Value
    values: HashMap<String, Value>,

    /// Tracks unfinished φ-nodes per block (tracked variable and its φ-node)
    incomplete_phis: Vec<HashSet<(String, String)>>,

    /// For each variable, we save a set of blocks where it is used and how
    uses: HashMap<String, HashSet<(usize, Use)>>,

    sealed_blocks: Vec<bool>,
    next_var: usize,
}


impl<'a> CfgConstructor<'a> {
    pub fn new(cfg: &'a mut CFG) -> Self {
        Self {
            cfg,
            definitions: vec![HashMap::new()],
            values: HashMap::new(),
            incomplete_phis: vec![HashSet::new()],
            uses: HashMap::new(),
            sealed_blocks: vec![true],
            next_var: 0,
        }
    }

    //Create a new block for the cfg, but, at the same time, increase the size of the block
    //variable definition
    fn create_block(&mut self) -> usize {
        self.definitions.push(HashMap::new());
        self.incomplete_phis.push(HashSet::new());
        self.sealed_blocks.push(false);
        self.cfg.create_new_block()
    }

    /// Create a fresh SSA name
    fn fresh(&mut self) -> String {
        let new_var = format!("v{}", self.next_var);
        self.next_var += 1;
        new_var
    }

    /// Seal a block: finish φ-nodes
    fn seal_block(&mut self, block: usize) {
        let pending = std::mem::take(&mut self.incomplete_phis[block]);
        for (name, phi) in pending {
            self.add_phi_operands(&name, &phi, block);
        }
        self.sealed_blocks[block] = true;
    }

    /// Write a new SSA binding for source `src` in `block` with `val`
    fn write_variable(&mut self, src: &str, block: usize, val: Value) -> String {
        let dest = self.fresh();
        self.values.insert(dest.clone(), val);
        self.definitions[block].insert(src.to_string(), dest.clone());
        dest
    }

    /// Read a variable, inserting φ if needed
    fn read_variable(&mut self, name: &str, block: usize) -> String {
        if let Some(ssa) = self.definitions[block].get(name) {
            return ssa.clone();
        }
        self.read_recursive(name, block)
    }

    fn read_recursive(&mut self, name: &str, block: usize) -> String {
        if !self.sealed_blocks[block] {
            // Incomplete block: create φ placeholder
            let val = Value { operator: Some(OperatorOrPhi::Phi), operands: Vec::new() };
            let phi = self.write_variable(name, block, val);
            self.incomplete_phis[block].insert((name.to_string(), phi.clone()));
            return phi;
        }
        let preds = self.cfg.predecessors(block);
        if preds.len() == 1 {
            // Single predecessor -> no φ needed
            let v = self.read_variable(name, preds[0]);
            // TODO: Why does Braun et al write the var in the current block?
            // Maybe to avoid the recursive lookup in future cases?
            // Don't write_variable because it creates a new SSA name 
            self.definitions[block].insert(name.to_string(), v.clone());
            return v;
        }
        // Multiple predecessors -> φ
        // Break potential cycles with operandless phi
        let val = Value { operator: Some(OperatorOrPhi::Phi), operands: Vec::new() };
        let phi = self.write_variable(name, block, val);
        self.add_phi_operands(name, &phi, block);
        // In case phi is trivial, we read again
        self.read_variable(name, block)
    }

    /// Add φ operands from all predecessors for `name` in `block`
    fn add_phi_operands(&mut self, name: &str, phi: &str, block: usize) {
        let mut ops = Vec::new();
        let predecessors: Vec<_> = self.cfg.predecessors(block).to_vec();
        for &pred in &predecessors {
            //TODO: Avoid pushing trivial phis
            ops.push(self.read_variable(name, pred));
        }
        let entry = self.values.get_mut(phi).expect("Phi node missing");
        entry.operands.extend(ops.into_iter().map(|op| Expression::Atomic(Atomic::Variable(op))));

        let entry_clone = entry.clone();
        //TODO: Fix remove trivial
        if !self.try_remove_trivial(phi, name, block) {
            //TODO: Does this go here?
            let stmt = Statement { num_type: None, output: Some(phi.to_string()), value: entry_clone };
            self.cfg.add_phi_function(block, stmt);
        }
    }

    /// Simplify trivial φ-nodes with identical operands
    fn try_remove_trivial(&mut self, phi: &str, name: &str, block: usize) -> bool{
        let val = match self.values.get(phi) {
            Some(v) if v.is_phi() => v.clone(),
            _ => return false,
        };
        let mut same: Option<String> = None;
        for op in &val.operands {
            if let Expression::Atomic(Atomic::Variable(var)) = op {
                if var == phi { continue; }     //Self reference
                if let Some(r) = &same {
                    //The phi merges at least two different values: not trivial
                    if r != var { return false; }
                }
                same = Some(var.clone());
            }
            else {
                panic!("Phi functions should only have variables as operands");
            }
        }

        if let Some(same_v) = same {
            if let Some(users) = self.uses.remove(phi) {
                for (block_u, user) in users {
                    match user {
                        Use::InDeclaration(user_name) => {
                            self.cfg.change_declaration_operands(block_u, &user_name, phi, &same_v);
                            let name_ssa = self.definitions[block_u].get(&user_name).expect("Declaration not found").clone();
                            let _ = self.try_remove_trivial(&user_name, &name_ssa, block_u);
                        }
                        Use::InStmt(line) => {
                            self.cfg.change_statement_operands(block_u, line, phi, &same_v);
                        }
                        Use::InCondition => {
                            self.cfg.change_condition(block_u, phi, &same_v);
                        }
                    }
                }
            }
            self.definitions[block].insert(name.to_string(), same_v);
            self.values.remove(phi);
        }
        //TODO: Case of same is None → Remove its users recursively
        true
    }

    fn read_expression(&mut self, op: &Expression, block: usize) -> Expression {
        match op {
            Expression::Atomic(Atomic::Variable(var)) => {
                Expression::Atomic(Atomic::Variable(self.read_variable(var, block)))
            }
            Expression::Atomic(atomic) => Expression::Atomic(atomic.clone()),
            Expression::Parameter(param) => {
                let mut new_param = param.clone();
                let mut update_atomic = |a: &mut Atomic| {
                    if let Atomic::Variable(var) = a {
                        *var = self.read_variable(var, block);
                    }
                };

                match &mut new_param {
                    Parameter::Signal { index, size }
                    | Parameter::I64Memory { index, size }
                    | Parameter::FfMemory { index, size } => {
                        update_atomic(index);
                        update_atomic(size);
                    }
                    Parameter::SubcmpSignal { component, index, size } => {
                        update_atomic(component);
                        update_atomic(index);
                        update_atomic(size);
                    }
                }
                Expression::Parameter(new_param)
            }
        }
    }

    /// Process AST nodes into CFG, returning the last block
    /// loop_blocks: (entry, exit)
    pub fn process_body(&mut self, body: &[ASTNode], mut curr: usize, loop_blocks: Option<(usize, usize)>) -> usize {
        for stmt in body {
            curr = match stmt {
                ASTNode::Operation { num_type, operator, output, operands } => {
                    self.handle_operation(curr, num_type, operator, output, operands)
                }
                ASTNode::Loop { body: loop_body } => self.handle_loop(loop_body, curr),
                ASTNode::IfThenElse { condition, if_case, else_case } => {
                    let cond = self.read_expression(condition, curr);
                    self.handle_if(&cond, if_case, else_case, curr, loop_blocks)
                }
                ASTNode::Break => {
                    let out = loop_blocks.expect("break outside loop").1;
                    self.cfg.add_uncond_link(curr, out);
                    break;
                }
                ASTNode::Continue => {
                    let entry = loop_blocks.expect("continue outside loop").0;
                    self.cfg.add_uncond_link(curr, entry);
                    break;
                }
            };
        }

        curr
    }

    fn handle_operation(
        &mut self, curr: usize,
        num_type: &Option<crate::types::NumericType>,
        operator: &Option<crate::types::Operator>,
        output: &Option<String>,
        operands: &Vec<Expression>,
    ) -> usize {
        //TODO: Track use of operands
        let mut ops = Vec::new();
        for op in operands {
            ops.push(self.read_expression(op, curr));
        }
    
        let val = Value { operator: operator.clone().map(OperatorOrPhi::Operator), operands: ops };
    
        let var;
        if let Some(v) = output {
            var = Some(self.write_variable(v, curr, val.clone()));
        }
        else {
            var = None;
        }
    
        let stmt = Statement { num_type: num_type.clone(), output: var, value: val };
        self.cfg.add_instruction(curr, stmt);
    
        curr
    }
    
    /// Helper: create a new block and link from `curr`
    fn create_and_link(&mut self, curr: usize) -> usize {
        let b = self.create_block();
        self.cfg.add_uncond_link(curr, b);
        b
    }

    fn handle_loop(&mut self, loop_body: &[ASTNode], curr: usize) -> usize {
        // Add new block for the loop body (if the current is not empty)
        let entry = self.create_and_link(curr);

        // Add new block for the instructions after the loop
        let after = self.create_block();

        // The block that will be the last one in the loop
        let last = self.process_body(loop_body, entry, Some((entry, after)));
        if self.cfg.predecessors(last).len() > 1 {
            self.cfg.add_uncond_link(last, after);
        }

        // Seal the blocks
        self.seal_block(entry);
        self.seal_block(after);

        // Continue after the loop
        after
    }

    fn handle_if(
        &mut self,
        condition: &Expression,
        if_case: &[ASTNode],
        else_case: &Option<Vec<ASTNode>>,
        curr: usize,
        loop_blocks: Option<(usize, usize)>,
    ) -> usize {
        // If body
        let then_b = self.create_block();

        // Convergence block
        // TODO: Create this always or only when there are more instructions?
        let join = self.create_block();

        // Add else block (optional)
        let else_b = if else_case.is_some() { self.create_block() }
                            else { join };

        // Link current block with the then and else blocks
        self.cfg.add_cond_link(curr, condition.clone(), then_b, else_b);

        // Seal the blocks that start then and else
        self.seal_block(then_b);
        // Ensure the join is not accidentally sealed
        if else_case.is_some() {
            self.seal_block(else_b);
        }

        // Process then case
        let end_then = self.process_body(if_case, then_b, loop_blocks);
        self.cfg.add_uncond_link(end_then, join);

        if let Some(else_stmts) = else_case {
            let end_else = self.process_body(else_stmts, else_b, loop_blocks);
            self.cfg.add_uncond_link(end_else, join);
        }

        // Seal join
        self.seal_block(join);

        join
    }

    pub fn remove_unreachable_blocks(&mut self) {
        todo!();
    }
}

#[cfg(test)]
mod tests {
    use num_bigint_dig::BigInt;

    use crate::{ast::Template, types::*};

    use super::*;

    #[test]
    fn test_simple_ast() {
        let template = Template {
            id: 0,
            name: "template_0".to_string(),
            outputs: vec!["output".to_string()],
            inputs: vec!["input1".to_string(), "input2".to_string()],
            signals: 10,
            components: vec![5, 3, 2],
            body: vec![
            ASTNode::Operation {
                num_type: Some(NumericType::FiniteField),
                operator: None,
                output: Some("x".to_string()),
                operands: vec![Expression::Atomic(Atomic::Constant(ConstantType::FF(BigInt::from(1))))],
            },
            ASTNode::Operation {
                num_type: Some(NumericType::FiniteField),
                operator: None,
                output: Some("y".to_string()),
                operands: vec![Expression::Atomic(Atomic::Constant(ConstantType::FF(BigInt::from(10))))],
            },
            ASTNode::Operation {
                num_type: Some(NumericType::FiniteField),
                operator: None,
                output: Some("z".to_string()),
                operands: vec![Expression::Atomic(Atomic::Constant(ConstantType::FF(BigInt::from(9))))],
            },
            ASTNode::Operation {
                num_type: Some(NumericType::FiniteField),
                operator: Some(Operator::Add),
                output: Some("a".to_string()),
                operands: vec![
                Expression::Atomic(Atomic::Variable("y".to_string())),
                Expression::Atomic(Atomic::Variable("z".to_string())),
                ],
            },
            ASTNode::Operation {
                num_type: Some(NumericType::FiniteField),
                operator: None,
                output: Some("condition".to_string()),
                operands: vec![Expression::Atomic(Atomic::Constant(ConstantType::I64(1)))],
            },
            ASTNode::Operation {
                num_type: Some(NumericType::FiniteField),
                operator: None,
                output: Some("loop_condition".to_string()),
                operands: vec![Expression::Atomic(Atomic::Constant(ConstantType::I64(1)))],
            },
            ASTNode::IfThenElse {
                condition: Expression::Atomic(Atomic::Variable("condition".to_string())),
                if_case: vec![
                ASTNode::Operation {
                    num_type: Some(NumericType::FiniteField),
                    operator: Some(Operator::Sub),
                    output: Some("b".to_string()),
                    operands: vec![
                    Expression::Atomic(Atomic::Variable("x".to_string())),
                    Expression::Atomic(Atomic::Variable("y".to_string())),
                    ],
                },
                ],
                else_case: None,
            },
            ASTNode::Loop {
                body: vec![
                ASTNode::Operation {
                    num_type: Some(NumericType::FiniteField),
                    operator: Some(Operator::Mul),
                    output: Some("b".to_string()),
                    operands: vec![
                        Expression::Atomic(Atomic::Variable("x".to_string())),
                        Expression::Atomic(Atomic::Variable("z".to_string())),
                    ],
                },
                ASTNode::IfThenElse {
                    condition: Expression::Atomic(Atomic::Variable("loop_condition".to_string())),
                    if_case: vec![
                        ASTNode::Operation {
                            num_type: Some(NumericType::FiniteField),
                            operator: Some(Operator::Div),
                            output: Some("c".to_string()),
                            operands: vec![
                            Expression::Atomic(Atomic::Variable("x".to_string())),
                            Expression::Atomic(Atomic::Variable("z".to_string())),
                            ],
                        },
                        ASTNode::Break,
                    ],
                    else_case: Some(vec![
                        ASTNode::Operation {
                            num_type: Some(NumericType::FiniteField),
                            operator: Some(Operator::Sub),
                            output: Some("d".to_string()),
                            operands: vec![
                            Expression::Atomic(Atomic::Variable("x".to_string())),
                            Expression::Atomic(Atomic::Variable("z".to_string())),
                            ],
                        },
                        ASTNode::Continue,
                    ]),
                },
                ],
            },
            ASTNode::Operation {
                num_type: Some(NumericType::FiniteField),
                operator: Some(Operator::Add),
                output: Some("x".to_string()),
                operands: vec![
                Expression::Atomic(Atomic::Variable("y".to_string())),
                Expression::Atomic(Atomic::Variable("z".to_string())),
                ],
            },
            ],
        };
        let cfg = CFG::new_from_template(template);
        let dot_representation = cfg.to_dot();
        std::fs::write("./test/cfg_output.dot", dot_representation).expect("Unable to write DOT file");
        let json_representation = cfg.to_json();
        std::fs::write("./test/cfg_output.json", json_representation).expect("Unable to write JSON file");
    }
}
