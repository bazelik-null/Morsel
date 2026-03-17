// Copyright (c) 2026 bazelik-null

use crate::morsel_core::evaluating::functions::FunctionTable;
use crate::morsel_core::lexing::operators::OperatorType;
use crate::morsel_core::lexing::token::Token;
use once_cell::sync::Lazy;
use regex::Regex;
use std::sync::Arc;

static TOKENIZER_REGEX: Lazy<Regex> =
    Lazy::new(|| Regex::new(r"(\d+\.\d+|\d+|[a-zA-Z_]+|[+\-*/()=^%,])").unwrap());

/// Tokenizes input strings into a token array
pub struct Lexer {
    func_table: Arc<FunctionTable>,
}

impl Lexer {
    /// Create a new lexing with a function table
    pub fn new(func_table: Arc<FunctionTable>) -> Self {
        Lexer { func_table }
    }

    /// Tokenize input string into a token array
    pub fn tokenize(&self, input: &str) -> Result<Vec<Token>, String> {
        if input.trim().is_empty() {
            return Err("Input string is empty".to_string());
        }

        let cleaned = input.replace(char::is_whitespace, "").to_lowercase();
        let mut tokens = Vec::new();

        for m in TOKENIZER_REGEX.find_iter(&cleaned) {
            let token = self.parse_token(m.as_str(), &tokens, m.end())?;
            tokens.push(token);
        }

        Ok(tokens)
    }

    fn parse_token(
        &self,
        lexeme: &str,
        preceding_tokens: &[Token],
        pos: usize,
    ) -> Result<Token, String> {
        // Try parsing as number
        if let Ok(value) = lexeme.parse::<f64>() {
            return Ok(Token::Number(value));
        }

        // Try parsing as constant
        if let Some(value) = self.try_parse_constant(lexeme) {
            return Ok(Token::Number(value));
        }

        // Try parsing as operator
        if let Some(mut value) = self.try_parse_operator(lexeme) {
            if value == OperatorType::Subtract && self.is_unary_position(preceding_tokens) {
                value = OperatorType::Negate;
            }
            return Ok(Token::Operator(value));
        }

        // Try parsing as function
        if self.func_table.is_function(lexeme) {
            return Ok(Token::Function(lexeme.to_lowercase()));
        }

        Err(format!("Unknown token at position {}: '{}'", pos, lexeme))
    }

    /// Attempts to parse a constant like pi or e
    fn try_parse_constant(&self, lexeme: &str) -> Option<f64> {
        Some(match lexeme {
            "pi" => std::f64::consts::PI,
            "e" => std::f64::consts::E,
            _ => return None,
        })
    }

    /// Maps lexeme strings to operator types
    fn try_parse_operator(&self, lexeme: &str) -> Option<OperatorType> {
        let op = match lexeme {
            // Arithmetic
            "+" => OperatorType::Add,
            "-" => OperatorType::Subtract,
            "*" => OperatorType::Multiply,
            "/" => OperatorType::Divide,
            "%" => OperatorType::Modulo,

            // Exponentiation
            "^" => OperatorType::Exponent,

            // Syntax
            "(" => OperatorType::LParen,
            ")" => OperatorType::RParen,
            "," => OperatorType::Comma,

            _ => return None,
        };

        Some(op)
    }

    /// Determines if a minus operator should be treated as unary negation
    /// This happens when it appears at the start or after another operator
    fn is_unary_position(&self, preceding_tokens: &[Token]) -> bool {
        match preceding_tokens.last() {
            None => true,                                         // Start of expression
            Some(Token::Operator(OperatorType::RParen)) => false, // After closing paren (binary minus)
            Some(Token::Operator(_)) => true,                     // After other operators
            Some(Token::Function(_)) => false,                    // After any function
            Some(Token::Number(_)) => false,                      // After a number (binary minus)
        }
    }
}
