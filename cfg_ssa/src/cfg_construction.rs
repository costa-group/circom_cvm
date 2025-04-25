use crate::{ast::ASTNode, BasicBlock, OperatorOrPhi, Statement, Successor, CFG};

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
                    //Add new block for the loop body
                    let entry_loop = cfg.create_new_block();
                    cfg.add_uncond_link(curr, entry_loop);
                    self.entry_loop_context.push(entry_loop);

                    //Add new block for the instructions after the loop
                    let out_loop = cfg.create_new_block();
                    self.exit_loop_context.push(out_loop);

                    let exit_loop = self.process_body(cfg, body_loop, entry_loop);
                    cfg.add_uncond_link(entry_loop, exit_loop);

                    self.entry_loop_context.pop();
                    self.exit_loop_context.pop();

                    curr = out_loop;
                }
                ASTNode::IfThenElse { condition, if_case, else_case } => {
                    //If body
                    let to_then = cfg.create_new_block();

                    //Add else block (optional)
                    let mut to_else = None;
                    if let Some(else_case) = else_case {
                        to_else = Some(cfg.create_new_block());
                    }

                    cfg.add_cond_link(curr, condition.clone(), to_then, to_else);

                    //Process then case
                    let exit_if = self.process_body(cfg, if_case, to_then);

                    //Process else case
                    let mut exit_else = None;
                    if let (Some(else_case), Some(else_id)) = (else_case, to_else) {
                        exit_else = Some(self.process_body(cfg, else_case, else_id));
                    }

                    //Convergence block
                    let conv_id = cfg.create_new_block();

                    //Link with if case
                    cfg.add_uncond_link(exit_if, conv_id);

                    //Link with else case
                    if let Some(exit_else) = exit_else {
                        cfg.add_uncond_link(exit_else, conv_id);
                    }
                }
                ASTNode::Break => {
                    if let Some(out_loop) = self.exit_loop_context.last() {
                        cfg.add_uncond_link(curr, *out_loop);
                    }
                    else {
                        //TODO: Improve error
                        panic!("There must not be a break outside a loop");
                    }
                }
                ASTNode::Continue => {
                    if let Some(entry_loop) = self.entry_loop_context.last() {
                        cfg.add_uncond_link(curr, *entry_loop);
                    }
                    else {
                        //TODO: Improve error
                        panic!("There must not be a continue outside a loop");
                    }
                }
            }
        }

        0
    }
}

