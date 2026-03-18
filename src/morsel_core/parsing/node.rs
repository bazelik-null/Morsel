// Copyright (c) 2026 bazelik-null

use crate::morsel_core::evaluating::variable::{Type, Value};
use std::fmt;

#[derive(Debug, Clone)]
pub enum Node {
    // Literals
    Literal(Value),
    // Variable references
    Variable(String),

    // Operations (unary, binary, functions)
    Call {
        name: String,
        args: Vec<Node>, // [left, right] for binary, [child] for unary
    },

    // Statements (let)
    Statement {
        name: String,
        mutability: bool,
        value: Box<Node>,
        type_annotation: Option<Type>,
    },

    // Assignment (x = y)
    Assignment {
        name: String,
        value: Box<Node>,
    },

    // Blocks
    Block(Vec<Node>),
}

impl fmt::Display for Node {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.tree_string())
    }
}

impl Node {
    /// Get the inferred type of this node
    fn inferred_type(&self) -> Type {
        match self {
            Node::Literal(value) => value_to_type(value),
            Node::Variable(_) => Type::Any,
            Node::Block(_) => Type::Null,
            Node::Statement {
                type_annotation,
                value,
                ..
            } => type_annotation.unwrap_or_else(|| value.inferred_type()),
            Node::Assignment { value, .. } => value.inferred_type(),
            Node::Call { .. } => Type::Any,
        }
    }

    /// Get the string representation of a node type for debugging
    pub fn node_type(&self) -> &'static str {
        match self {
            Node::Block(_) => "Block",
            Node::Literal(_) => "Literal",
            Node::Variable(_) => "Variable",
            Node::Statement { .. } => "Let",
            Node::Assignment { .. } => "Assign",
            Node::Call { .. } => "Call",
        }
    }

    /// Check if this node is an atom node
    pub fn is_atom(&self) -> bool {
        matches!(self, Node::Literal(_) | Node::Variable(_))
    }

    /// Get all child nodes
    pub fn children(&self) -> Vec<&Node> {
        match self {
            Node::Block(statements) => statements.iter().collect(),
            Node::Literal(_) | Node::Variable(_) => vec![],
            Node::Statement { value, .. } => vec![value.as_ref()],
            Node::Assignment { value, .. } => vec![value.as_ref()],
            Node::Call { args, .. } => args.iter().collect(),
        }
    }

    /// Get mutable references to all child nodes
    pub fn children_mut(&mut self) -> Vec<&mut Node> {
        match self {
            Node::Block(statements) => statements.iter_mut().collect(),
            Node::Literal(_) | Node::Variable(_) => vec![],
            Node::Statement { value, .. } => vec![value.as_mut()],
            Node::Assignment { value, .. } => vec![value.as_mut()],
            Node::Call { args, .. } => args.iter_mut().collect(),
        }
    }

    /// Calculate the depth of the tree
    pub fn depth(&self) -> usize {
        self.children()
            .iter()
            .map(|child| child.depth())
            .max()
            .unwrap_or(0)
            + (if self.is_atom() { 0 } else { 1 })
    }

    /// Count the total number of nodes in the tree
    pub fn node_count(&self) -> usize {
        1 + self
            .children()
            .iter()
            .map(|child| child.node_count())
            .sum::<usize>()
    }

    /// Get a human-readable tree representation
    pub fn tree_string(&self) -> String {
        self.format_tree("", true)
    }

    /// Get type information for a node
    pub fn type_info(&self) -> TypeInfo {
        TypeInfo {
            node_type: self.node_type().to_string(),
            inferred_type: self.inferred_type(),
            is_leaf: self.is_atom(),
            child_count: self.children().len(),
        }
    }

    /// Format a single node line with type annotation
    fn format_node_line(&self) -> String {
        match self {
            Node::Block(_) => {
                format!("Block [{}]", self.inferred_type())
            }
            Node::Literal(value) => {
                format!("Literal({}) [{}]", value, self.inferred_type())
            }
            Node::Variable(name) => {
                format!("Variable(\"{}\") [{}]", name, self.inferred_type())
            }
            Node::Statement {
                name,
                mutability,
                type_annotation,
                ..
            } => {
                let type_display = match type_annotation {
                    Some(ty) => {
                        if *mutability {
                            format!("[mut {}]", ty)
                        } else {
                            format!("[{}]", ty)
                        }
                    }
                    None => {
                        if *mutability {
                            "[mut inferred]".to_string()
                        } else {
                            "[inferred]".to_string()
                        }
                    }
                };
                format!("Let(\"{}\") {}", name, type_display)
            }
            Node::Assignment { name, value } => {
                format!("Assign(\"{}\") [{}]", name, value.inferred_type())
            }
            Node::Call { name, .. } => {
                format!("Call(\"{}\") [{}]", name, self.inferred_type())
            }
        }
    }

    /// Recursively format the tree with proper indentation
    fn format_tree(&self, prefix: &str, is_last: bool) -> String {
        let connector = if is_last { "└── " } else { "├── " };
        let mut result = format!("{}{}{}\n", prefix, connector, self.format_node_line());

        let children = self.children();
        if !children.is_empty() {
            let extension = if is_last { "    " } else { "│   " };
            let child_prefix = format!("{}{}", prefix, extension);

            for (i, child) in children.iter().enumerate() {
                result.push_str(&child.format_tree(&child_prefix, i == children.len() - 1));
            }
        }

        result
    }
}

/// Struct for detailed type information
#[derive(Debug, Clone)]
pub struct TypeInfo {
    pub node_type: String,
    pub inferred_type: Type,
    pub is_leaf: bool,
    pub child_count: usize,
}

impl fmt::Display for TypeInfo {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{} [{}] (leaf: {}, children: {})",
            self.node_type, self.inferred_type, self.is_leaf, self.child_count
        )
    }
}

/// Convert a Value to its corresponding Type
fn value_to_type(value: &Value) -> Type {
    match value {
        Value::Integer(_) => Type::Integer,
        Value::Float(_) => Type::Float,
        Value::String(_) => Type::String,
        Value::Boolean(_) => Type::Boolean,
        Value::Null => Type::Null,
    }
}
