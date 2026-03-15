use crate::interpreter::ast::node::Node;
use crate::interpreter::operators::OperatorType;

/// Evaluates an AST and returns evaluation result.
pub fn eval(node: &Node) -> Result<f64, String> {
    eval_node(node)
}

fn eval_node(node: &Node) -> Result<f64, String> {
    match node {
        // Number
        Node::Number(value) => Ok(*value),

        // Unary expression
        Node::UnaryExpr { op, child } => {
            // Evaluate child node
            let child_value = eval_node(child)?;

            apply_unary_operation(child_value, op)
        }

        // Binary expression
        Node::BinaryExpr { op, lvalue, rvalue } => {
            // Evaluate lvalue
            let left = eval_node(lvalue)?;
            // Evaluate rvalue
            let right = eval_node(rvalue)?;

            apply_binary_operation(left, right, op)
        }
    }
}

fn apply_unary_operation(value: f64, operation: &OperatorType) -> Result<f64, String> {
    match operation {
        OperatorType::Negate => Ok(-value),
        _ => Err(format!("Invalid unary operator: {:?}", operation)),
    }
}

fn apply_binary_operation(left: f64, right: f64, operation: &OperatorType) -> Result<f64, String> {
    match operation {
        OperatorType::Add => Ok(left + right),
        OperatorType::Subtract => Ok(left - right),
        OperatorType::Multiply => Ok(left * right),
        OperatorType::Divide => {
            if right == 0.0 {
                Err("Division by zero".to_string())
            } else {
                Ok(left / right)
            }
        }
        _ => Err(format!("Invalid binary operator: {:?}", operation)),
    }
}
