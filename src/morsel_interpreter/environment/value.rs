// Copyright (c) 2026 bazelik-null

use crate::morsel_interpreter::environment::types::Type;

/// Represents any value in the language
#[derive(Clone, Debug, PartialEq)]
pub enum Value {
    Float(f64),
    Integer(i64),
    String(String),
    Boolean(bool),
    Null,
}

impl Value {
    /// Get the type of this value
    pub fn type_of(&self) -> Type {
        Type::of(self)
    }

    /// Convert to f64 (only for numeric types)
    /// Integers are automatically promoted to floats.
    pub fn to_float(&self) -> Result<f64, String> {
        match self {
            Value::Float(f) => Ok(*f),
            Value::Integer(i) => Ok(*i as f64),
            _ => Err(format!("Cannot convert {} to float", self.type_of())),
        }
    }

    /// Convert to integer (only for numeric types)
    /// Floats are truncated to integers.
    pub fn to_integer(&self) -> Result<i64, String> {
        match self {
            Value::Integer(i) => Ok(*i),
            Value::Float(f) => Ok(*f as i64),
            _ => Err(format!("Cannot convert {} to integer", self.type_of())),
        }
    }

    /// Convert to string (only for string types)
    pub fn to_string_value(&self) -> Result<String, String> {
        match self {
            Value::String(s) => Ok(s.clone()),
            _ => Err(format!("Cannot convert {} to string", self.type_of())),
        }
    }

    /// Convert to boolean (only for boolean types)
    pub fn to_bool(&self) -> Result<bool, String> {
        match self {
            Value::Boolean(b) => Ok(*b),
            _ => Err(format!("Cannot convert {} to boolean", self.type_of())),
        }
    }

    /// Convert to string for display purposes
    /// Used by the print/println functions. Strings are returned without quotes.
    pub fn display(&self) -> String {
        match self {
            Value::String(s) => s.clone(),
            Value::Float(f) => {
                if f.fract() == 0.0 && f.is_finite() {
                    format!("{:.1}", f)
                } else {
                    f.to_string()
                }
            }
            Value::Integer(i) => i.to_string(),
            Value::Boolean(b) => b.to_string(),
            Value::Null => "null".to_string(),
        }
    }

    /// Check if this value is truthy (for conditionals)
    pub fn is_truthy(&self) -> bool {
        match self {
            Value::Boolean(b) => *b,
            Value::Null => false,
            Value::Integer(i) => *i != 0,
            Value::Float(f) => *f != 0.0,
            Value::String(s) => !s.is_empty(),
        }
    }

    /// Check if this value is falsy
    pub fn is_falsy(&self) -> bool {
        !self.is_truthy() // Yeah
    }
}

impl std::fmt::Display for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.display())
    }
}
