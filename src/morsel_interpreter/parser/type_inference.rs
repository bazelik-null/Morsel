use crate::morsel_interpreter::environment::types::Type;
use crate::morsel_interpreter::parser::ast_node::Node;
use crate::morsel_interpreter::parser::builder::AstBuilder;

impl AstBuilder {
    /// Infer the type of node by recursively analyzing its structure.
    pub fn infer_type_from_node(&self, node: &Node) -> Result<Type, String> {
        match node {
            Node::Literal(value) => Ok(value.type_of()),

            Node::Reference(name) => self
                .symbol_table
                .variables
                .lookup(name)
                .map(|var| var.type_annotation)
                .ok_or_else(|| format!("Variable '{}' is not defined", name)),

            Node::Call { name, args } => {
                // Check if it's a unary operator
                if args.len() == 1 && self.is_unary_operator(name) {
                    let operand_type = self.infer_type_from_node(&args[0])?;
                    return self.infer_unary_op_type(name, &operand_type);
                }

                // Check if it's a binary operator
                if args.len() == 2 && self.is_binary_operator(name) {
                    let left_type = self.infer_type_from_node(&args[0])?;
                    let right_type = self.infer_type_from_node(&args[1])?;
                    return self.infer_binary_op_type(name, &left_type, &right_type);
                }

                // Treat as function call
                let func = self
                    .symbol_table
                    .functions
                    .lookup(name)
                    .ok_or_else(|| format!("Function '{}' is not defined", name))?;

                Ok(func.return_type)
            }

            Node::Block(statements) => {
                if statements.is_empty() {
                    return Ok(Type::Unit);
                }

                // Find last non-statement expression
                let last_expr = statements
                    .iter()
                    .rev()
                    .find(|stmt| !matches!(stmt, Node::LetBinding { .. } | Node::FuncBinding()));

                match last_expr {
                    Some(expr) => self.infer_type_from_node(expr),
                    None => Ok(Type::Unit),
                }
            }

            // Statements return Unit type
            Node::Assignment { .. } | Node::LetBinding { .. } | Node::FuncBinding() => {
                Ok(Type::Unit)
            }
        }
    }

    /// Infer type of binary operation with type compatibility check.
    fn infer_binary_op_type(
        &self,
        op: &str,
        left_type: &Type,
        right_type: &Type,
    ) -> Result<Type, String> {
        // Require exact type match for binary operations
        if !left_type.is_compatible_with(right_type) {
            return Err(format!(
                "Type mismatch in binary operation '{}': {} and {}",
                op, left_type, right_type
            ));
        }

        match op {
            // Arithmetic operators
            "+" | "-" | "*" | "/" | "%" | "^" => match left_type {
                Type::Integer | Type::Float => Ok(*left_type),
                Type::String if op == "+" => Ok(Type::String), // String concatenation
                _ => Err(format!(
                    "Operator '{}' cannot be applied to type {}",
                    op, left_type
                )),
            },
            // Comparison operators always return boolean
            "==" | "!=" | "<" | "<=" | ">" | ">=" => Ok(Type::Boolean),
            // Logical operators require boolean operands
            "&&" | "||" => {
                if *left_type == Type::Boolean {
                    Ok(Type::Boolean)
                } else {
                    Err(format!(
                        "Operator '{}' requires boolean operands, got {}",
                        op, left_type
                    ))
                }
            }
            _ => Err(format!("Unknown operator: {}", op)),
        }
    }

    /// Infer type of unary operation with type validation.
    fn infer_unary_op_type(&self, op: &str, operand_type: &Type) -> Result<Type, String> {
        match op {
            // Negation requires numeric type
            "-" => match operand_type {
                Type::Integer | Type::Float => Ok(*operand_type),
                _ => Err(format!(
                    "Unary operator '-' cannot be applied to type {}",
                    operand_type
                )),
            },
            // Logical NOT requires boolean type
            "!" => {
                if *operand_type == Type::Boolean {
                    Ok(Type::Boolean)
                } else {
                    Err(format!(
                        "Operator '!' requires boolean operand, got {}",
                        operand_type
                    ))
                }
            }
            _ => Err(format!("Unknown unary operator: {}", op)),
        }
    }

    /// Check if an operator is a binary operator.
    fn is_binary_operator(&self, op: &str) -> bool {
        matches!(
            op,
            "+" | "-" | "*" | "/" | "%" | "^" | "==" | "!=" | "<" | "<=" | ">" | ">=" | "&&" | "||"
        )
    }

    /// Check if an operator is a unary operator.
    fn is_unary_operator(&self, op: &str) -> bool {
        matches!(op, "-" | "!")
    }
}
