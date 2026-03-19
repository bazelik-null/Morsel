// Copyright (c) 2026 bazelik-null

use crate::morsel_core::evaluating::functions::FunctionTable;
use crate::morsel_core::evaluating::stack::ScopeStack;
use crate::morsel_core::evaluating::variable::{Type, Value};
use crate::morsel_core::parsing::node::Node;
use std::sync::Arc;

/// Evaluates an AST
pub struct Evaluator {
    func_table: Arc<FunctionTable>,
    scopes: ScopeStack,
}

impl Evaluator {
    /// Create a new evaluator with a function table
    pub fn new(func_table: Arc<FunctionTable>) -> Self {
        Evaluator {
            func_table,
            scopes: ScopeStack::new(),
        }
    }

    /// Evaluate an AST node
    pub fn eval(&mut self, node: &Node) -> Result<Value, String> {
        match node {
            Node::Literal(value) => Ok(value.clone()),
            Node::Variable(name) => self.get_variable(name),
            Node::Block(statements) => self.eval_block(statements),
            Node::Statement {
                name,
                mutability,
                value,
                type_annotation,
            } => self.eval_let(name, *mutability, value, type_annotation),
            Node::Assignment { name, value } => self.eval_assignment(name, value),
            Node::Call { name, args } => self.eval_call(name, args),
        }
    }

    /// Get a variable value
    pub fn get_variable(&self, name: &str) -> Result<Value, String> {
        self.scopes
            .get(name)
            .ok_or_else(|| format!("Undefined variable: '{}'", name))
    }

    /// Evaluate a block of statements, returning the last result
    fn eval_block(&mut self, statements: &[Node]) -> Result<Value, String> {
        self.scopes.push_scope();

        let mut result = Value::Null;
        for stmt in statements {
            result = self.eval(stmt)?;
        }

        self.scopes.pop_scope();
        Ok(result)
    }

    /// Evaluate a let binding
    fn eval_let(
        &mut self,
        name: &str,
        mutable: bool,
        value: &Node,
        type_annotation: &Option<Type>,
    ) -> Result<Value, String> {
        let result = self.eval(value)?;

        // Check type annotation if provided
        if let Some(expected_type) = type_annotation {
            let actual_type = result.type_of();
            if actual_type != *expected_type {
                return Err(format!(
                    "Type annotation mismatch for '{}': expected {}, got {}",
                    name, expected_type, actual_type
                ));
            }
        }

        self.scopes
            .set_local(name.to_string(), result.clone(), mutable)?;
        Ok(result)
    }

    /// Evaluate an assignment to an existing variable
    fn eval_assignment(&mut self, name: &str, value: &Node) -> Result<Value, String> {
        let result = self.eval(value)?;
        self.scopes.set_existing(name, result.clone())?;
        Ok(result)
    }

    /// Evaluate a function call or operator
    fn eval_call(&mut self, name: &str, args: &[Node]) -> Result<Value, String> {
        let values: Result<Vec<Value>, String> = args.iter().map(|arg| self.eval(arg)).collect();
        let values = values?;

        match name {
            "+" | "-" | "*" | "/" | "^" | "%" if values.len() == 2 => {
                self.apply_binary(name, &values[0], &values[1])
            }
            "-" if values.len() == 1 => self.apply_unary_minus(&values[0]),
            _ => self.apply_function(name, &values),
        }
    }

    /// Validates arguments and applies either builtin or user function
    fn apply_function(&mut self, name: &str, args: &[Value]) -> Result<Value, String> {
        if self.func_table.is_builtin(name) {
            self.func_table.validate_args(name, args.len())?;
            return self.apply_builtin(name, args);
        }

        self.apply_user_function(name, args)
    }

    /// Apply a binary operator with type checking
    fn apply_binary(&self, op: &str, left: &Value, right: &Value) -> Result<Value, String> {
        let left_type = left.type_of();
        let right_type = right.type_of();

        // Type checking for operators
        if !matches!(left_type, Type::Integer | Type::Float) {
            return Err(format!(
                "Operator '{}' requires numeric operands, got {} on left",
                op, left_type
            ));
        }
        if !matches!(right_type, Type::Integer | Type::Float) {
            return Err(format!(
                "Operator '{}' requires numeric operands, got {} on right",
                op, right_type
            ));
        }

        let left_f = left.to_float()?;
        let right_f = right.to_float()?;

        let result = match op {
            "+" => left_f + right_f,
            "-" => left_f - right_f,
            "*" => left_f * right_f,
            "/" => {
                if right_f == 0.0 {
                    return Err("Division by zero".to_string());
                }
                left_f / right_f
            }
            "^" => left_f.powf(right_f),
            "%" => left_f.rem_euclid(right_f),
            _ => unreachable!(),
        };

        // Preserve integer type when both operands are integers (except division)
        if left_type == Type::Integer && right_type == Type::Integer && op != "/" {
            Ok(Value::Integer(result as i64))
        } else {
            Ok(Value::Float(result))
        }
    }

    /// Apply unary minus
    fn apply_unary_minus(&self, operand: &Value) -> Result<Value, String> {
        match operand {
            Value::Float(f) => Ok(Value::Float(-f)),
            Value::Integer(i) => Ok(Value::Integer(-i)),
            _ => Err(format!(
                "Unary minus requires numeric operand, got {}",
                operand.type_of()
            )),
        }
    }

    /// Apply a builtin function
    fn apply_builtin(&self, func: &str, args: &[Value]) -> Result<Value, String> {
        match func {
            // Single-argument math functions
            "sqrt" => self.apply_single_arg_math(args[0].to_float()?, f64::sqrt),
            "cbrt" => self.apply_single_arg_math(args[0].to_float()?, f64::cbrt),
            "ln" => self.apply_single_arg_math(args[0].to_float()?, f64::ln),
            "sin" => self.apply_single_arg_math(args[0].to_float()?, f64::sin),
            "cos" => self.apply_single_arg_math(args[0].to_float()?, f64::cos),
            "tan" => self.apply_single_arg_math(args[0].to_float()?, f64::tan),
            "asin" => self.apply_single_arg_math(args[0].to_float()?, f64::asin),
            "acos" => self.apply_single_arg_math(args[0].to_float()?, f64::acos),
            "atan" => self.apply_single_arg_math(args[0].to_float()?, f64::atan),

            // Rounding functions that preserve type
            "round" => self.apply_rounding_math(&args[0], f64::round),
            "floor" => self.apply_rounding_math(&args[0], f64::floor),
            "ceil" => self.apply_rounding_math(&args[0], f64::ceil),

            "abs" => {
                self.require_numeric(&args[0], "abs")?;
                match &args[0] {
                    Value::Integer(i) => Ok(Value::Integer(i.abs())),
                    Value::Float(f) => Ok(Value::Float(f.abs())),
                    _ => unreachable!(),
                }
            }

            "root" => {
                self.require_numeric(&args[0], "root")?;
                self.require_numeric(&args[1], "root")?;
                let base = args[0].to_float()?;
                let n = args[1].to_float()?;
                Ok(Value::Float(base.powf(1.0 / n)))
            }

            "log" => {
                self.require_numeric(&args[0], "log")?;
                self.require_numeric(&args[1], "log")?;
                let base = args[0].to_float()?;
                let val = args[1].to_float()?;
                Ok(Value::Float(val.log(base)))
            }

            "max" => self.apply_minmax(args, true),
            "min" => self.apply_minmax(args, false),

            "println" | "print" => {
                let output = args
                    .iter()
                    .map(|v| v.display())
                    .collect::<Vec<_>>()
                    .join(" ");

                if func == "println" {
                    println!("{}", output);
                } else {
                    // TODO: Output is not written so i changed it to println
                    println!("{}", output);
                }

                Ok(Value::Null)
            }

            _ => Err(format!("Unknown function: '{}'", func)),
        }
    }

    /// Apply a single-argument math function
    fn apply_single_arg_math<F>(&self, val: f64, f: F) -> Result<Value, String>
    where
        F: Fn(f64) -> f64,
    {
        Ok(Value::Float(f(val)))
    }

    /// Apply rounding functions and preserve output type
    fn apply_rounding_math<F>(&self, value: &Value, f: F) -> Result<Value, String>
    where
        F: Fn(f64) -> f64,
    {
        self.require_numeric(value, "rounding function")?;
        let val = value.to_float()?;
        let result = f(val);
        Ok(Value::Integer(result as i64))
    }

    /// Apply min/max function
    fn apply_minmax(&self, args: &[Value], is_max: bool) -> Result<Value, String> {
        // Validate all arguments are numeric
        for (i, arg) in args.iter().enumerate() {
            self.require_numeric(arg, &format!("minmax (argument {})", i + 1))?;
        }

        // Fast path for all integers
        if args.iter().all(|v| matches!(v, Value::Integer(_))) {
            let result = args
                .iter()
                .filter_map(|v| match v {
                    Value::Integer(i) => Some(*i),
                    _ => None,
                })
                .fold(
                    if is_max { i64::MIN } else { i64::MAX },
                    if is_max { i64::max } else { i64::min },
                );
            return Ok(Value::Integer(result));
        }

        // Float path
        let result = args.iter().try_fold(
            if is_max {
                f64::NEG_INFINITY
            } else {
                f64::INFINITY
            },
            |acc, v| {
                v.to_float()
                    .map(|f| if is_max { acc.max(f) } else { acc.min(f) })
            },
        )?;

        Ok(Value::Float(result))
    }

    /// Apply a user-defined function (WIP)
    fn apply_user_function(&mut self, func: &str, args: &[Value]) -> Result<Value, String> {
        self.func_table.validate_args(func, args.len())?;

        let function = self
            .func_table
            .get_function_owned(func)
            .ok_or_else(|| format!("Unknown function: '{}'", func))?;

        let impl_node = function
            .implementation
            .as_ref()
            .ok_or_else(|| format!("Undefined implementation for function: {}", func))?;

        // Create new scope for function execution
        self.scopes.push_scope();
        let result = self.eval(impl_node);
        self.scopes.pop_scope();

        result
    }

    /// Require a value to be numeric
    fn require_numeric(&self, value: &Value, context: &str) -> Result<(), String> {
        match value.type_of() {
            Type::Integer | Type::Float => Ok(()),
            other => Err(format!(
                "{} requires numeric argument, got {}",
                context, other
            )),
        }
    }
}
