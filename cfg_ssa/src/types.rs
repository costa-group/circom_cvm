extern crate num_bigint_dig as num_bigint;
use std::fmt;
use num_bigint::BigInt;
use serde::Serialize;

#[derive(Debug, Clone, PartialEq, Serialize)]
pub enum NumericType {
    Integer,
    FiniteField,
}

#[derive(Debug, Clone, PartialEq, Serialize)]
pub enum Type {
    Variable(NumericType),
    /// Type of the output and type of the inputs
    Function(Option<NumericType>, Vec<(NumericType, Vec<usize>)>),
}

#[derive(Debug, Clone, PartialEq, Serialize)]
pub enum Operator {
    //TODO: There are more operations
    //Arithmetic
    Add,
    Sub,
    Mul,
    Div,
    Rem,
    IDiv,       //Only for ff
    Pow,

    //Relational
    Greater,
    GreaterEqual,
    Less,
    LessEqual,
    Equal,
    NotEqual,
    EqualZero,

    //Boolean
    And,
    Or,

    //Bit operations
    ShiftRight,
    ShiftLeft,
    BitAnd,
    BitOr,
    BitXor,
    BitNot,

    //Conversions
    Extend,     //i64 → ff
    Wrap,       //ff → i64

    //Memory
    Load,
    Store,
     //Stores for ff only
    MStore,
    MStoreFromSignal,
    MStoreFromCmpSignal,

    //Signal
    GetSignal,
    GetCmpSignal,
    SetSignal,
    MSetSignal,
    MSetSignalFromMemory,

    SetCmpIn,
    SetCmpInCnt,
    SetCmpInRun,
    SetCmpInCntCheck,

    MSetCmpIn,
    MSetCmpInCnt,
    MSetCmpInRun,
    MSetCmpInCntCheck,

    MSetCmpInFromCmp,
    MSetCmpInFromCmpCnt,
    MSetCmpInFromCmpRun,
    MSetCmpInFromCmpCntCheck,

    MSetCmpInFromMemory,
    MSetCmpInFromMemoryCnt,
    MSetCmpInFromMemoryRun,
    MSetCmpInFromMemoryCntCheck,

    //Functions
    Call,
    MCall,
    Return,
    MReturn,

    //Templates
    GetTemplateId,
    GetTemplateSignalPosition,
    GetTemplateSignalSize,
    GetTemplateSignalDim,
    GetTemplateSignalType,

    //Buses
    GetBusSignalPos,
    GetBusSignalSize,
    GetBusSignalDim,
    GetBusSignalType,

    //Misc
    Error,
    //TODO: outs
}

#[derive(Clone, PartialEq, Serialize)]
pub enum ConstantType {
    FF(BigInt),
    I64(i64),
}

impl fmt::Debug for ConstantType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ConstantType::FF(value) => write!(f, "{}", value),
            ConstantType::I64(value) => write!(f, "{}", value),
        }
    }
}

#[derive(Clone, PartialEq, Serialize)]
pub enum Atomic {
    Constant(ConstantType),
    Variable(String),
    Function(String),
}

impl fmt::Debug for Atomic {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Atomic::Constant(constant) => write!(f, "{:?}", constant),
            Atomic::Variable(var) => write!(f, "{}", var),
            Atomic::Function(fun) => write!(f, "${}", fun),
        }
    }
}

#[derive(Clone, PartialEq, Serialize)]
pub enum Parameter {
    //TODO: Not index is number of dimensions and length of each
    //Change second to vector of size first
    Signal {
        index: Atomic,
        size: Atomic,
    },
    SubcmpSignal {
        component: Atomic,
        index: Atomic,
        size: Atomic,
    },
    I64Memory {
        index: Atomic,
        size: Atomic,
    },
    FfMemory {
        index: Atomic,
        size: Atomic,
    },
}

impl fmt::Debug for Parameter {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Parameter::Signal { index, size } => write!(f, "signal({:?}, {:?})", index, size),
            Parameter::SubcmpSignal { component, index, size } => {
                write!(f, "subcmpsignal({:?}, {:?}, {:?})", component, index, size)
            }
            Parameter::I64Memory { index, size } => write!(f, "i64.memory({:?}, {:?})", index, size),
            Parameter::FfMemory { index, size } => write!(f, "ff.memory({:?}, {:?})", index, size),
        }
    }
}

#[derive(Clone, PartialEq, Serialize)]
pub enum Expression {
    Atomic(Atomic),
    Parameter(Parameter),
}

pub fn get_variable_names(expression: &Expression) -> Vec<String> {
    match expression {
        Expression::Atomic(atomic) => match atomic {
            Atomic::Variable(name) => vec![name.clone()],
            //TODO: I don't add them to the vector because they are not ssa variables in the same
            //sense as the rest of the variables. Maybe should be changed.
            Atomic::Function(name) => vec![],
            Atomic::Constant(_) => vec![],
        },
        Expression::Parameter(parameter) => match parameter {
            Parameter::Signal { index, size } => {
                let mut names = get_variable_names(&Expression::Atomic(index.clone()));
                names.extend(get_variable_names(&Expression::Atomic(size.clone())));
                names
            }
            Parameter::SubcmpSignal { component, index, size } => {
                let mut names = get_variable_names(&Expression::Atomic(component.clone()));
                names.extend(get_variable_names(&Expression::Atomic(index.clone())));
                names.extend(get_variable_names(&Expression::Atomic(size.clone())));
                names
            }
            Parameter::I64Memory { index, size } | Parameter::FfMemory { index, size } => {
                let mut names = get_variable_names(&Expression::Atomic(index.clone()));
                names.extend(get_variable_names(&Expression::Atomic(size.clone())));
                names
            }
        },
    }
}

impl fmt::Debug for Expression {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Expression::Atomic(atomic) => write!(f, "{:?}", atomic),
            Expression::Parameter(parameter) => write!(f, "{:?}", parameter),
        }
    }
}
