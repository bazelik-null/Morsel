// Copyright (c) 2026 bazelik-null

use std::str::FromStr;

/// Represents any value in the interpreter
#[derive(Clone, Debug, PartialEq)]
pub enum Value {
    Float(f64),
    Integer(i64),
    String(String),
    Boolean(bool),
    Null,
}

/// Represents the type of value for strict type checking
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub enum Type {
    Float,
    Integer,
    String,
    Boolean,
    Null,
    Any, // For functions that accept any type
}

impl Type {
    /// Get the type of value
    pub fn of(value: &Value) -> Self {
        match value {
            Value::Float(_) => Type::Float,
            Value::Integer(_) => Type::Integer,
            Value::String(_) => Type::String,
            Value::Boolean(_) => Type::Boolean,
            Value::Null => Type::Null,
        }
    }

    /// Check if this type is compatible with another (for implicit conversions)
    pub fn is_compatible_with(&self, other: &Type) -> bool {
        match (self, other) {
            (a, b) if a == b => true,
            (Type::Any, _) | (_, Type::Any) => true,
            // Integer can be used where Float is expected
            (Type::Integer, Type::Float) => true,
            (Type::Float, Type::Integer) => true,
            _ => false,
        }
    }

    /// Check if implicit conversion is allowed
    pub fn can_convert_to(&self, target: &Type) -> bool {
        match (self, target) {
            (a, b) if a == b => true,
            (Type::Any, _) | (_, Type::Any) => true,
            // Allow numeric conversions
            (Type::Integer, Type::Float) => true,
            (Type::Float, Type::Integer) => true,
            // Allow string conversion from any type
            (_, Type::String) => true,
            _ => false,
        }
    }
}

impl FromStr for Type {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "float" => Ok(Type::Float),
            "int" => Ok(Type::Integer),
            "string" => Ok(Type::String),
            "bool" => Ok(Type::Boolean),
            "null" => Ok(Type::Null),
            "any" => Ok(Type::Any),

            _ => Err(format!("Unknown type: {}", s)),
        }
    }
}

impl std::fmt::Display for Type {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Type::Float => write!(f, "float"),
            Type::Integer => write!(f, "int"),
            Type::String => write!(f, "string"),
            Type::Boolean => write!(f, "bool"),
            Type::Null => write!(f, "null"),
            Type::Any => write!(f, "any"),
        }
    }
}

impl Value {
    /// Get the type of this value
    pub fn type_of(&self) -> Type {
        Type::of(self)
    }

    /// Convert to f64 (only for numeric types)
    pub fn to_float(&self) -> Result<f64, String> {
        match self {
            Value::Float(f) => Ok(*f),
            Value::Integer(i) => Ok(*i as f64),
            _ => Err(format!("Cannot convert {} to float", self.type_of())),
        }
    }

    /// Convert to integer (only for numeric types)
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

    /// Display value (for print function)
    pub fn display(&self) -> String {
        match self {
            Value::String(s) => s.clone(),
            Value::Float(f) => f.to_string(),
            Value::Integer(i) => i.to_string(),
            Value::Boolean(b) => b.to_string(),
            Value::Null => "null".to_string(),
        }
    }
}

impl std::fmt::Display for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.display())
    }
}
