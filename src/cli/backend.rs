// Copyright (c) 2026 bazelik-null

use crate::morsel_core::evaluating::variable::Value;
use crate::morsel_core::interpreter::Interpreter;
use std::{fs, io};

pub fn calculate(input: &str, is_debug: bool) -> Result<(), String> {
    let interpreter = Interpreter::new(is_debug);
    interpreter.execute(input)
}

pub fn calculate_with_result(input: &str, is_debug: bool) -> Result<f64, String> {
    let interpreter = Interpreter::new(is_debug);
    let result = interpreter.execute_with_result(input);
    match result {
        Ok(result) => match result {
            Value::Float(result) => Ok(result),
            Value::Integer(result) => Ok(result as f64),
            Value::String(_) => Err("Evaluation result is string.".into()),
            Value::Boolean(_) => Err("Evaluation result is boolean.".into()),
            Value::Null => Err("Evaluation result is null.".into()),
        },
        Err(err) => Err(err.to_string()),
    }
}

/// Reads file and evaluates it.
pub fn eval_file(file_path: &str, is_debug: bool) -> io::Result<()> {
    let input = fs::read_to_string(file_path)?;
    calculate(input.trim(), is_debug).map_err(io::Error::other)?;
    Ok(())
}
