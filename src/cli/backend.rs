// Copyright (c) 2026 bazelik-null

use crate::morsel_interpreter::Interpreter;
use std::{fs, io};

pub fn cli_execute(input: &str, is_debug: bool) -> Result<(), String> {
    let mut interpreter = Interpreter::new(is_debug);
    interpreter.execute(input)
}

/// Reads file and evaluates it.
pub fn eval_file(file_path: &str, is_debug: bool) -> io::Result<()> {
    let input = fs::read_to_string(file_path)?;
    cli_execute(input.trim(), is_debug).map_err(io::Error::other)?;
    Ok(())
}
