use crate::interpreter::tokens::{OperatorType, Token};

pub fn eval(tokens: &[Token]) -> Result<f64, &'static str> {
    if tokens.is_empty() {
        return Err("No tokens found");
    }

    let mut result = 0.0;
    let mut operation = OperatorType::ADD;
    let mut last_number: Option<f64> = None;

    // Evaluate each token
    for token in tokens {
        match token {
            Token::Number(n) => {
                last_number = Some(*n);
            }
            Token::Operator(op_type) => {
                let number = last_number.ok_or("Expected number before operator")?;

                result = apply_operation(result, number, &operation);

                operation = *op_type;
                last_number = None;
            }
        }
    }

    // Apply the final operation
    let final_number = last_number.ok_or("Expression ends with operator")?;
    result = apply_operation(result, final_number, &operation);

    Ok(result)
}

fn apply_operation(result: f64, value: f64, operation: &OperatorType) -> f64 {
    match operation {
        OperatorType::ADD => result + value,
        OperatorType::SUBTRACT => result - value,
        OperatorType::MULTIPLY => result * value,
        OperatorType::DIVIDE => result / value,
        _ => result,
    }
}
