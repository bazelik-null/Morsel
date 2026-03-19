// Copyright (c) 2026 bazelik-null

use crate::morsel_core::evaluating::functions::FunctionTable;
use crate::morsel_core::lexing::operators::OperatorType;
use crate::morsel_core::lexing::token::{LiteralValue, Token};
use std::sync::Arc;

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
        let mut chars = input.chars().peekable();

        while let Some(&ch) = chars.peek() {
            match ch {
                // Skip whitespace
                ' ' | '\t' | '\n' | '\r' => {
                    chars.next();
                }
                // String literals
                '"' => {
                    chars.next();
                    let string = self.parse_string(&mut chars)?;
                    tokens.push(Token::Literal(LiteralValue::String(string)));
                }
                // Numbers
                '0'..='9' => {
                    let number = self.parse_number(&mut chars);
                    tokens.push(number);
                }
                // Identifiers, keywords, functions, types
                'a'..='z' | 'A'..='Z' | '_' => {
                    let ident = self.parse_identifier(&mut chars);
                    tokens.push(self.classify_identifier(&ident));
                }
                // Operators and punctuation
                '+' | '-' | '*' | '/' | '%' | '^' | '(' | ')' | ',' | '=' | ';' | ':' => {
                    chars.next();
                    let op = self.parse_operator(ch, &tokens)?;
                    tokens.push(Token::Operator(op));
                }
                _ => {
                    return Err(format!("Unexpected character: '{}'", ch));
                }
            }
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

    fn classify_identifier(&self, ident: &str) -> Token {
        if let Some(value) = self.parse_boolean(ident) {
            return Token::Literal(LiteralValue::Boolean(value));
        }

        if let Some(value) = self.parse_constant(ident) {
            return Token::Literal(LiteralValue::Float(value));
        }

        if RESERVED_KEYWORDS.contains(&ident) {
            return Token::Keyword(ident.to_string());
        }

        if RESERVED_TYPES.contains(&ident) {
            return Token::Type(ident.to_string());
        }

        if self.func_table.is_function(ident) {
            return Token::Function(ident.to_string());
        }

        Token::Identifier(ident.to_string())
    }

    /// Parse constants
    fn parse_constant(&self, lexeme: &str) -> Option<f64> {
        match lexeme {
            "pi" => Some(std::f64::consts::PI),
            "e" => Some(std::f64::consts::E),
            _ => None,
        }
    }

    /// Parse boolean literals
    fn parse_boolean(&self, lexeme: &str) -> Option<bool> {
        match lexeme {
            "true" => Some(true),
            "false" => Some(false),
            _ => None,
        }
    }

    /// Parse variable names
    fn parse_identifier(&self, chars: &mut std::iter::Peekable<std::str::Chars>) -> String {
        let mut ident = String::new();

        while let Some(&ch) = chars.peek() {
            match ch {
                'a'..='z' | 'A'..='Z' | '0'..='9' | '_' => {
                    ident.push(ch);
                    chars.next();
                }
                _ => break,
            }
        }

        ident
    }

    /// Parse string literals (removes quotes and handles escape sequences)
    fn parse_string(
        &self,
        chars: &mut std::iter::Peekable<std::str::Chars>,
    ) -> Result<String, String> {
        let mut result = String::new();
        let mut escaped = false;

        while let Some(&ch) = chars.peek() {
            chars.next();

            if escaped {
                match ch {
                    'n' => result.push('\n'),
                    't' => result.push('\t'),
                    'r' => result.push('\r'),
                    '"' => result.push('"'),
                    '\\' => result.push('\\'),
                    _ => {
                        result.push('\\');
                        result.push(ch);
                    }
                }
                escaped = false;
            } else if ch == '\\' {
                escaped = true;
            } else if ch == '"' {
                return Ok(result);
            } else {
                result.push(ch);
            }
        }

        Err("Unterminated string literal".to_string())
    }

    /// Parse numbers
    fn parse_number(&self, chars: &mut std::iter::Peekable<std::str::Chars>) -> Token {
        let mut number = String::new();
        let mut is_float = false;

        while let Some(&ch) = chars.peek() {
            match ch {
                '0'..='9' => {
                    number.push(ch);
                    chars.next();
                }
                '.' if !is_float && chars.clone().nth(1).is_some_and(|c| c.is_ascii_digit()) => {
                    is_float = true;
                    number.push(ch);
                    chars.next();
                }
                _ => break,
            }
        }

        if is_float {
            Token::Literal(LiteralValue::Float(number.parse().unwrap()))
        } else {
            Token::Literal(LiteralValue::Integer(number.parse().unwrap()))
        }
    }

    /// Parse operators
    fn parse_operator(&self, ch: char, preceding_tokens: &[Token]) -> Result<OperatorType, String> {
        let op = match ch {
            '+' => OperatorType::Add,
            '-' => OperatorType::Subtract,
            '*' => OperatorType::Multiply,
            '/' => OperatorType::Divide,
            '%' => OperatorType::Modulo,
            '^' => OperatorType::Exponent,
            '(' => OperatorType::LParen,
            ')' => OperatorType::RParen,
            ',' => OperatorType::Comma,
            '=' => OperatorType::Assign,
            ';' => OperatorType::Semicolon,
            ':' => OperatorType::Colon,
            _ => return Err(format!("Unknown operator: {}", ch)),
        };

        let final_op = if op == OperatorType::Subtract && self.should_be_unary(preceding_tokens) {
            OperatorType::Negate
        } else {
            op
        };

        Ok(final_op)
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
