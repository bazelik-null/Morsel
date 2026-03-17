// Copyright (c) 2026 bazelik-null

use crate::morsel_core::interpreter::Interpreter;
use std::{fs, io};

pub fn calculate(input: &str, is_debug: bool) -> Result<f64, String> {
    let interpreter = Interpreter::new(is_debug);
    interpreter.execute(input)
}

/// Reads file and evaluates it.
pub fn eval_file(file_path: &str, is_debug: bool) -> io::Result<()> {
    let input = fs::read_to_string(file_path)?;
    let result = calculate(input.trim(), is_debug).map_err(io::Error::other)?;
    println!("{}", result);
    Ok(())
}
