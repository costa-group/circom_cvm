pub mod ast;
pub mod types;
//TODO: This should not be public, but i want to test first
pub mod type_checking;
mod cfg_construction;
mod tests;

use std::collections::{HashMap, VecDeque};

use ast::{Function, Template, AST};
use cfg_construction::CfgConstructor;

use serde::Serialize;

use crate::types::*;

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

#[derive(Debug, Clone, Serialize)]
struct Value {
    operator: Option<Operator>,
    operands: Vec<Expression>,
}

#[derive(Clone, Serialize)]
pub struct Statement {
    num_type: Option<NumericType>,
    output: Option<String>,
    value: Value,
}

use std::fmt;
impl fmt::Debug for Statement {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut s = String::new();
        if let Some(out) = &self.output {
            s.push_str(out);
            s.push_str(" = ");
        }
        if let Some(typ) = &self.num_type {
            s.push_str(match typ {
                NumericType::Integer => "i64.",
                NumericType::FiniteField => "ff."
            })
        }
        if let Some(op) = &self.value.operator {
            s.push_str(&format!("{:?} ", op).to_lowercase());
        }
        for op in self.value.operands.iter() {
            s.push_str(&format!("{:?} ", op));
        }
        write!(f, "{}", s)
    }
}


/// The list of possibilities are the name of the ssa variable and the block where it comes from
#[derive(Debug, Clone, Serialize)]
pub struct PhiPossibility {
    variable: String,
    block: usize,
}

#[derive(Clone, Serialize)]
pub struct PhiFunction {
    output: String,
    possibilities: Vec<PhiPossibility>,
}

impl fmt::Debug for PhiFunction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut s = String::new();
        s.push_str(&self.output);
        s.push_str(" = φ ");
        for phi in self.possibilities.iter() {
            s.push_str(&format!("{} (B{}), ", phi.variable, phi.block));
        }
        write!(f, "{}", s)
    }
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

#[derive(Debug, Clone, Serialize)]
struct PositionDeclaration {
    is_phi: bool,
    line: usize,
}

#[derive(Debug, Serialize)]
pub struct BasicBlock {
    id: usize,
    phi_functions: Vec<PhiFunction>,
    statements: Vec<Statement>,
    predecessors: Vec<usize>,
    successors: Option<Successor>,
    ///Whether a variable is declared as a phi function and its position in the list of phi
    ///functions or statements accordingly
    //TODO: Maybe search directly in the vectors if they are usually small?
    declarations: HashMap<String, PositionDeclaration>,
}

impl BasicBlock {
    pub fn new(id: usize) -> Self {
        Self {
            id,
            phi_functions: Vec::new(),
            statements: Vec::new(),
            predecessors: Vec::new(),
            successors: None,
            declarations: HashMap::new(),
        }
    }

    fn add_phi_function(&mut self, phi: PhiFunction) {
        self.declarations.insert(phi.output.clone(), PositionDeclaration { is_phi: true, line: self.phi_functions.len() });
        self.phi_functions.push(phi);
    }

    fn add_instruction(&mut self, stmt: Statement) {
        if let Some(output) = &stmt.output {
            self.declarations.insert(output.clone(), PositionDeclaration { is_phi: false, line: self.statements.len() });
        }
        self.statements.push(stmt);
    }

    fn change_declaration_operands(&mut self, name: &str, target: &str, replacement: &str) {
        if let Some(PositionDeclaration { is_phi: false, line }) = self.declarations.get(name) {
            let stmt = self.statements.get_mut(*line).expect("Missing statement");
            for op in stmt.value.operands.iter_mut() {
                replace_variable_in_expression(op, target, replacement);
            }
        } else if let Some(PositionDeclaration { is_phi: true, line }) = self.declarations.get(name) {
            let phi = self.phi_functions.get_mut(*line).expect("Missing phi function");
            for possibility in phi.possibilities.iter_mut() {
                if possibility.variable == target {
                    possibility.variable = replacement.to_string();
                }
            }
        } else {
            panic!("Variable {} not found in block {}", name, self.id);
        }
    }

    fn change_statement_operands(&mut self, line: usize, target: &str, replacement: &str) {
        let stmt = self.statements.get_mut(line).expect("Missing statement");
        for op in stmt.value.operands.iter_mut() {
            replace_variable_in_expression(op, target, replacement);
        }
    }

    fn add_predecessor(&mut self, pred: usize) {
        self.predecessors.push(pred);
    }

    fn add_succesor(&mut self, suc: Successor) {
        self.successors = Some(suc);
    }

    fn change_condition(&mut self, target: &str, replacement: &str) {
        if let Some(Successor::Conditional { condition, to_then: _, to_else: _ }) = &mut self.successors {
            replace_variable_in_expression(condition, target, replacement);
        } else {
            panic!("Expected conditional successors");
        }
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

        let mut constructor = CfgConstructor::new(&mut cfg);
        constructor.process_body(&f.body, entry, None);

        cfg
    }

    pub fn new_from_template(t: Template) -> Self {
        let entry = 0;
        let mut cfg = CFG::new(entry);

        let mut constructor = CfgConstructor::new(&mut cfg);
        constructor.process_body(&t.body, entry, None);

        cfg
    }

    pub fn add_phi_function(&mut self, block: usize, phi: PhiFunction) {
        self.blocks[block].add_phi_function(phi);
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

    fn change_declaration_operands(&mut self, block: usize, name: &str, target: &str, replacement: &str) {
        self.blocks[block].change_declaration_operands(name, target, replacement);
    }

    fn change_statement_operands(&mut self, block: usize, line: usize, target: &str, replacement: &str) {
        self.blocks[block].change_statement_operands(line, target, replacement);
    }

    fn change_condition(&mut self, block: usize, target: &str, replacement: &str) {
        self.blocks[block].change_condition(target, replacement);
    }

    #[deprecated(note = "This function is only for debugging purposes!")]
    #[doc(hidden)]
    //TODO: Change error type
    //TODO: Fix
    fn check_ssa(&self) -> Result<(), String> {
        //First pass, check double declarations and add them to a map
        //declarations:
        //- Key: Variable declared
        //- Value: Block in which it is declared
        let mut declarations = HashMap::new();

        for (index, block) in self.blocks.iter().enumerate() {
            for phi in &block.phi_functions {
                if declarations.contains_key(&phi.output) {
                    //Error: variable is declared more than once
                    return Err(format!("Variable '{}' is declared more than once.", phi.output));
                } else {
                    declarations.insert(phi.output.clone(), index);
                }
            }
            for stmt in &block.statements {
                if let Some(var) = &stmt.output {
                    if declarations.contains_key(var) {
                        //Error: variable is declared more than once
                        return Err(format!("Variable '{}' is declared more than once.", var));
                    } else {
                        declarations.insert(var.clone(), index);
                    }
                }
            }
        }

        //Second pass, check uses of variables
        //Every variable must be declared before it is used
        let mut reachable = vec![vec![false; self.blocks.len()]; self.blocks.len()];
        //Blocks in which we already have every reachable block
        let mut closed_blocks = vec![false; self.blocks.len()];
        //The entry block only has itself as reachable therefore
        closed_blocks[0] = true;

        for (index, block) in self.blocks.iter().enumerate() {
            //A block is reachable by itself
            reachable[index][index] = true;
            for prec in &block.predecessors {
                //A block can reach its predecessors
                reachable[index][*prec] = true;
            }
        }

        //use_line represents whether it is used in a phi function or a statement and the line in
        //the corresponding vector of the simple block with id "use_block"
        let mut ensure_reachability = |var: &str, use_block: usize, use_line: PositionDeclaration| -> Result<(), String> {
            let declaration_block = *declarations
                .get(var)
                .ok_or_else(|| format!("Variable '{}' was not declared in the program.", var))?;

            if use_block == declaration_block {
                //TODO: Change unwrap
                let decl_line = self.blocks[declaration_block].declarations.get(var).unwrap();
                // Error cases:
                // - Declared as a phi function and used in one, but before declaration
                // - Declared in stmt and used in one, but before declaration
                // - Declared in stmt, but used in a phi
                if (decl_line.is_phi == use_line.is_phi && use_line.line < decl_line.line)
                   || (decl_line.is_phi && !use_line.is_phi) {
                    return Err(format!(
                            "Variable '{}' was declared in block {} (line {}), but used in line {}.",
                            var, declaration_block, decl_line.line, use_line.line
                    ))
                }
            }

            if reachable[use_block][declaration_block] {
                return Ok(());
            }

            if closed_blocks[use_block] {
                return Err(format!(
                        "Variable '{}' was declared in block '{}', but used in block '{}' which is not reachable.",
                        var, declaration_block, use_block
                ));
            }

            //Case in which the block is not yet closed, we go backwards into the
            //predecessors with dfs marking the visited blocks
            let mut visited = vec![false; self.blocks.len()];
            let mut queue = VecDeque::new();

            visited[use_block] = true;
            queue.push_back(use_block);

            while let Some(node) = queue.pop_front() {
                //TODO: Improve this so that not only the index block is updated (also
                //all those that we visit)
                reachable[use_block][node] = true;
                if node == declaration_block {
                    break;
                }

                for &neighbor in &self.blocks[node].predecessors {
                    if !visited[neighbor] {
                        visited[neighbor] = true;
                        queue.push_back(neighbor);
                    }
                }
            }

            if queue.is_empty() {
                closed_blocks[use_block] = true;
            }

            //Error if the index block is closed, but the position block is not
            //reachable from it
            if !reachable[use_block][declaration_block] && closed_blocks[use_block] {
                return Err(format!(
                        "Variable '{}' was declared in block '{}', but used in block '{}' and it is not reachable.",
                        var, declaration_block, use_block
                ));
            }

            Ok(())
        };

        for (block_index, block) in self.blocks.iter().enumerate() {
            //Check the uses of each phi function
            for (line, phi) in block.phi_functions.iter().enumerate() {
                for pos in &phi.possibilities {
                    let use_line = PositionDeclaration { is_phi: true, line };
                    ensure_reachability(&pos.variable, block_index, use_line)?;
                }
            }

            //Check the uses of each statement
            for (line, stmt) in block.statements.iter().enumerate() {
                let val = &stmt.value;
                for operand in &val.operands {
                    for var in get_variable_names(operand) {
                        let use_line = PositionDeclaration { is_phi: false, line };
                        ensure_reachability(&var, block_index, use_line)?;
                    }
                }
            }
        }

        Ok(())
    }

    #[deprecated(note = "This function is only for debugging purposes!")]
    #[doc(hidden)]
    pub fn to_json(&self) -> String {
        serde_json::to_string_pretty(self).unwrap()
    }

    #[deprecated(note = "This function is only for debugging purposes!")]
    #[doc(hidden)]
    pub fn to_dot(&self, id: usize) -> String {
        // helper: only escape quotes, leave \n alone
        fn esc(s: &str) -> String {
            s.replace('"', "\\\"")
        }

        let mut dot = String::new();
        dot.push_str(&format!("digraph G{} {{\n", id));

        for block in &self.blocks {
            // 1) collect one line per statement (with Debug)
            let mut lines = Vec::new();
            lines.push(format!("Block {}", block.id));
            for phi in &block.phi_functions {
                lines.push(format!("{:?}", phi));
            }

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
                                "  {} -> {} [label=\"{} != 0\"];\n",
                                block.id, to_then, cond
                        ));
                        dot.push_str(&format!("  {} -> {} [label=\"{} == 0\"];\n",
                                block.id, to_else, cond));
                    }
                }
            }
        }

        dot.push_str("}\n");
        dot
    }

    fn get_block_size(&self, block: usize) -> usize {
        self.blocks[block].statements.len()
    }
}

#[derive(Default, Debug, Serialize)]
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

    #[deprecated(note = "This function is only for debugging purposes!")]
    #[doc(hidden)]
    pub fn check_ssa(&self) -> Result<(), String> {
        for (i, cfg) in self.cfgs.iter().enumerate() {
            cfg.check_ssa().map_err(|e| format!("cfg[{}] failed SSA check: {}", i, e))?;
        }
        Ok(())
    }

    #[deprecated(note = "This function is only for debugging purposes!")]
    #[doc(hidden)]
    pub fn to_json(&self) -> String {
        serde_json::to_string_pretty(self).unwrap()
    }

    #[deprecated(note = "This function is only for debugging purposes!")]
    #[doc(hidden)]
    pub fn to_dot(&self) -> Vec<String> {
        self.cfgs.iter().enumerate().map(|(id, cfg)| cfg.to_dot(id)).collect()
    }
}
