use std::collections::{HashMap, HashSet};

use crate::{ast::ASTNode, types::{Atomic, Expression, Parameter}, OperatorOrPhi, Statement, Value, CFG};

type Stack<T> = Vec<T>;

/// A SSA-based CFG builder
pub struct CfgConstructor<'a> {
    cfg: &'a mut CFG,

    ///Stacks that save the last loop entry and exit blocks (needed for the break and continue)
    entry_loop_context: Stack<usize>,
    exit_loop_context: Stack<usize>,

    /// Save for each block the definition of the variables it contains (the new name)
    definitions: Vec<HashMap<String, String>>,

    /// Current SSA values: name -> concrete Value
    values: HashMap<String, Value>,

    /// Tracks unfinished φ-nodes per block
    incomplete_phis: Vec<HashSet<String>>,

    /// For each variable, we save the variables that make use of it
    uses: HashMap<String, HashSet<String>>,

    sealed_blocks: Vec<bool>,
    next_var: usize,
}


impl<'a> CfgConstructor<'a> {
    pub fn new(cfg: &'a mut CFG) -> Self {
        Self {
            cfg,
            entry_loop_context: Stack::new(),
            exit_loop_context: Stack::new(),
            definitions: Vec::new(),
            values: HashMap::new(),
            incomplete_phis: Vec::new(),
            uses: HashMap::new(),
            sealed_blocks: Vec::new(),
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
        for phi in pending {
            self.add_phi_operands(&phi, block);
        }
        self.sealed_blocks[block] = true;
    }

    /// Write a new SSA binding for source `src` in `block` with `val`
    fn write_variable(&mut self, src: &str, block: usize, val: Value) -> String {
        let dest = self.fresh();

        // Track uses of operands
        for op in &val.operands {
            //TODO: The rest of expressions with variables (parameters, etc.))
            if let Expression::Atomic(Atomic::Variable(var)) = op {
                self.uses.entry(var.clone()).or_default().insert(dest.clone());
            }
        }
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
            self.incomplete_phis[block].insert(phi.clone());
            return phi;
        }
        let preds = self.cfg.predecessors(block);
        if preds.len() == 1 {
            // Single predecessor -> no φ needed
            let v = self.read_variable(name, preds[0]);
            let val = self.values.get(&v).cloned().expect("Value missing");
            return self.write_variable(&v, block, val);
        }
        // Multiple predecessors -> φ
        // Break potential cycles with operandless phi
        let val = Value { operator: Some(OperatorOrPhi::Phi), operands: Vec::new() };
        let phi = self.write_variable(name, block, val);
        self.add_phi_operands(&phi, block);
        phi
    }

    /// Add φ operands from all predecessors for `phi` in `block`
    fn add_phi_operands(&mut self, phi: &str, block: usize) {
        let mut ops = Vec::new();
        let predecessors: Vec<_> = self.cfg.predecessors(block).to_vec();
        for &pred in &predecessors {
            ops.push(self.read_variable(phi, pred));
        }
        let entry = self.values.get_mut(phi).expect("Phi node missing");
        entry.operands.extend(ops.into_iter().map(|op| Expression::Atomic(Atomic::Variable(op))));
        self.try_remove_trivial(phi);
    }

    fn replace_variable_in_expression(expr: &mut Expression, target: &str, replacement: &str) {
        match expr {
            Expression::Atomic(Atomic::Variable(v)) if v == target => {
                *v = replacement.to_string();
            }
            Expression::Atomic(_) => {}
            Expression::Parameter(param) => {
                let update_atomic = |a: &mut Atomic| {
                    if let Atomic::Variable(v) = a {
                        if v == target {
                            *v = replacement.to_string();
                        }
                    }
                };

                match param {
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
            }
        }
    }

    /// Simplify trivial φ-nodes with identical operands
    fn try_remove_trivial(&mut self, phi: &str) {
        let val = match self.values.get(phi) {
            Some(v) if v.is_phi() => v.clone(),
            _ => return,
        };
        let mut same: Option<String> = None;
        for op in &val.operands {
            if let Expression::Atomic(Atomic::Variable(var)) = op {
                if var == phi { continue; }     //Self reference
                if let Some(r) = &same {
                    //The phi merges at least two different values: not trivial
                    if r != var { return; }
                }
                same = Some(var.clone());
            }
            else {
                panic!("Phi functions should only have variables as operands");
            }
        }

        if let Some(same_v) = same {
            if let Some(users) = self.uses.remove(phi) {
                for user in users {
                    if let Some(value) = self.values.get_mut(&user) {
                        for operand in value.operands.iter_mut() {
                            Self::replace_variable_in_expression(operand, &phi, &same_v);
                        }

                        if value.is_phi() {
                            self.try_remove_trivial(&user);
                        }
                    }
                }
            }
            self.values.remove(phi);
        }
        //TODO: Case of same is None → Remove its users recursively
    }

    /// Process AST nodes into CFG, returning the last block
    pub fn process_body(&mut self, body: &[ASTNode], mut curr: usize) -> usize {
        for stmt in body {
            curr = match stmt {
                ASTNode::Operation { num_type, operator, output, operands } => {
                    let stmt = Statement {
                        //TODO: improve, avoid cloning everything
                        num_type: num_type.clone(),
                        output: output.clone(),
                        value: Value { operator: operator.clone().map(OperatorOrPhi::Operator), operands: operands.clone() }
                    };
                    self.cfg.add_instruction(curr, stmt);
                    curr
                }
                ASTNode::Loop { body: loop_body } => self.handle_loop(loop_body, curr),
                ASTNode::IfThenElse { condition, if_case, else_case } => {
                    self.handle_if(condition, if_case, else_case, curr)
                }
                ASTNode::Break => {
                    let out = *self.exit_loop_context.last().expect("break outside loop");
                    self.cfg.add_uncond_link(curr, out);
                    break;
                }
                ASTNode::Continue => {
                    let entry = *self.entry_loop_context.last().expect("continue outside loop");
                    self.cfg.add_uncond_link(curr, entry);
                    break;
                }
            };
        }

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
        let entry = if self.cfg.check_empty_block(curr) { curr }
                           else { self.create_and_link(curr) };
        self.entry_loop_context.push(entry);

        // Add new block for the instructions after the loop
        let after = self.create_block();
        self.exit_loop_context.push(after);

        // The block that will be the last one in the loop
        let last = self.process_body(loop_body, entry);
        self.cfg.add_uncond_link(last, entry);

        // Popo the context
        self.entry_loop_context.pop();
        self.exit_loop_context.pop();

        // Continue after the loop
        after
    }

    fn handle_if(
        &mut self,
        condition: &Expression,
        if_case: &[ASTNode],
        else_case: &Option<Vec<ASTNode>>,
        curr: usize,
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

        // Process then case
        let end_then = self.process_body(if_case, then_b);
        self.cfg.add_uncond_link(end_then, join);

        if let Some(else_stmts) = else_case {
            let end_else = self.process_body(else_stmts, else_b);
            self.cfg.add_uncond_link(end_else, join);
        }

        join
    }

    pub fn remove_unreachable_blocks(&mut self) {
        todo!();
    }
}

#[cfg(test)]
mod tests {
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
                operator: Some(Operator::Add),
                output: Some("a".to_string()),
                operands: vec![
                Expression::Atomic(Atomic::Variable("y".to_string())),
                Expression::Atomic(Atomic::Variable("z".to_string())),
                ],
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
                        // ASTNode::Continue,
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
