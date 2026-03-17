// Copyright (c) 2026 bazelik-null

use crate::morsel_core::lexing::operators::OperatorType;
use std::fmt;

#[derive(Debug, Clone)]
pub enum Node {
    Number(f64),
    UnaryExpr {
        op: OperatorType,
        child: Box<Node>,
    },
    FunctionCall {
        func: String,
        args: Vec<Node>,
    },
    BinaryExpr {
        op: OperatorType,
        lvalue: Box<Node>,
        rvalue: Box<Node>,
    },
}

impl fmt::Display for Node {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let ascii_tree = self.tree_string_internal("", true);
        write!(f, "{}", ascii_tree)
    }
}

impl Node {
    fn tree_string_internal(&self, prefix: &str, is_last: bool) -> String {
        let mut result = String::new();

        // Add the current node's connector and content
        let connector = if is_last { "└── " } else { "├── " };
        result.push_str(prefix);
        result.push_str(connector);

        match self {
            Node::Number(n) => {
                result.push_str(&format!("Number({})\n", n));
            }

            Node::UnaryExpr { op, child } => {
                result.push_str(&format!("UnaryExpr({})\n", op));

                let extension = if is_last { "    " } else { "│   " };
                let child_prefix = format!("{}{}", prefix, extension);

                result.push_str(&child.tree_string_internal(&child_prefix, true));
            }

            Node::FunctionCall { func, args } => {
                result.push_str(&format!("FunctionCall({})\n", func));

                let extension = if is_last { "    " } else { "│   " };
                let child_prefix = format!("{}{}", prefix, extension);

                // Display all arguments
                for (i, arg) in args.iter().enumerate() {
                    let is_last_arg = i == args.len() - 1;
                    result.push_str(&arg.tree_string_internal(&child_prefix, is_last_arg));
                }
            }

            Node::BinaryExpr { op, lvalue, rvalue } => {
                result.push_str(&format!("BinaryExpr({})\n", op));

                let extension = if is_last { "    " } else { "│   " };
                let child_prefix = format!("{}{}", prefix, extension);

                // Add left value
                result.push_str(&lvalue.tree_string_internal(&child_prefix, false));

                // Add right value
                result.push_str(&rvalue.tree_string_internal(&child_prefix, true));
            }
        }

        result
    }

    /// Get the string representation of a node type for debugging
    pub fn node_type(&self) -> &'static str {
        match self {
            Node::Number(_) => "Number",
            Node::UnaryExpr { .. } => "UnaryExpr",
            Node::FunctionCall { .. } => "FunctionCall",
            Node::BinaryExpr { .. } => "BinaryExpr",
        }
    }

    /// Check if this node is a atom node (no children)
    pub fn is_atom(&self) -> bool {
        matches!(self, Node::Number(_))
    }

    /// Get all child nodes
    pub fn children(&self) -> Vec<&Node> {
        match self {
            Node::Number(_) => vec![],
            Node::UnaryExpr { child, .. } => vec![child.as_ref()],
            Node::FunctionCall { args, .. } => args.iter().collect(),
            Node::BinaryExpr { lvalue, rvalue, .. } => vec![lvalue.as_ref(), rvalue.as_ref()],
        }
    }

    /// Get mutable references to all child nodes
    pub fn children_mut(&mut self) -> Vec<&mut Node> {
        match self {
            Node::Number(_) => vec![],
            Node::UnaryExpr { child, .. } => vec![child.as_mut()],
            Node::FunctionCall { args, .. } => args.iter_mut().collect(),
            Node::BinaryExpr { lvalue, rvalue, .. } => {
                vec![lvalue.as_mut(), rvalue.as_mut()]
            }
        }
    }

    /// Calculate the depth of the tree
    pub fn depth(&self) -> usize {
        match self {
            Node::Number(_) => 0,
            Node::UnaryExpr { child, .. } => 1 + child.depth(),
            Node::FunctionCall { args, .. } => {
                1 + args.iter().map(|arg| arg.depth()).max().unwrap_or(0)
            }
            Node::BinaryExpr { lvalue, rvalue, .. } => 1 + lvalue.depth().max(rvalue.depth()),
        }
    }

    /// Count the total number of nodes in the tree
    pub fn node_count(&self) -> usize {
        1 + self
            .children()
            .iter()
            .map(|child| child.node_count())
            .sum::<usize>()
    }
}
