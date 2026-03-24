use crate::core::compiler::parser::tree::Type;
use crate::core::compiler::preprocessor::token::{LiteralValue, OperatorValue};

pub fn infer_literal_type(lit: &LiteralValue) -> Type {
    match lit {
        LiteralValue::Integer(_) => Type::Integer,
        LiteralValue::Float(_) => Type::Float,
        LiteralValue::Boolean(_) => Type::Boolean,
        LiteralValue::String(_) => Type::String,
    }
}

pub fn infer_binary_type(
    lhs: &Type,
    op: &OperatorValue,
    rhs: &Type,

    errors: &mut Vec<String>,
) -> Result<Type, ()> {
    match op {
        OperatorValue::Plus
        | OperatorValue::Minus
        | OperatorValue::Multiply
        | OperatorValue::Divide
        | OperatorValue::Modulo => arithmetic_type(lhs, rhs, op, errors),

        OperatorValue::Less
        | OperatorValue::LessEqual
        | OperatorValue::Greater
        | OperatorValue::GreaterEqual
        | OperatorValue::Equal
        | OperatorValue::NotEqual => comparison_type(lhs, rhs, errors),

        OperatorValue::And | OperatorValue::Or => logical_type(lhs, rhs, errors),

        _ => {
            errors.push(format!("Unsupported binary operator: {}", op));
            Err(())
        }
    }
}

pub fn infer_unary_type(
    op: &OperatorValue,
    rhs: &Type,
    errors: &mut Vec<String>,
) -> Result<Type, ()> {
    match op {
        OperatorValue::Minus => match rhs {
            Type::Integer | Type::Float => Ok(rhs.clone()),
            _ => {
                errors.push(format!("Cannot negate type {}", rhs));
                Err(())
            }
        },
        OperatorValue::Not => {
            if rhs == &Type::Boolean {
                Ok(Type::Boolean)
            } else {
                errors.push(format!("Logical NOT requires boolean, got {}", rhs));
                Err(())
            }
        }
        _ => {
            errors.push(format!("Unsupported unary operator: {}", op));
            Err(())
        }
    }
}

fn arithmetic_type(
    lhs: &Type,
    rhs: &Type,
    op: &OperatorValue,
    errors: &mut Vec<String>,
) -> Result<Type, ()> {
    match (lhs, rhs) {
        // Integer arithmetic
        (Type::Integer, Type::Integer) => Ok(Type::Integer),

        // Float arithmetic
        (Type::Float, Type::Float) => Ok(Type::Float),

        // Mixed numeric types - result is Float
        (Type::Integer, Type::Float) | (Type::Float, Type::Integer) => {
            // Modulo only works on integers
            if matches!(op, OperatorValue::Modulo) {
                errors.push("Modulo operator requires integer operands".to_string());
                return Err(());
            }
            Ok(Type::Float)
        }

        // String concatenation
        (Type::String, Type::String) if matches!(op, OperatorValue::Plus) => Ok(Type::String),

        // Invalid types
        _ => {
            errors.push(format!(
                "Invalid operand types for {}: {} and {}",
                op, lhs, rhs
            ));
            Err(())
        }
    }
}

fn comparison_type(lhs: &Type, rhs: &Type, errors: &mut Vec<String>) -> Result<Type, ()> {
    if types_compatible(lhs, rhs) {
        Ok(Type::Boolean)
    } else {
        errors.push(format!("Cannot compare types {} and {}", lhs, rhs));
        Err(())
    }
}

fn logical_type(lhs: &Type, rhs: &Type, errors: &mut Vec<String>) -> Result<Type, ()> {
    if lhs == &Type::Boolean && rhs == &Type::Boolean {
        Ok(Type::Boolean)
    } else {
        errors.push(format!(
            "Logical operators require boolean operands, got {} and {}",
            lhs, rhs
        ));
        Err(())
    }
}

/// Check if "actual" type can be used where "expected" type is required.
/// Same types are always compatible
/// Integer can be implicitly converted to Float
/// Array types are compatible if their element types are compatible
pub fn types_compatible(actual: &Type, expected: &Type) -> bool {
    match (actual, expected) {
        (a, b) if a == b => true,
        (Type::Integer, Type::Float) => true,
        (Type::Array(a), Type::Array(b)) => types_compatible(a, b),
        _ => false,
    }
}
