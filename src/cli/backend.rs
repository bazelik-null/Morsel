// Copyright (c) 2026 bazelik-null

use crate::morsel_core::evaluating::variable::Value;
use crate::morsel_core::interpreter::Interpreter;
use std::{fs, io};

pub fn cli_execute(input: &str, is_debug: bool) -> Result<(), String> {
    let interpreter = Interpreter::new(is_debug);
    interpreter.execute(input)
}

pub fn cli_execute_with_result(input: &str, is_debug: bool) -> Result<Option<f64>, String> {
    let interpreter = Interpreter::new(is_debug);
    let result = interpreter.execute_with_result(input);
    match result {
        Ok(result) => match result {
            Value::Float(result) => Ok(Some(result)),
            Value::Integer(result) => Ok(Some(result as f64)),
            Value::String(_) => Ok(None),
            Value::Boolean(_) => Ok(None),
            Value::Null => Ok(None),
        },
        Err(err) => Err(err.to_string()),
    }
}

/// Reads file and evaluates it.
pub fn eval_file(file_path: &str, is_debug: bool) -> io::Result<()> {
    let input = fs::read_to_string(file_path)?;
    cli_execute(input.trim(), is_debug).map_err(io::Error::other)?;
    Ok(())
}
