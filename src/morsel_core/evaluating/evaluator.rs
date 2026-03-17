// Copyright (c) 2026 bazelik-null

use crate::morsel_core::evaluating::functions::FunctionTable;
use crate::morsel_core::lexing::operators::OperatorType;
use crate::morsel_core::parsing::node::Node;

use std::sync::Arc;

/// Evaluates an AST and returns the result
pub struct Evaluator {
    func_table: Arc<FunctionTable>,
}

impl Evaluator {
    /// Create a new evaluating with a function table
    pub fn new(func_table: Arc<FunctionTable>) -> Self {
        Evaluator { func_table }
    }

    /// Evaluate an AST node
    pub fn eval(&self, node: &Node) -> Result<f64, String> {
        self.eval_node(node)
    }

    fn eval_node(&self, node: &Node) -> Result<f64, String> {
        match node {
            // Number
            Node::Number(value) => Ok(*value),

            // Unary expression
            Node::UnaryExpr { op, child } => {
                let value = self.eval_node(child)?;
                self.apply_unary(*op, value)
            }

            // Function call
            Node::FunctionCall { func, args } => {
                // Evaluate all arguments
                let values: Result<Vec<f64>, String> =
                    args.iter().map(|arg| self.eval_node(arg)).collect();

                self.apply_function(func, values?)
            }

            // Binary expression
            Node::BinaryExpr { op, lvalue, rvalue } => {
                // Evaluate lvalue
                let left = self.eval_node(lvalue)?;
                // Evaluate rvalue
                let right = self.eval_node(rvalue)?;

                self.apply_binary(*op, left, right)
            }
        }
    }

    /// Applies a unary operation
    fn apply_unary(&self, op: OperatorType, value: f64) -> Result<f64, String> {
        match op {
            OperatorType::Negate => Ok(-value),
            _ => Err(format!("'{}' is not a unary operator", op)),
        }
    }

    /// Applies a binary operation
    fn apply_binary(&self, op: OperatorType, left: f64, right: f64) -> Result<f64, String> {
        match op {
            OperatorType::Add => Ok(left + right),
            OperatorType::Subtract => Ok(left - right),
            OperatorType::Multiply => Ok(left * right),
            OperatorType::Divide => Ok(left / right),

            OperatorType::Exponent => Ok(left.powf(right)),
            OperatorType::Modulo => Ok(left.rem_euclid(right)),

            _ => Err(format!("'{}' is not a binary operator", op)),
        }
    }

    /// Applies a function with variable arguments
    fn apply_function(&self, func: &str, args: Vec<f64>) -> Result<f64, String> {
        // Validate arguments first
        self.func_table.validate_args(func, args.len())?;

        match func {
            "sqrt" => Ok(args[0].sqrt()),
            "ln" => Ok(args[0].ln()),
            "sin" => Ok(args[0].sin()),
            "cos" => Ok(args[0].cos()),
            "tan" => Ok(args[0].tan()),
            "asin" => Ok(args[0].asin()),
            "acos" => Ok(args[0].acos()),
            "atan" => Ok(args[0].atan()),
            "abs" => Ok(args[0].abs()),
            "round" => Ok(args[0].round()),
            "log" => Ok(args[1].log(args[0])),
            "min" => Ok(args.iter().copied().fold(f64::INFINITY, f64::min)),
            "max" => Ok(args.iter().copied().fold(f64::NEG_INFINITY, f64::max)),

            _ => Err(format!("Unknown function: '{}'", func)),
        }
    }
}
