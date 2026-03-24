use crate::core::compiler::error_handler::CompilerError;
use crate::core::compiler::preprocessor::token::{LiteralValue, OperatorValue};
use lasso::Spur;
use std::fmt;
use std::fmt::Formatter;

pub struct ParserOutput {
    pub nodes: Vec<Node>,
    pub errors: Vec<CompilerError>,
}

impl Default for ParserOutput {
    fn default() -> Self {
        Self::new()
    }
}

impl ParserOutput {
    pub fn new() -> Self {
        Self {
            nodes: Vec::new(),
            errors: Vec::new(),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum Type {
    Integer,
    Float,
    Boolean,
    String,
    Array(Box<Type>),
    FixedArray(Box<Type>, usize),
    Void,
}

impl fmt::Display for Type {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Type::Integer => write!(f, "int"),
            Type::Float => write!(f, "float"),
            Type::Boolean => write!(f, "bool"),
            Type::String => write!(f, "string"),
            Type::Array(inner) => write!(f, "[{}]", inner),
            Type::FixedArray(inner, size) => write!(f, "[{}: {}]", inner, size),
            Type::Void => write!(f, "void"),
        }
    }
}

#[derive(Debug, Clone)]
pub struct Parameter {
    pub name: Spur,
    pub type_annotation: Type,
}

#[derive(Debug, Clone)]
pub enum Node {
    // Expressions
    Literal(LiteralValue),
    ArrayLiteral(Vec<Node>),
    Identifier(Spur),
    Unary {
        op: OperatorValue,
        rhs: Box<Node>,
    },
    Binary {
        lhs: Box<Node>,
        op: OperatorValue,
        rhs: Box<Node>,
    },
    Assignment {
        target: Box<Node>,
        value: Box<Node>,
    },

    // Statements
    Block(Vec<Node>),
    If {
        condition: Box<Node>,
        then_branch: Box<Node>,
        else_branch: Option<Box<Node>>,
    },
    While {
        condition: Box<Node>,
        body: Box<Node>,
    },
    VariableDecl {
        name: Spur,
        mutable: bool,
        type_annotation: Option<Type>,
        value: Box<Node>,
    },
    FunctionDecl {
        name: Spur,
        params: Vec<Parameter>,
        body: Box<Node>,
        return_type: Option<Type>,
    },
    FunctionCall {
        name: Box<Node>,
        args: Vec<Node>,
    },
    ArrayAccess {
        array: Box<Node>,
        index: Box<Node>,
    },
    Return(Option<Box<Node>>),
}
