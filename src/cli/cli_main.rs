use crate::interpreter;
use crate::interpreter::ast::parser;
use crate::interpreter::tokenizer::lexer;

use crate::cli::app_state::AppState;
use crate::cli::calc_errors::CalcError;
use crate::cli::cmd::Command;

use std::io::{self, Write};

pub fn cli_init() {
    print_banner();
    let mut state = AppState::default();
    let mut input_buffer = String::new();

    loop {
        if !handle_input_cycle(&mut state, &mut input_buffer) {
            break;
        }
    }

    println!("[INFO]: Exiting...");
}

// Input handling

fn handle_input_cycle(state: &mut AppState, input_buffer: &mut String) -> bool {
    print_prompt();

    input_buffer.clear();
    if let Err(e) = io::stdin().read_line(input_buffer) {
        eprintln!("[ERROR]: {}", CalcError::IoError(e.to_string()));
        return true; // Continue on IO error
    }

    let trimmed = input_buffer.trim();

    // Skip empty lines
    if trimmed.is_empty() {
        return true;
    }

    // Check for commands
    match Command::from_input(trimmed) {
        Command::Exit | Command::Quit => return false,
        Command::Debug => {
            state.toggle_debug();
            return true;
        }
        Command::Help => {
            print_help();
            return true;
        }
        Command::Unknown => {} // Continue to calculation
    }

    // Calculate and display result
    match calculate(trimmed, state.is_debug) {
        Ok(result) => println!("{}", result),
        Err(err) => eprintln!("[ERROR]: {}", err),
    }

    true
}

// Backend

/// Takes a raw input string and:
/// 1. Parses string into Tokens array.
/// 2. Builds Abstract Syntax Tree (AST) from Tokens.
/// 3. Evaluates AST Nodes and returns result.
fn calculate(input: &str, is_debug: bool) -> Result<f64, CalcError> {
    if is_debug {
        println!("[DEBUG]: Raw input: {}", input);
    }

    // Tokenize
    let tokens = lexer::tokenize(input).map_err(|e| CalcError::Tokenize(e.to_string()))?;

    if is_debug {
        println!("[DEBUG]: Tokens: {:?}", tokens);
    }

    // Parse into AST
    let mut parser = parser::Parser::new(tokens);
    let ast = parser
        .parse()
        .map_err(|e| CalcError::Parse(e.to_string()))?;

    if is_debug {
        println!("[DEBUG]: Raw AST: {:?}", ast);
        println!("[DEBUG]: Pretty AST: {}", ast);
    }

    // Evaluate AST
    let result =
        interpreter::evaluator::eval(&ast).map_err(|e| CalcError::Evaluate(e.to_string()))?;

    Ok(result)
}

// UI

fn print_banner() {
    println!("====================================================");
    println!("==== RetardCalc: Rust math expression evaluator ====");
    println!("====================================================");
    println!();
}

fn print_prompt() {
    print!(">>> ");
    io::stdout().flush().expect("Failed to flush stdout");
}

fn print_help() {
    println!("Available commands:");
    println!("  help     - Show this help message.");
    println!("  debug    - Toggle debug mode.");
    println!("  exit     - Exit.");
    println!();
    println!("Enter any mathematical expression to evaluate it.");
    println!();
}
