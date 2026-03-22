// Copyright (c) 2026 bazelik-null

use std::fmt;

/// Lower numbers = lower precedence
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Precedence {
    Additive = 1,
    Multiplicative = 2,
    Exponent = 3,
}

impl Precedence {
    pub fn next_higher(self) -> Self {
        match self {
            Precedence::Additive => Precedence::Multiplicative,
            Precedence::Multiplicative => Precedence::Exponent,
            Precedence::Exponent => Precedence::Exponent,
        }
    }
}

#[derive(Debug, Default, Copy, Clone, PartialEq)]
pub enum SyntaxOperator {
    // Arithmetic
    Add,      // x + y
    Subtract, // x - y
    Multiply, // x * y
    Divide,   // x / y
    // Exponents
    Exponent, // x ^ y
    // Misc
    Negate, // -x
    Modulo, // x % y
    // Syntax
    LParen,      // (
    RParen,      // )
    CurlyLParen, // {
    CurlyRParen, // }
    Comma,       // ,
    Assign,      // =
    Semicolon,   // ;
    Colon,       // :

    #[default]
    Unknown,
}

impl SyntaxOperator {
    /// Returns precedence for binary operators
    pub fn precedence(&self) -> Option<Precedence> {
        Some(match self {
            Self::Add | Self::Subtract => Precedence::Additive,
            Self::Multiply | Self::Divide | Self::Modulo => Precedence::Multiplicative,
            Self::Exponent => Precedence::Exponent,
            _ => return None,
        })
    }

    pub fn is_right_associative(&self) -> bool {
        matches!(self, Self::Exponent)
    }

    pub fn is_unary(&self) -> bool {
        matches!(self, Self::Negate)
    }

    pub fn is_postfix(&self) -> bool {
        false
    }
}

impl fmt::Display for SyntaxOperator {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            // Arithmetic
            SyntaxOperator::Add => write!(f, "+"),
            SyntaxOperator::Subtract => write!(f, "-"),
            SyntaxOperator::Multiply => write!(f, "*"),
            SyntaxOperator::Divide => write!(f, "/"),
            // Exponents
            SyntaxOperator::Exponent => write!(f, "^"),
            // Misc
            SyntaxOperator::Negate => write!(f, "-"),
            SyntaxOperator::Modulo => write!(f, "%"),
            // Syntax
            SyntaxOperator::LParen => write!(f, "("),
            SyntaxOperator::RParen => write!(f, ")"),
            SyntaxOperator::CurlyLParen => write!(f, "{{"),
            SyntaxOperator::CurlyRParen => write!(f, "}}"),
            SyntaxOperator::Comma => write!(f, ","),
            SyntaxOperator::Assign => write!(f, "="),
            SyntaxOperator::Semicolon => write!(f, ";"),
            SyntaxOperator::Colon => write!(f, ":"),

            SyntaxOperator::Unknown => write!(f, "?"),
        }
    }
}
