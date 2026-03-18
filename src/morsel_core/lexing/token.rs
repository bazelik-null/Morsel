// Copyright (c) 2026 bazelik-null

use crate::morsel_core::lexing::operators::OperatorType;

#[derive(Debug, Clone, PartialEq)]
pub enum LiteralValue {
    Integer(i64),
    Float(f64),
    String(String),
    Boolean(bool),
}

#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    Literal(LiteralValue),  // Plain numbers
    Operator(OperatorType), // Operators like '+', '-'
    Function(String),       // Functions
    Keyword(String),        // For 'let', 'if', 'else', etc.
    Identifier(String),     // For variable names
    Type(String),           // For variable types
}

impl Token {
    pub fn as_operator(&self) -> Option<&OperatorType> {
        match self {
            Token::Operator(op) => Some(op),
            _ => None,
        }
    }
    pub fn as_function(&self) -> Option<&String> {
        match self {
            Token::Function(func) => Some(func),
            _ => None,
        }
    }
}
