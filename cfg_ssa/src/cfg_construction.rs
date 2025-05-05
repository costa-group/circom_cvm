use std::collections::HashMap;

use crate::{ast::ASTNode, OperatorOrPhi, Statement, Value, CFG};

type Stack<T> = Vec<T>;

pub struct CfgConstructor<'a> {
    cfg: &'a mut CFG,
    //This stacks save the last loop entry and exit blocks (needed for the break and continue)
    entry_loop_context: Stack<usize>,
    exit_loop_context: Stack<usize>,
    // Save for each block the definition of the variables it contains (the new name)
    current_definition: Vec<HashMap<String, String>>,
    // Save for each static single assign variable its actual value
    definition_value: HashMap<String, Value>,
    variable_index: usize,
    sealed_blocks: Vec<bool>,
    incomplete_phis: Vec<HashMap<String, Value>>,
}


impl<'a> CfgConstructor<'a> {
    pub fn new(cfg: &'a mut CFG) -> Self { Self { cfg, entry_loop_context: Stack::new(),
    exit_loop_context: Stack::new(), current_definition: Vec::new(), definition_value:
        HashMap::new(), sealed_blocks: Vec::new(), incomplete_phis: Vec::new(), variable_index: 0 }
    }

    //Create a new block for the cfg, but, at the same time, increase the size of the block
    //variable definition
    fn create_block(&mut self) -> usize {
        self.current_definition.push(HashMap::new());
        self.incomplete_phis.push(HashMap::new());
        self.sealed_blocks.push(false);
        self.cfg.create_new_block()
    }

    fn create_new_var(&mut self) -> String {
        let new_var = format!("v{}", self.variable_index);
        self.variable_index += 1;
        new_var
    }

    fn seal_block(&mut self, block: usize) {
        //TODO: Check variables in incomplete Phis of the block
        self.sealed_blocks[block] = true;
    }

    fn write_variable(&mut self, variable: String, block: usize, value: Value) {
        // self.current_definition[block].insert(variable.clone(), value.clone());
        let new_var = self.create_new_var();
        self.definition_value.insert(new_var.clone(), value.clone());
        self.current_definition[block].insert(variable.clone(), new_var);
    }

    fn read_variable(&mut self, variable: &str, block: usize) -> String {
        if self.current_definition[block].contains_key(variable) {
            //TODO: Don't clone
            return self.current_definition[block][variable].clone();
        }
        self.read_variable_recursive(variable, block)
    }

    fn read_variable_recursive(&mut self, variable: &str, block: usize) -> String {
        //TODO: Remove all those clones
        let new_var;
        if !self.sealed_blocks[block] {
            // Incomplete CFG
            new_var = self.create_new_var();
            let value = Value { operator: Some(OperatorOrPhi::Phi), operands: Vec::new() };
            // self.incomplete_phis[block].insert(new_var.clone(), value);
        }
        else if self.cfg.predecessors(block).len() == 1 {
            // Optimize the common case of one predecessor: No phi needed
            let var = self.read_variable(variable, self.cfg.predecessors(block)[0]);
            let value = self.definition_value[&var].clone();
            self.write_variable(var.clone(), block, value);
            new_var = self.read_variable(&var, block);
        }
        else {
            // Break potential cycles with operandless phi
            self.write_variable(variable.to_string(), block, Value { operator: Some(OperatorOrPhi::Phi), operands: Vec::new() });
            new_var = self.read_variable(variable, block);

        }
        new_var
    }

    fn add_phi_operands(&mut self, variable: String, phi: &mut Value, block: usize) {
        for pred in self.cfg.predecessors(block) {
            phi.append_operand(self.read_variable(variable, *pred));
        }
        return try_remove_trivial_phi(phi);
    }

    fn try_remove_trivial_phi(&mut self, phi: &mut Value) {
        todo!();
        // let mut same = None;
        // for op in phi.operands {
        //     if op == same || op == phi {
        //         continue;
        //     }
        //     if same.is_some() {
        //         return phi;
        //     }
        //     some = Some(op);
        // }
        // user = phi.users.remove(phi);
        // phi.replace_by(same);
        //
        // for use in users {
        //     if use is Phi {
        //         self.try_remove_trivial_phi(use);
        //     }
        // }
        //
        // return same;
    }

    //Returns the exit block
    //TODO: Right now we return a cfg with unreachable blocks, we should remove them or not include them
    pub fn process_body(&mut self, body: &Vec<ASTNode>, mut curr: usize) -> usize {
        for stat in body {
            match stat {
                ASTNode::Operation { num_type, operator, output, operands } => {
                    let stmt = Statement {
                        //TODO: improve, avoid cloning everything
                        num_type: num_type.clone(),
                        output: output.clone(),
                        value: Value { operator: operator.clone().map(OperatorOrPhi::Operator), operands: operands.clone() }
                    };
                    self.cfg.add_instruction(curr, stmt);
                }
                ASTNode::Loop { body: body_loop } => {
                    //Add new block for the loop body (if the current is not empty)
                    let entry_block_loop;
                    if !self.cfg.check_empty_block(curr) {
                        entry_block_loop = self.create_block();
                        self.cfg.add_uncond_link(curr, entry_block_loop);
                    }
                    else {
                        entry_block_loop = curr;
                    }
                    self.entry_loop_context.push(entry_block_loop);

                    //Add new block for the instructions after the loop
                    let after_loop_block = self.create_block();
                    self.exit_loop_context.push(after_loop_block);

                    //The block that will be the last one in the loop
                    let last_block_loop = self.process_body(body_loop, entry_block_loop);
                    //Link the last block with the entry block for looping
                    self.cfg.add_uncond_link(last_block_loop, entry_block_loop);

                    //Pop the context
                    self.entry_loop_context.pop();
                    self.exit_loop_context.pop();

                    //Continue after the loop
                    curr = after_loop_block;
                }
                ASTNode::IfThenElse { condition, if_case, else_case } => {
                    //If body
                    let to_then = self.create_block();

                    //Convergence block
                    //TODO: Create this always or only when there are more instructions?
                    let conv_id = self.create_block();

                    //Add else block (optional)
                    let to_else = else_case
                        .as_ref()
                        .map_or(conv_id, |_| self.create_block());

                    //Link current block with the then and else blocks
                    self.cfg.add_cond_link(curr, condition.clone(), to_then, to_else);

    //Process then case
                    let exit_if = self.process_body(if_case, to_then);
                    self.cfg.add_uncond_link(exit_if, conv_id);

                    //Process else case
                    if let Some(else_case) = else_case {
                        let exit_else = self.process_body(else_case, to_else);
                        self.cfg.add_uncond_link(exit_else, conv_id);
                    }

                    curr = conv_id;
                }
                ASTNode::Break => {
                    if let Some(out_loop) = self.exit_loop_context.last() {
                        self.cfg.add_uncond_link(curr, *out_loop);
                        //The instructions following a break will never be executed
                        break;
                    }
                    else {
                        //TODO: Improve error
                        panic!("There must not be a break outside a loop");
                    }
                }
                ASTNode::Continue => {
                    if let Some(entry_loop) = self.entry_loop_context.last() {
                        self.cfg.add_uncond_link(curr, *entry_loop);
                        //The instructions following a continue will never be executed
                        break;
                    }
                    else {
                        //TODO: Improve error
                        panic!("There must not be a continue outside a loop");
                    }
                }
            }
        }

        curr
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
