// Copyright (c) 2026 bazelik-null

use crate::cli::app_state::AppState;
use crate::cli::backend::{cli_execute_with_result, eval_file};
use crate::cli::cmd::Command;
use std::io::Write;

use rustyline::DefaultEditor;
use rustyline::error::ReadlineError;

const HISTORY_FILE: &str = ".morsel_history";
const PROMPT: &str = ">>> ";
const EDIT_PROMPT: &str = "... ";

pub fn cli_init() {
    print_banner();

    let mut editor = match DefaultEditor::new() {
        Ok(ed) => ed,
        Err(e) => {
            eprintln!("[ERROR]: Failed to initialize editor: {}", e);
            return;
        }
    };

    let mut state = AppState::default();

    // Main REPL loop
    loop {
        match editor.readline(PROMPT) {
            Ok(line) => {
                let trimmed = line.trim();

                // Skip empty lines
                if trimmed.is_empty() {
                    continue;
                }

                // Add to history
                if let Err(e) = editor.add_history_entry(trimmed) {
                    eprintln!("[WARN]: Failed to add history entry: {}", e);
                }

                // Process input
                if !handle_command(&mut state, trimmed, &mut editor) {
                    break;
                }
            }
            Err(ReadlineError::Interrupted) => {
                println!("\n[INFO]: Exiting...");
                break;
            }
            Err(ReadlineError::Eof) => {
                println!("\n[INFO]: Exiting...");
                break;
            }
            Err(e) => {
                eprintln!("[ERROR]: {}", e);
                break;
            }
        }
    }

    // Save history before exiting
    if let Err(e) = editor.save_history(HISTORY_FILE) {
        eprintln!("[WARN]: Failed to save history: {}", e);
    }
}

fn multiline_editor(editor: &mut DefaultEditor, is_debug: bool) {
    let mut buffer = String::new();
    println!("[INFO]: Entering edit mode. Type 'edit' on a new line to finish.\n");

    loop {
        match editor.readline(EDIT_PROMPT) {
            Ok(line) => {
                let trimmed = line.trim();

                // Check for exit command
                if trimmed == "edit" {
                    println!("[INFO]: Exiting edit mode.\n");
                    break;
                }

                // Add line to buffer
                if !buffer.is_empty() {
                    buffer.push('\n');
                }
                buffer.push_str(&line);

                // Add to history
                if let Err(e) = editor.add_history_entry(trimmed) {
                    eprintln!("[WARN]: Failed to add history entry: {}", e);
                }
            }
            Err(ReadlineError::Interrupted) => {
                println!("\n[INFO]: Edit cancelled.");
                return;
            }
            Err(ReadlineError::Eof) => {
                println!("\n[INFO]: EOF reached. Exiting edit mode.\n");
                break;
            }
            Err(e) => {
                eprintln!("[ERROR]: {}", e);
                return;
            }
        }
    }

    if buffer.is_empty() {
        return;
    }

    execute_expression(&buffer, is_debug);
}

// Command Processing

fn handle_command(state: &mut AppState, input: &str, editor: &mut DefaultEditor) -> bool {
    match Command::from_input(input) {
        Command::Exit => {
            return false;
        }
        Command::Debug => {
            state.toggle_debug();
            return true;
        }
        Command::Help => {
            print_help();
            return true;
        }
        Command::File(file_path) => {
            handle_file_command(file_path.as_str(), state.is_debug);
            return true;
        }
        Command::Edit => {
            multiline_editor(editor, state.is_debug);
            return true;
        }
        Command::Unknown => {
            // Continue to evaluation
        }
    }

    // Execute
    execute_expression(input, state.is_debug);
    true
}

fn handle_file_command(file_path: &str, debug: bool) {
    match eval_file(file_path, debug) {
        Ok(_) => {
            // eval_file prints result
        }
        Err(err) => {
            eprintln!("[ERROR]: Failed to evaluate file '{}': {}", file_path, err);
        }
    }
}

fn execute_expression(input: &str, debug: bool) {
    match cli_execute_with_result(input, debug) {
        Ok(Some(result)) => {
            println!("{}", result);
        }
        Ok(None) => {}
        Err(err) => {
            eprintln!("[ERROR]: {}", err);
        }
    }

    let _ = std::io::stdout().flush();
}

// UI

fn print_banner() {
    println!("======================================");
    println!("==== Morsel Interpreter Interface ====");
    println!("======================================");
    println!();
}

fn print_help() {
    println!("== Available commands ==\n");
    println!("  help       - Show this help message.");
    println!("  file       - Execute a file.");
    println!("  edit       - Start multiline editor.");
    println!("  debug      - Toggle debug mode.");
    println!("  exit       - Exit.\n");
}
