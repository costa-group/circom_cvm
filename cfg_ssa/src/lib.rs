use std::collections::HashSet;

pub mod ast;
pub mod types;
//TODO: This should not be public, but i want to test first
pub mod type_checking;
mod cfg_construction;
mod tests;

use ast::{ASTNode, Function, Template, AST};
use cfg_construction::CFG_Constructor;

use crate::types::*;


#[derive(Debug, Clone)]
enum OperatorOrPhi {
    Operator(Operator),
    Phi,
}

#[derive(Debug, Clone)]
pub struct Statement {
    //TODO: Should we store thiw now?
    // num_type: Option<NumericType>,
    operator: Option<OperatorOrPhi>,
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

    fn add_predecessor(&mut self, pred: usize) {
        self.predecessors.insert(pred);
    }

    fn add_succesor(&mut self, suc: Successor) {
        self.successors = Some(suc);
    }
}

#[derive(Default)]
pub struct CFG {
    entry: usize,
    blocks: Vec<BasicBlock>,
}

impl CFG {
    pub fn new_from_fun(f: Function) -> Self {
        let mut entry = 0;
        let mut blocks = vec![BasicBlock::new(entry)];
        //TODO: add declarations of parameters

        let mut constructor = CFG_Constructor::new();
        constructor.process_body(&mut blocks, &f.body, entry);

        CFG { entry, blocks }
    }

    pub fn new_from_template(t: Template) -> Self {
        let mut entry = 0;
        let mut blocks = vec![BasicBlock::new(entry)];
        //TODO: add declarations of inputs

        let mut constructor = CFG_Constructor::new();
        constructor.process_body(&mut blocks, &t.body, entry);

        CFG { entry, blocks }
    }

    pub fn add_instruction(&mut self, block: usize, stmt: Statement) {
        self.blocks[block].add_instruction(stmt);
    }

    pub fn create_new_block(&mut self) -> usize {
        let id = self.blocks.len();
        let new_block = BasicBlock::new(id);
        self.blocks.push(new_block);

        id
    }

    pub fn add_uncond_link(&mut self, pred: usize, suc: usize) {
        self.blocks[pred].add_succesor(Successor::Unconditional { to: suc });
        self.blocks[suc].add_predecessor(pred);
    }

    pub fn add_cond_link(&mut self, pred: usize, condition: Expression, to_then: usize, to_else: Option<usize>) {
        self.blocks[pred].add_succesor(Successor::Conditional { condition, to_then, to_else });
        self.blocks[to_then].add_predecessor(pred);
        if let Some(to_else) = to_else {
            self.blocks[to_else].add_predecessor(pred);
        }
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
