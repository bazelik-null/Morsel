// Copyright (c) 2026 bazelik-null

use crate::morsel_core::evaluating::functions::FunctionTable;
use crate::morsel_core::lexing::operators::OperatorType;
use crate::morsel_core::lexing::token::{LiteralValue, Token};
use once_cell::sync::Lazy;
use regex::Regex;
use std::sync::Arc;

static TOKENIZER_REGEX: Lazy<Regex> = Lazy::new(|| {
    Regex::new(r#"("(?:\\.|[^"\\])*"|\d+\.\d+|\d+|[a-zA-Z_]+|[+\-*/()=^%,;:])"#).unwrap()
});

static RESERVED_KEYWORDS: &[&str] = &["let", "mut", "if", "else", "fn", "for", "while"];
static RESERVED_TYPES: &[&str] = &["float", "int", "string", "bool", "null"];

/// Tokenizes input strings into a token array
pub struct Lexer {
    func_table: Arc<FunctionTable>,
}

impl Lexer {
    pub fn new(func_table: Arc<FunctionTable>) -> Self {
        Lexer { func_table }
    }

    /// Tokenize input string into a token array
    pub fn tokenize(&self, input: &str) -> Result<Vec<Token>, String> {
        if input.trim().is_empty() {
            return Err("Input string is empty".to_string());
        }

        let input = self.strip_comments(input);
        let mut tokens = Vec::new();

        for m in TOKENIZER_REGEX.find_iter(&input) {
            tokens.push(self.parse_token(m.as_str(), &tokens)?);
        }

        Ok(tokens)
    }

    /// Remove comments from input (// to end of line)
    fn strip_comments(&self, input: &str) -> String {
        input
            .lines()
            .filter_map(|line| {
                let trimmed = line.split("//").next()?.trim_end();
                if trimmed.is_empty() {
                    None
                } else {
                    Some(trimmed)
                }
            })
            .collect::<Vec<_>>()
            .join(" ")
    }

    /// Parse a single lexeme into a token
    fn parse_token(&self, lexeme: &str, preceding_tokens: &[Token]) -> Result<Token, String> {
        // Try to parse as boolean
        if let Some(value) = self.parse_boolean(lexeme) {
            return Ok(Token::Literal(LiteralValue::Boolean(value)));
        }

        // Try to parse as string
        if lexeme.starts_with('"') && lexeme.ends_with('"') {
            let string_value = self.parse_string(lexeme)?;
            return Ok(Token::Literal(LiteralValue::String(string_value)));
        }

        // Try to parse as int
        if let Ok(value) = lexeme.parse::<i64>() {
            return Ok(Token::Literal(LiteralValue::Integer(value)));
        }

        // Try to parse as float
        if let Ok(value) = lexeme.parse::<f64>() {
            return Ok(Token::Literal(LiteralValue::Float(value)));
        }

        // Try to parse as constant
        if let Some(value) = self.parse_constant(lexeme) {
            return Ok(Token::Literal(LiteralValue::Float(value)));
        }

        // Try to parse as operator
        if let Some(op) = self.parse_operator(lexeme, preceding_tokens) {
            return Ok(Token::Operator(op));
        }

        // Try to parse as keyword
        if RESERVED_KEYWORDS.contains(&lexeme) {
            return Ok(Token::Keyword(lexeme.to_string()));
        }

        // Try to parse as type
        if RESERVED_TYPES.contains(&lexeme) {
            return Ok(Token::Type(lexeme.to_string()));
        }

        // Try to parse as function
        if self.func_table.is_function(lexeme) {
            return Ok(Token::Function(lexeme.to_string()));
        }

        // Parse as variable reference
        Ok(Token::Identifier(lexeme.to_string()))
    }

    /// Parse constants
    fn parse_constant(&self, lexeme: &str) -> Option<f64> {
        match lexeme {
            "pi" => Some(std::f64::consts::PI),
            "e" => Some(std::f64::consts::E),
            _ => None,
        }
    }

    /// Parse operators
    fn parse_operator(&self, lexeme: &str, preceding_tokens: &[Token]) -> Option<OperatorType> {
        let op = match lexeme {
            "+" => OperatorType::Add,
            "-" => OperatorType::Subtract,
            "*" => OperatorType::Multiply,
            "/" => OperatorType::Divide,
            "%" => OperatorType::Modulo,
            "^" => OperatorType::Exponent,
            "(" => OperatorType::LParen,
            ")" => OperatorType::RParen,
            "," => OperatorType::Comma,
            "=" => OperatorType::Assign,
            ";" => OperatorType::Semicolon,
            ":" => OperatorType::Colon,
            _ => return None,
        };

        // Convert binary minus to unary negate when needed
        let final_op = if op == OperatorType::Subtract && self.should_be_unary(preceding_tokens) {
            OperatorType::Negate
        } else {
            op
        };

        Some(final_op)
    }

    /// Parse boolean literals
    fn parse_boolean(&self, lexeme: &str) -> Option<bool> {
        match lexeme {
            "true" => Some(true),
            "false" => Some(false),
            _ => None,
        }
    }

    /// Parse string literals (removes quotes and handles escape sequences)
    fn parse_string(&self, lexeme: &str) -> Result<String, String> {
        if !lexeme.starts_with('"') || !lexeme.ends_with('"') {
            return Err(format!("Invalid string literal: {}", lexeme));
        }

        let content = &lexeme[1..lexeme.len() - 1];
        let unescaped = content
            .replace("\\n", "\n")
            .replace("\\t", "\t")
            .replace("\\r", "\r")
            .replace("\\\"", "\"")
            .replace("\\\\", "\\");

        Ok(unescaped)
    }

    /// Determine if a minus operator should be treated as unary negation
    fn should_be_unary(&self, preceding_tokens: &[Token]) -> bool {
        match preceding_tokens.last() {
            None => true,
            Some(Token::Operator(op)) => !matches!(op, OperatorType::RParen),
            Some(Token::Function(_) | Token::Keyword(_)) => true,
            Some(Token::Literal(_) | Token::Identifier(_) | Token::Type(_)) => false,
        }
    }
}
