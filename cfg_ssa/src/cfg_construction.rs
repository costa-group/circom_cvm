use crate::{ast::ASTNode, OperatorOrPhi, Statement, CFG};

type Stack<T> = Vec<T>;

pub struct CfgConstructor {
    //This stacks save the last loop entry and exit blocks (needed for the break and continue)
    entry_loop_context: Stack<usize>,
    exit_loop_context: Stack<usize>,
}


impl  CfgConstructor {
    pub fn new() -> Self {
        Self { entry_loop_context: Stack::new(), exit_loop_context: Stack::new(), }
    }

    //Returns the exit block
    //TODO: Right now we return a cfg with unreachable blocks, we should remove them or not include them
    pub fn process_body(&mut self, cfg: &mut CFG, body: &Vec<ASTNode>, mut curr: usize) -> usize {
        for stat in body {
            match stat {
                ASTNode::Operation { num_type: _, operator, output, operands } => {
                    let stmt = Statement {
                        //TODO: improve, avoid cloning everything
                        operator: operator.clone().map(OperatorOrPhi::Operator),
                        output: output.clone(),
                        operands: operands.clone(),
                    };
                    cfg.add_instruction(curr, stmt);
                }
                ASTNode::Loop { body: body_loop } => {
                    //Add new block for the loop body (if the current is not empty)
                    let entry_block_loop;
                    if !cfg.check_empty_block(curr) {
                        entry_block_loop = cfg.create_new_block();
                        cfg.add_uncond_link(curr, entry_block_loop);
                    }
                    else {
                        entry_block_loop = curr;
                    }
                    self.entry_loop_context.push(entry_block_loop);

                    //Add new block for the instructions after the loop
                    let after_loop_block = cfg.create_new_block();
                    self.exit_loop_context.push(after_loop_block);

                    //The block that will be the last one in the loop
                    let last_block_loop = self.process_body(cfg, body_loop, entry_block_loop);
                    //Link the last block with the entry block for looping
                    cfg.add_uncond_link(last_block_loop, entry_block_loop);

                    //Pop the context
                    self.entry_loop_context.pop();
                    self.exit_loop_context.pop();

                    //Continue after the loop
                    curr = after_loop_block;
                }
                ASTNode::IfThenElse { condition, if_case, else_case } => {
                    //If body
                    let to_then = cfg.create_new_block();

                    //Convergence block
                    //TODO: Create this always or only when there are more instructions?
                    let conv_id = cfg.create_new_block();

                    //Add else block (optional)
                    let to_else;
                    if else_case.is_some() {
                        to_else = cfg.create_new_block();
                    }
                    else {
                        to_else = conv_id;
                    }

                    //Link current block with the then and else blocks
                    cfg.add_cond_link(curr, condition.clone(), to_then, to_else);

                    //Process then case
                    let exit_if = self.process_body(cfg, if_case, to_then);
                    cfg.add_uncond_link(exit_if, conv_id);

                    //Process else case
                    if let Some(else_case) = else_case {
                        let exit_else = self.process_body(cfg, else_case, to_else);
                        cfg.add_uncond_link(exit_else, conv_id);
                    }

                    curr = conv_id;
                }
                ASTNode::Break => {
                    if let Some(out_loop) = self.exit_loop_context.last() {
                        cfg.add_uncond_link(curr, *out_loop);
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
                        cfg.add_uncond_link(curr, *entry_loop);
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

    pub fn remove_unreachable_blocks(&mut self, cfg: &mut CFG) {
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
