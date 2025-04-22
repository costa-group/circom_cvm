use std::collections::HashSet;

pub mod ast;
pub mod types;
//TODO: This should not be public, but i want to test first
pub mod type_checking;
mod tests;

use ast::{ASTNode, Function, Template, AST};

use crate::types::*;

#[derive(Debug, Clone)]
pub struct Statement {
    num_type: Option<NumericType>,
    operator: Option<Operator>,
    output: Option<String>,
    operands: Vec<Expression>,
}

#[derive(Debug, Clone)]
pub enum Successor {
    Unconditional {
        to: usize,
    },
    Conditional {
        condition: Expression,
        to_then: usize,
        to_else: Option<usize>,
    },
}

pub struct BasicBlock {
    id: usize,
    statements: Vec<Statement>,
    predecessors: HashSet<usize>,
    successors: Option<Successor>,
}

impl BasicBlock {
    pub fn new(id: usize) -> Self {
        Self {
            id,
            statements: Vec::new(),
            predecessors: HashSet::new(),
            successors: None,
        }
    }

    pub fn add_instruction(&mut self, stmt: Statement) {
        self.statements.push(stmt);
    }

    pub fn add_predecessor(&mut self, pred: usize) {
        self.predecessors.insert(pred);
    }

    pub fn add_succesor(&mut self, suc: Successor) {
        self.successors = Some(suc);
    }
}

#[derive(Default)]
pub struct CFG {
    entry: usize,
    blocks: Vec<BasicBlock>,
}

//Returns the exit block
fn process_body(blocks: &mut Vec<BasicBlock>, body: &Vec<ASTNode>, curr: usize) -> usize {
    for stat in body {
        match stat {
            ASTNode::Operation { num_type, operator, output, operands } => {
                let stmt = Statement {
                    //TODO: improve, avoid cloning everything
                    num_type: num_type.clone(),
                    operator: operator.clone(),
                    output: output.clone(),
                    operands: operands.clone(),
                };
                blocks[curr].add_instruction(stmt);
            }
            ASTNode::Loop { body } => {

            }
            ASTNode::IfThenElse { condition, if_case, else_case } => {
                //Add if block
                let to_if = body.len();
                let if_block = BasicBlock::new(to_if);
                blocks.push(if_block);

                //Add else block (optional)
                let mut to_else = None;
                if else_case.is_some() {
                    to_else = Some(body.len());
                    let else_block = BasicBlock::new(body.len() + 1);
                    blocks.push(else_block);
                }

                let suc = Successor::Conditional { condition: condition.clone(), to_then: body.len(), to_else };
                blocks[curr].add_succesor(suc);

                //If body
                let exit_if = process_body(blocks, body, to_if);

                //Else body
                let mut exit_else: Option<usize> = None;
                if let Some(to_else) = to_else {
                    exit_else = Some(process_body(blocks, body, to_else));
                }

                //Convergence block
                let conv_id = body.len();
                let conv_block = BasicBlock::new(conv_id);
                blocks.push(conv_block);

                //Link with if case
                blocks[exit_if].add_succesor(Successor::Unconditional { to: conv_id });
                blocks[conv_id].add_predecessor(exit_if);

                //Link with else case
                if let Some(exit_else) = exit_else {
                    blocks[exit_else].add_succesor(Successor::Unconditional { to: conv_id });
                    blocks[conv_id].add_predecessor(exit_else);
                }
            }
            ASTNode::Break => {

            }
            ASTNode::Continue => {

            }
        }
    }

    //Todo: return the correct exit block
    0
}

impl CFG {
    pub fn new_from_fun(f: Function) -> Self {
        let mut entry = 0;
        let mut blocks = vec![BasicBlock::new(entry)];
        //TODO: add declarations of parameters

        process_body(&mut blocks, &f.body, entry);

        CFG { entry, blocks }
    }

    pub fn new_from_template(t: Template) -> Self {
        let mut entry = 0;
        let mut blocks = vec![BasicBlock::new(entry)];
        //TODO: add declarations of inputs

        process_body(&mut blocks, &t.body, entry);

        CFG { entry, blocks }
    }
}

pub struct CFGList {
    entry: usize,
    cfgs: Vec<CFG>,
}

impl CFGList {
    pub fn new(ast: AST) -> Self {
        let mut cfgs = Vec::new();
        let mut entry = 0;

        //CFGs for functions
        for f in ast.functions {
            cfgs.push(CFG::new_from_fun(f));
        }

        //CFGs for templates
        for t in ast.templates {
            if t.name == ast.main_template {
                entry = cfgs.len();
            }
            cfgs.push(CFG::new_from_template(t));
        }

        Self { entry, cfgs }
    }
}
