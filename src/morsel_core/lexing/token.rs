// Copyright (c) 2026 bazelik-null

use crate::morsel_core::lexing::operators::OperatorType;

#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    Operator(OperatorType),
    Function(String),
    Number(f64),
}

impl Token {
    pub fn is_number(&self) -> bool {
        matches!(self, Token::Number(_))
    }

    pub fn is_operator(&self) -> bool {
        matches!(self, Token::Operator(_))
    }

    pub fn is_function(&self) -> bool {
        matches!(self, Token::Function(_))
    }

    pub fn as_number(&self) -> Option<f64> {
        match self {
            Token::Number(value) => Some(*value),
            _ => None,
        }
    }

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
