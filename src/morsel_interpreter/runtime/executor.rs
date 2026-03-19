// Copyright (c) 2026 bazelik-null

use crate::morsel_interpreter::environment::functions::{
    FunctionInfo, FunctionParam, FunctionTable,
};
use crate::morsel_interpreter::environment::scope::Scope;
use crate::morsel_interpreter::environment::types::Type;
use crate::morsel_interpreter::environment::variable::Value;
use crate::morsel_interpreter::parser::ast_node::Node;

/// Executor which evaluates AST
pub struct Executor<'a> {
    func_table: &'a mut FunctionTable,
    scopes: Scope,
}

impl<'a> Executor<'a> {
    /// Create a new executor with a function table
    pub fn new(func_table: &'a mut FunctionTable) -> Self {
        Executor {
            func_table,
            scopes: Scope::new(),
        }
    }

    /// Main entry point. Executes the program by calling main()
    pub fn execute(&mut self, node: &Node) -> Result<(), String> {
        // Extract statements from the block
        let statements = match node {
            Node::Block(stmts) => stmts,
            _ => return Err("Program must be a block".to_string()),
        };

        // Register all functions
        for statement in statements {
            if let Node::FuncBinding {
                name,
                args,
                implementation,
            } = statement
            {
                self.eval_func_binding(name.as_str(), args, implementation)?;
            }
        }

        if !self.func_table.is_function("main") {
            return Err("No main function defined".to_string());
        }

        // Call main()
        self.evaluate_function("main", &[])?;

        Ok(())
    }

    /// Evaluate an AST node
    fn eval(&mut self, node: &Node) -> Result<Value, String> {
        match node {
            Node::Literal(value) => Ok(value.clone()),
            Node::Reference(name) => self.get_variable(name),
            Node::Block(statements) => self.eval_block(statements),
            Node::LetBinding {
                name,
                mutability,
                value,
                type_annotation,
            } => self.eval_let(name, *mutability, value, type_annotation),
            Node::Assignment { name, value } => self.eval_assignment(name, value),
            Node::Call { name, args } => self.eval_call(name, args),
            Node::FuncBinding {
                name,
                args,
                implementation,
            } => self.eval_func_binding(name, args, implementation),
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
        type_annotation: &Type,
    ) -> Result<Value, String> {
        let result = self.eval(value)?;

        // Check type compatibility
        let actual_type = &result.type_of();
        if !type_annotation.is_compatible_with(actual_type) {
            return Err(format!(
                "Type annotation mismatch for '{}': expected {}, got {}",
                name, type_annotation, actual_type
            ));
        }

        // Convert if needed using implicit conversion
        let converted_value = result.implicit_conversion(result.clone(), *type_annotation)?;

        self.scopes
            .set_local(name.to_string(), converted_value.clone(), mutable)?;
        Ok(converted_value)
    }

    /// Evaluate a function binding and register it
    fn eval_func_binding(
        &mut self,
        name: &str,
        args: &[Node],
        implementation: &Node,
    ) -> Result<Value, String> {
        // Extract parameter information from let binding nodes
        let mut parameters = Vec::new();
        for arg in args {
            if let Node::LetBinding {
                name: param_name,
                type_annotation,
                ..
            } = arg
            {
                parameters.push(FunctionParam {
                    name: param_name.clone(),
                    param_type: *type_annotation,
                });
            }
        }

        // Create function info
        let func_info = FunctionInfo {
            name: name.to_string(),
            builtin: false,
            min_args: parameters.len(),
            max_args: Some(parameters.len()),
            parameters,
            implementation: Some(implementation.clone()),
        };

        // Register the function
        self.func_table.register_function(func_info)?;

        Ok(Value::Null)
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

        self.dispatch_call(name, &values)
    }

    /// Dispatch function calls and operators
    fn dispatch_call(&mut self, name: &str, values: &[Value]) -> Result<Value, String> {
        match name {
            // Binary operators
            "+" | "-" | "*" | "/" | "^" | "%" if values.len() == 2 => {
                self.apply_binary(name, &values[0], &values[1])
            }
            // Unary operators
            "-" if values.len() == 1 => self.apply_unary_minus(&values[0]),
            // Everything else (function calls)
            _ => self.apply_function(name, values),
        }
    }

    /// Validates arguments and applies either builtin or user function
    fn apply_function(&mut self, name: &str, args: &[Value]) -> Result<Value, String> {
        if self.func_table.is_builtin(name) {
            self.func_table.validate_args(name, args.len())?;
            return self.evaluate_builtin_function(name, args);
        }

        self.evaluate_function(name, args)
    }

    /// Validate that both operands are numeric
    fn validate_numeric_operands(
        &self,
        op: &str,
        left: &Value,
        right: &Value,
    ) -> Result<(), String> {
        let left_type = left.type_of();
        let right_type = right.type_of();

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
        Ok(())
    }

    /// Apply a binary operator
    fn apply_binary(&self, op: &str, left: &Value, right: &Value) -> Result<Value, String> {
        self.validate_numeric_operands(op, left, right)?;

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

        // Preserve integer type when both operands are integers
        if left.type_of() == Type::Integer && right.type_of() == Type::Integer {
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

    /// Evaluates a builtin function
    fn evaluate_builtin_function(&self, func: &str, args: &[Value]) -> Result<Value, String> {
        let result = match func {
            // Trigonometric functions
            "sin" => args[0].to_float()?.sin(),
            "cos" => args[0].to_float()?.cos(),
            "tan" => args[0].to_float()?.tan(),
            "asin" => args[0].to_float()?.asin(),
            "acos" => args[0].to_float()?.acos(),
            "atan" => args[0].to_float()?.atan(),

            // Single-argument math functions
            "sqrt" => args[0].to_float()?.sqrt(),
            "cbrt" => args[0].to_float()?.cbrt(),
            "ln" => args[0].to_float()?.ln(),

            // Rounding functions
            "round" => args[0].to_float()?.round(),
            "floor" => args[0].to_float()?.floor(),
            "ceil" => args[0].to_float()?.ceil(),

            "abs" => args[0].to_float()?.abs(),

            "root" => {
                self.require_numeric(&args[0], "root")?;
                self.require_numeric(&args[1], "root")?;
                args[0].to_float()?.powf(1.0 / args[1].to_float()?)
            }

            "log" => {
                self.require_numeric(&args[0], "log")?;
                self.require_numeric(&args[1], "log")?;
                args[1].to_float()?.log(args[0].to_float()?)
            }

            "max" => args
                .iter()
                .try_fold(f64::NEG_INFINITY, |acc, v| v.to_float().map(|f| acc.max(f)))?,

            "min" => args
                .iter()
                .try_fold(f64::INFINITY, |acc, v| v.to_float().map(|f| acc.min(f)))?,

            "println" | "print" => {
                let output = args
                    .iter()
                    .map(|v| v.display())
                    .collect::<Vec<_>>()
                    .join(" ");

                if func == "println" {
                    println!("{}", output);
                } else {
                    print!("{}", output);
                }

                return Ok(Value::Null);
            }

            _ => return Err(format!("Unknown function: '{}'", func)),
        };

        // Will be implicitly converted if needed
        Ok(Value::Float(result))
    }

    /// Evaluate a user-defined function
    /// Creates a new scope, binds parameters to arguments with type checking and implicit conversion, evaluates the function implementation, and returns the result.
    fn evaluate_function(&mut self, func: &str, args: &[Value]) -> Result<Value, String> {
        self.func_table.validate_args(func, args.len())?;

        // Get function information and implementation
        let (impl_node, parameters) = {
            let function = self.func_table.lookup_function(func)?;

            let impl_node = function
                .implementation
                .as_ref()
                .ok_or_else(|| format!("Undefined implementation for function: {}", func))?
                .clone();

            let parameters = function.parameters.clone();

            (impl_node, parameters)
        };

        // Create new scope for function execution
        self.scopes.push_scope();

        // Bind parameters to arguments with type checking and implicit conversion
        for (param, arg_value) in parameters.iter().zip(args.iter()) {
            let arg_type = arg_value.type_of();

            // Type checking
            let converted_value = if arg_type == param.param_type {
                arg_value.clone()
            } else {
                // Attempt implicit conversion
                arg_value.implicit_conversion(arg_value.clone(), param.param_type)?
            };

            self.scopes
                .set_local(param.name.clone(), converted_value, false)?;
        }

        // Evaluate function
        let result = self.eval(&impl_node);

        // Return result
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
