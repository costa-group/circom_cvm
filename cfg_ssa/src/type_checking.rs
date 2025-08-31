use crate::ast::*;
use crate::types::*;

use std::collections::BTreeMap;

#[derive(Debug, Clone)]
pub struct TypeChecker {
    // Crate public to allow testing
    pub(crate) variables_enviroment: BTreeMap<String, Type>,
    pub(crate) functions_enviroment: BTreeMap<String, Type>,
}

impl TypeChecker {
    pub fn new() -> Self {
        Self {
            variables_enviroment: BTreeMap::new(),
            functions_enviroment: BTreeMap::new()
        }
    }

    pub fn check(&mut self, ast: &AST) -> Result<(), String> {
        //Add functions to the enviroment
        for function in &ast.functions {
            self.functions_enviroment
                .insert(function.name.clone(), Type::Function(function.output.clone(), function.inputs.clone()));
        }

        // Check all functions
        for function in &ast.functions {
            self.check_function(function)?;
        }

        // Check all templates
        for template in &ast.templates {
            self.check_template(template)?;
        }

        Ok(())
    }

    fn check_template(&mut self, template: &Template) -> Result<(), String> {
        // Check the body of the template
        for node in &template.body {
            self.check_node(node)?;
        }

        // Clear the environment after checking the template
        self.variables_enviroment.clear();

        Ok(())
    }

    fn check_function(&mut self, function: &Function) -> Result<(), String> {
        for node in &function.body {
            self.check_node(node)?;
        }

        self.variables_enviroment.clear();

        Ok(())
    }

    // Crate public to allow testing
    pub(crate) fn check_node(&mut self, node: &ASTNode) -> Result<(), String> {
        match node {
            ASTNode::Operation { num_type, operator, output, operands } => {
                self.check_operation(num_type, operator, output, operands)
            },
            ASTNode::IfThenElse { num_type, condition, if_case, else_case } => {
                if self.type_expression(condition)? != Type::Variable(num_type.clone()) {
                    return Err(format!(
                        "Condition must be of type {:?}, but found {:?}.",
                        num_type,
                        self.type_expression(condition)?
                    ));
                }

                for inner_node in if_case {
                    self.check_node(inner_node)?;
                }

                // Check the else-case if it exists
                // Create a new environment for the else-case
                if let Some(else_case) = else_case {
                    for inner_node in else_case {
                        self.check_node(inner_node)?;
                    }
                }

                Ok(())
            }
            ASTNode::Loop { body } => {
                for inner_node in body {
                    self.check_node(inner_node)?;
                }
                Ok(())
            }
            ASTNode::Break | ASTNode::Continue => {
                Ok(())
            }
        }
    }

    fn check_operation(
        &mut self,
        num_type: &Option<NumericType>,
        operator: &Option<Operator>,
        output: &Option<String>,
        operands: &[Expression],
    ) -> Result<(), String> {
        let check_len = |expected: usize| -> Result<(), String> {
            if operands.len() != expected {
                Err(format!(
                        "Operator {:?} requires exactly {} operands, but {} were provided.",
                        operator, expected, operands.len()
                ))
            } else {
                Ok(())
            }
        };

        let check_operand = |idx: usize, expected: NumericType| -> Result<(), String> {
            let op = operands.get(idx).ok_or_else(|| "Missing operand".to_string())?;
            if self.type_expression(op)? != Type::Variable(expected.clone()) {
                Err(format!(
                        "Operand at position {} ({:?}) does not match the required type {:?} in operator {:?}.",
                        idx, op, expected, operator
                ))
            } else {
                Ok(())
            }
        };

        let mut var_type;
        match num_type {
            Some(NumericType::FiniteField) => {
                var_type = Type::Variable(NumericType::FiniteField);

                match operator {
                    Some(Operator::Add) | Some(Operator::Sub) | Some(Operator::Mul)
                        | Some(Operator::Div) | Some(Operator::IDiv) | Some(Operator::Rem) | Some(Operator::Pow)
                        | Some(Operator::Greater) | Some(Operator::GreaterEqual)
                        | Some(Operator::Less) | Some(Operator::LessEqual)
                        | Some(Operator::Equal) | Some(Operator::NotEqual)
                        | Some(Operator::And) | Some(Operator::Or) | Some(Operator::BitAnd)
                        | Some(Operator::BitOr) | Some(Operator::BitXor)
                        | Some(Operator::ShiftLeft) | Some(Operator::ShiftRight)
                    => {
                        check_len(2)?;
                        check_operand(0, NumericType::FiniteField)?;
                        check_operand(1, NumericType::FiniteField)?;
                    }
                    Some(Operator::EqualZero) | Some(Operator::BitNot) | Some(Operator::Return) => {
                        check_len(1)?;
                        check_operand(0, NumericType::FiniteField)?;
                    }
                    Some(Operator::Load) => {
                        check_len(1)?;
                        check_operand(0, NumericType::Integer)?;
                    }
                    Some(Operator::Store) => {
                        check_len(2)?;
                        check_operand(0, NumericType::Integer)?;
                        check_operand(1, NumericType::FiniteField)?;
                    }
                    Some(Operator::MStore) | Some(Operator::MStoreFromSignal) | Some(Operator::MStoreFromCmpSignal)
                        | Some(Operator::MReturn)
                    => {
                        check_len(3)?;
                        check_operand(0, NumericType::Integer)?;
                        check_operand(1, NumericType::Integer)?;
                        check_operand(2, NumericType::Integer)?;
                    }
                    Some(Operator::Call) | Some(Operator::MCall) => {
                        self.check_call(operator, operands, Some(NumericType::FiniteField))?;
                    }
                    Some(Operator::Extend) => {
                        check_len(1)?;
                        check_operand(0, NumericType::Integer)?;
                    }
                    Some(Operator::Wrap) => {
                        return Err("Wrap operator must have type Finite Field".to_string());
                    }
                    Some(Operator::GetSignal) | Some(Operator::GetCmpSignal) | Some(Operator::SetSignal)
                        | Some(Operator::SetCmpIn) | Some(Operator::SetCmpInCnt) | Some(Operator::SetCmpInRun)
                        | Some(Operator::SetCmpInCntCheck) | Some(Operator::Error) | Some(Operator::MSetSignal)
                        | Some(Operator::MSetSignalFromMemory) | Some(Operator::MSetCmpIn) | Some(Operator::MSetCmpInCnt)
                        | Some(Operator::MSetCmpInRun) | Some(Operator::MSetCmpInCntCheck) | Some(Operator::MSetCmpInFromCmp)
                        | Some(Operator::MSetCmpInFromCmpCnt) | Some(Operator::MSetCmpInFromCmpRun) | Some(Operator::MSetCmpInFromCmpCntCheck)
                        | Some(Operator::MSetCmpInFromMemory) | Some(Operator::MSetCmpInFromMemoryCnt)| Some(Operator::MSetCmpInFromMemoryRun)
                        | Some(Operator::MSetCmpInFromMemoryCntCheck)
                        | Some(Operator::GetTemplateId) | Some(Operator::GetTemplateSignalPosition)
                        | Some(Operator::GetTemplateSignalSize) | Some(Operator::GetTemplateSignalDim) | Some(Operator::GetTemplateSignalType)
                        | Some(Operator::GetBusSignalPos) | Some(Operator::GetBusSignalSize)
                        | Some(Operator::GetBusSignalDim) | Some(Operator::GetBusSignalType)
                    => {
                        return Err(format!(
                            "Operator {:?} must not have a type", operator
                        ));
                    }
                    None => {
                        return Err(format!(
                            "No operator given for type Finite Field"
                        ));
                    }
                }
            }
            Some(NumericType::Integer) => {
                var_type = Type::Variable(NumericType::Integer);

                match operator {
                    Some(Operator::Add) | Some(Operator::Sub) | Some(Operator::Mul)
                        | Some(Operator::Div) | Some(Operator::IDiv) | Some(Operator::Rem) | Some(Operator::Pow)
                        | Some(Operator::Greater) | Some(Operator::GreaterEqual)
                        | Some(Operator::Less) | Some(Operator::LessEqual)
                        | Some(Operator::Equal) | Some(Operator::NotEqual)
                        | Some(Operator::And) | Some(Operator::Or) | Some(Operator::BitAnd)
                        | Some(Operator::BitOr) | Some(Operator::BitXor) | Some(Operator::Store)=> {
                            check_len(2)?;
                            check_operand(0, NumericType::Integer)?;
                            check_operand(1, NumericType::Integer)?;
                        }
                    Some(Operator::EqualZero) | Some(Operator::ShiftLeft) | Some(Operator::ShiftRight)
                        | Some(Operator::BitNot) | Some(Operator::Return) => {
                            check_len(1)?;
                            check_operand(0, NumericType::Integer)?;
                        }
                    Some(Operator::Load) => {
                        check_len(1)?;
                        check_operand(0, NumericType::Integer)?;
                    }
                    Some(Operator::Call) | Some(Operator::MCall) => {
                        self.check_call(operator, operands, Some(NumericType::Integer))?;
                    }
                    Some(Operator::Wrap) => {
                        check_len(1)?;
                        check_operand(0, NumericType::FiniteField)?;
                    }
                    Some(Operator::MReturn) => {
                        check_len(3)?;
                        check_operand(0, NumericType::Integer)?;
                        check_operand(1, NumericType::Integer)?;
                        check_operand(2, NumericType::Integer)?;
                    }
                    Some(Operator::Extend) | Some(Operator::MStore) | Some(Operator::MStoreFromSignal)
                    | Some(Operator::MStoreFromCmpSignal) => {
                        return Err(format!(
                            "Operator {:?} must have type Finite Field", operator
                        ));
                    }
                    Some(Operator::GetSignal) | Some(Operator::GetCmpSignal) | Some(Operator::SetSignal)
                        | Some(Operator::SetCmpIn) | Some(Operator::SetCmpInCnt) | Some(Operator::SetCmpInRun)
                        | Some(Operator::SetCmpInCntCheck) | Some(Operator::Error) | Some(Operator::MSetSignal)
                        | Some(Operator::MSetSignalFromMemory) | Some(Operator::MSetCmpIn) | Some(Operator::MSetCmpInCnt)
                        | Some(Operator::MSetCmpInRun) | Some(Operator::MSetCmpInCntCheck) | Some(Operator::MSetCmpInFromCmp)
                        | Some(Operator::MSetCmpInFromCmpCnt) | Some(Operator::MSetCmpInFromCmpRun) | Some(Operator::MSetCmpInFromCmpCntCheck)
                        | Some(Operator::MSetCmpInFromMemory) | Some(Operator::MSetCmpInFromMemoryCnt)| Some(Operator::MSetCmpInFromMemoryRun)
                        | Some(Operator::MSetCmpInFromMemoryCntCheck)
                        | Some(Operator::GetTemplateId) | Some(Operator::GetTemplateSignalPosition)
                        | Some(Operator::GetTemplateSignalSize) | Some(Operator::GetTemplateSignalDim) | Some(Operator::GetTemplateSignalType)
                        | Some(Operator::GetBusSignalPos) | Some(Operator::GetBusSignalSize)
                        | Some(Operator::GetBusSignalDim) | Some(Operator::GetBusSignalType)
                    => {
                        return Err(format!(
                            "Operator {:?} must not have a type", operator
                        ));
                    }
                    None => {
                        return Err(format!(
                            "No operator given for type Integer"
                        ));
                    }
                }
            }
            None => {
                var_type = Type::Variable(NumericType::FiniteField);
                
                match operator {
                    Some(Operator::GetSignal) | Some(Operator::Error) => {
                        check_len(1)?;
                        check_operand(0, NumericType::Integer)?;
                    }
                    Some(Operator::GetCmpSignal) => {
                        check_len(2)?;
                        check_operand(0, NumericType::Integer)?;
                        check_operand(1, NumericType::Integer)?;
                    }
                    Some(Operator::SetSignal) => {
                        check_len(2)?;
                        check_operand(0, NumericType::Integer)?;
                        check_operand(1, NumericType::FiniteField)?;
                    }
                    Some(Operator::MSetSignal) | Some(Operator::MSetSignalFromMemory) => {
                        check_len(3)?;
                        check_operand(0, NumericType::Integer)?;
                        check_operand(1, NumericType::Integer)?;
                        check_operand(2, NumericType::Integer)?;
                    }
                    Some(Operator::SetCmpIn) | Some(Operator::SetCmpInCnt) | Some(Operator::SetCmpInRun) | Some(Operator::SetCmpInCntCheck) => {
                        check_len(3)?;
                        check_operand(0, NumericType::Integer)?;
                        check_operand(1, NumericType::Integer)?;
                        check_operand(2, NumericType::FiniteField)?;
                    }
                    Some(Operator::MSetCmpIn) | Some(Operator::MSetCmpInCnt) | Some(Operator::MSetCmpInRun) | Some(Operator::MSetCmpInCntCheck)
                    | Some(Operator::MSetCmpInFromMemory) | Some(Operator::MSetCmpInFromMemoryCnt) | Some(Operator::MSetCmpInFromMemoryRun) | Some(Operator::MSetCmpInFromMemoryCntCheck) 
                    => {
                        check_len(4)?;
                        check_operand(0, NumericType::Integer)?;
                        check_operand(1, NumericType::Integer)?;
                        check_operand(2, NumericType::Integer)?;
                        check_operand(3, NumericType::Integer)?;
                    }
                    Some(Operator::MSetCmpInFromCmp) | Some(Operator::MSetCmpInFromCmpCnt) | Some(Operator::MSetCmpInFromCmpRun) | Some(Operator::MSetCmpInFromCmpCntCheck) => {
                        check_len(5)?;
                        check_operand(0, NumericType::Integer)?;
                        check_operand(1, NumericType::Integer)?;
                        check_operand(2, NumericType::Integer)?;
                        check_operand(3, NumericType::Integer)?;
                        check_operand(4, NumericType::Integer)?;
                    }
                    Some(Operator::GetTemplateId) => {
                        check_len(1)?;
                        check_operand(0, NumericType::Integer)?;
                        var_type = Type::Variable(NumericType::Integer);
                    }
                    Some(Operator::GetTemplateSignalSize) | Some(Operator::GetTemplateSignalPosition)
                    | Some(Operator::GetTemplateSignalDim) | Some(Operator::GetTemplateSignalType)
                    | Some(Operator::GetBusSignalPos) | Some(Operator::GetBusSignalSize)
                    | Some(Operator::GetBusSignalDim) | Some(Operator::GetBusSignalType)
                     => {
                        check_len(2)?;
                        check_operand(0, NumericType::Integer)?;
                        check_operand(1, NumericType::Integer)?;
                        var_type = Type::Variable(NumericType::Integer);
                    }
                    Some(Operator::Add) | Some(Operator::Sub) | Some(Operator::Mul)
                    | Some(Operator::Div) | Some(Operator::Rem) | Some(Operator::IDiv)
                    | Some(Operator::Pow) | Some(Operator::Greater) | Some(Operator::GreaterEqual)
                    | Some(Operator::Less) | Some(Operator::LessEqual) | Some(Operator::Equal)
                    | Some(Operator::NotEqual) | Some(Operator::EqualZero) | Some(Operator::And)
                    | Some(Operator::Or) | Some(Operator::ShiftRight) | Some(Operator::ShiftLeft)
                    | Some(Operator::BitAnd) | Some(Operator::BitOr) | Some(Operator::BitXor)
                    | Some(Operator::BitNot) | Some(Operator::Extend) | Some(Operator::Wrap)
                    | Some(Operator::Load) | Some(Operator::Store) | Some(Operator::MStore)
                    | Some(Operator::MStoreFromSignal) | Some(Operator::MStoreFromCmpSignal)
                    | Some(Operator::Call) | Some(Operator::MCall)
                    | Some(Operator::Return) | Some(Operator::MReturn)
                    => {
                    return Err(format!(
                                "Operator {:?} requires a numeric type, but none was provided.",
                                operator
                        ));
                    },
                    None => {
                        if operands.len() != 1 {
                            return Err("Assignment of a variable or a constant to another variable must only have one operand (the value).".to_string());
                        }
                        //Case x = constant
                        //FF
                        if let Some(Expression::Atomic(Atomic::Constant(ConstantType::FF(_)))) = operands.first() {
                            var_type = Type::Variable(NumericType::FiniteField);
                        }
                        //I64
                        else if let Some(Expression::Atomic(Atomic::Constant(ConstantType::I64(_)))) = operands.first() {
                            var_type = Type::Variable(NumericType::Integer);
                        }
                        else {
                            //If the operand is not a constant, it must be a variable
                            //Case x = y
                            if let Some(Expression::Atomic(Atomic::Variable(variable))) = operands.first() {

                                let input_type = self.variables_enviroment.get(variable);

                                var_type = input_type
                                    .ok_or(format!("Variable {} not found in environment", variable))?
                                    .clone();
                            } else {
                                    return Err(format!(
                                            "Operand {:?} must be a variable for variable assignment.",
                                            operands.first()
                                    ));
                            }
                        }
                    }
                }
            }
        }

        if let Some(o) = output {
            self.variables_enviroment.insert(o.clone(), var_type);
        }
        Ok(())
    }

    fn check_call(&mut self, operator: &Option<Operator>, operands: &[Expression], expected_call_type: Option<NumericType>) -> Result<(), String> {
        if operands.is_empty() {
            return Err(format!(
                    "Operator {:?} requires at least a function to call, but none was provided.",
                    operator,
            ));
        }

        // Check if the first operand is a function name
        if let Some(Expression::Atomic(Atomic::Function(fun_name))) = operands.first() {
            //Find the function in the environment
            let function_type = self.functions_enviroment
                .get(fun_name)
                .ok_or(format!("Function {} not found in environment", fun_name))?;
            if let Type::Function(output_type, input_types) = function_type {
                // Check if the number of operands matches the number of input types
                // The first operand is the function name, so we skip it
                if input_types.len() != operands.len() - 1 {
                    return Err(format!(
                            "Function {} requires {} inputs, but {} were provided.",
                            fun_name,
                            input_types.len(),
                            operands.len() - 1
                    ));
                }

                // Check if the types of the operands match the types of the inputs
                // The first operand is the function name, so we skip it
                for (operand, param) in operands.iter().skip(1).zip(input_types) {
                    //TODO: Properly check the parameters
                    if self.type_expression(operand)? != Type::Variable(param.0.clone()) {
                        return Err(format!(
                                "Operand {:?} does not match the required type {:?}.",
                                operand,
                                param
                        ));
                    }
                }

                // Check if the output type matches the expected type.
                // Only do this if the operator is a normal call, not a Memory Call
                if let Some(Operator::Call) = operator {
                    if let Some(output_type) = output_type {
                        if let Some(expected) = expected_call_type {
                            //Output type in definition and in call, but not equal
                            if *output_type != expected {
                                return Err(format!(
                                        "Function {} must output a {:?}, but call expects {:?}.",
                                        fun_name, output_type, expected
                                ));
                            }
                        } else {
                            //Output type in definition, but not in call
                            return Err(format!(
                                    "Function {} must output a {:?}, but call expects none.",
                                    fun_name, output_type
                            ));
                        }
                    }
                    else if let Some(expected) = expected_call_type {
                        //Output type in call, but not in definition
                        return Err(format!(
                                "Function {} call expects {:?}, but it must not.",
                                fun_name, expected
                        ));
                    }
                }
            } else {
                //The first operand is not an identifier of a function
                return Err(format!("{} is not a function", fun_name));
            }
        }
        else {
            //The first operand is not an identifier
            return Err(format!(
                    "Operator {:?} requires a function to call, but none was provided.",
                    operator,
            ));
        };
        Ok(())
    }
    
    fn type_expression(&self, expression: &Expression) -> Result<Type, String> {
        match expression {
            Expression::Atomic(Atomic::Constant(constant)) => {
                // Check the type of the constant expression
                match constant {
                    ConstantType::FF(_) => Ok(Type::Variable(NumericType::FiniteField)),
                    ConstantType::I64(_) => Ok(Type::Variable(NumericType::Integer)),
                }
            }
            Expression::Atomic(Atomic::Variable(variable)) => {
                // Check if the variable is in the environment
                if let Some(ty) = self.variables_enviroment.get(variable) {
                    Ok(ty.clone())
                } else {
                    Err(format!("Variable {} not found in environment", variable))
                }
            }
            Expression::Atomic(Atomic::Function(function)) => {
                // Check if the function is in the environment
                if let Some(ty) = self.functions_enviroment.get(function) {
                    Ok(ty.clone())
                } else {
                    Err(format!("Function {} not found in environment", function))
                }
            }
            //TODO: Properly check the parameters
            Expression::Parameter(parameter) => {
                match parameter {
                    Parameter::I64Memory { index: _, size: _ } => {
                        Ok(Type::Variable(NumericType::Integer))
                    },
                    _ => {
                        Ok(Type::Variable(NumericType::FiniteField))
                    }
                }
            }
        }
    }
}
