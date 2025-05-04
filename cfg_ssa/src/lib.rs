pub mod ast;
pub mod types;
//TODO: This should not be public, but i want to test first
pub mod type_checking;
mod cfg_construction;
mod tests;

use ast::{Function, Template, AST};
use cfg_construction::CfgConstructor;

use serde::Serialize;

use crate::types::*;


#[derive(Debug, Clone, Serialize)]
enum OperatorOrPhi {
    Operator(Operator),
    Phi,
}

#[derive(Debug, Clone, Serialize)]
struct Value {
    operator: Option<OperatorOrPhi>,
    operands: Vec<Expression>,
}

#[derive(Debug, Clone, Serialize)]
pub struct Statement {
    num_type: Option<NumericType>,
    output: Option<String>,
    value: Value,
}

#[derive(Debug, Clone, Serialize)]
pub enum Successor {
    Unconditional {
        to: usize,
    },
    Conditional {
        condition: Expression,
        to_then: usize,
        to_else: usize,
    },
}

#[derive(Debug, Serialize)]
pub struct BasicBlock {
    id: usize,
    statements: Vec<Statement>,
    predecessors: Vec<usize>,
    successors: Option<Successor>,
}

impl BasicBlock {
    pub fn new(id: usize) -> Self {
        Self {
            id,
            statements: Vec::new(),
            predecessors: Vec::new(),
            successors: None,
        }
    }

    pub fn add_instruction(&mut self, stmt: Statement) {
        self.statements.push(stmt);
    }

    fn add_predecessor(&mut self, pred: usize) {
        self.predecessors.push(pred);
    }

    fn add_succesor(&mut self, suc: Successor) {
        self.successors = Some(suc);
    }
}

#[derive(Default, Debug, Serialize)]
pub struct CFG {
    entry: usize,
    blocks: Vec<BasicBlock>,
}

impl CFG {
    pub fn new(entry: usize) -> Self {
        CFG { entry, blocks: vec![BasicBlock::new(entry)] }
    }

    pub fn new_from_fun(f: Function) -> Self {
        let entry = 0;
        let mut cfg = CFG::new(entry);
        //TODO: add declarations of parameters

        let mut constructor = CfgConstructor::new(&mut cfg);
        constructor.process_body(&f.body, entry);

        cfg
    }

    pub fn new_from_template(t: Template) -> Self {
        let entry = 0;
        let mut cfg = CFG::new(entry);
        //TODO: add declarations of inputs

        let mut constructor = CfgConstructor::new(&mut cfg);
        constructor.process_body(&t.body, entry);

        cfg
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

    pub fn check_empty_block(&self, block: usize) -> bool {
        self.blocks[block].statements.is_empty()
    }

    pub fn predecessors(&self, block: usize) -> &Vec<usize> {
        &self.blocks[block].predecessors
    }

    fn check_existing_successor(&self, block: usize) -> bool {
        self.blocks[block].successors.is_some()
    }

    pub fn add_uncond_link(&mut self, pred: usize, suc: usize) {
        //TODO: check if this is correct: do not overwrite existing successors
        if !self.check_existing_successor(pred) {
            self.blocks[pred].add_succesor(Successor::Unconditional { to: suc });
            self.blocks[suc].add_predecessor(pred);
        }
    }

    pub fn add_cond_link(&mut self, pred: usize, condition: Expression, to_then: usize, to_else: usize) {
        //TODO: check if this is correct: do not overwrite existing successors
        if !self.check_existing_successor(pred) {
            self.blocks[pred].add_succesor(Successor::Conditional { condition, to_then, to_else });
            self.blocks[to_then].add_predecessor(pred);
            self.blocks[to_else].add_predecessor(pred);
        }
    }


    pub fn get_entry(&self) -> usize {
        self.entry
    }

    #[deprecated(note = "This function is only for debugging purposes!")]
    #[doc(hidden)]
    pub fn to_json(&self) -> String {
        serde_json::to_string_pretty(self).unwrap()
    }

    #[deprecated(note = "This function is only for debugging purposes!")]
    #[doc(hidden)]
    pub fn to_dot(&self) -> String {
        // helper: only escape quotes, leave \n alone
        fn esc(s: &str) -> String {
            s.replace('"', "\\\"")
        }
    
        let mut dot = String::new();
        dot.push_str("digraph G {\n");
    
        for block in &self.blocks {
            // 1) collect one line per statement (with Debug)
            let mut lines = Vec::new();
            lines.push(format!("Block {}", block.id));
            for stmt in &block.statements {
                lines.push(format!("{:?}", stmt));
            }
    
            // 2) join with "\n"
            let raw_label = lines.join("\n");
            // 3) escape only quotes
            let label = esc(&raw_label);
    
            dot.push_str(&format!(
                "  {} [label=\"{}\", shape=box];\n",
                block.id, label
            ));
    
            // edges
            if let Some(succ) = &block.successors {
                match succ {
                    Successor::Unconditional { to } => {
                        dot.push_str(&format!("  {} -> {};\n", block.id, to));
                    }
                    Successor::Conditional { condition, to_then, to_else } => {
                        let cond = esc(&format!("{:?}", condition));
                        dot.push_str(&format!(
                            "  {} -> {} [label=\"if {}\"];\n",
                            block.id, to_then, cond
                        ));
                        dot.push_str(&format!("  {} -> {} [label=\"else\"];\n",
                                            block.id, to_else));
                    }
                }
            }
        }
    
        dot.push_str("}\n");
        dot
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
