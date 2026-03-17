// Copyright (c) 2026 bazelik-null

use crate::interpreter::ast::parser;
use crate::interpreter::evaluator;
use crate::interpreter::tokenizer::lexer;
use std::{fs, io};

/// Takes a raw input string and:
/// 1. Parses string into Tokens array.
/// 2. Builds Abstract Syntax Tree (AST) from Tokens.
/// 3. Evaluates AST Nodes and returns result.
pub fn calculate(input: &str, is_debug: bool) -> Result<f64, String> {
    if is_debug {
        println!("[DEBUG]: Raw input: {}", input);
    }

    // Tokenize
    let tokens = lexer::tokenize(input)?;
    if is_debug {
        println!("[DEBUG]: Tokens: {:?}", tokens);
    }

    // Parse into AST
    let mut parser = parser::Parser::new(tokens);
    let ast = parser.parse()?;
    if is_debug {
        println!("[DEBUG]: Abstract Syntax Tree:\n {}", ast);
    }

    // Evaluate AST
    let result = evaluator::eval(&ast)?;

    Ok(result)
}

/// Reads file and evaluates it.
pub fn eval_file(file_path: &str, is_debug: bool) -> io::Result<()> {
    let input = fs::read_to_string(file_path)?;
    let result = calculate(input.trim(), is_debug).map_err(io::Error::other)?;
    println!("{}", result);
    Ok(())
}
