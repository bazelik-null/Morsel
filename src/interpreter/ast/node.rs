// Copyright (c) 2026 bazelik-null

use crate::interpreter::operators::OperatorType;
use std::fmt;

#[derive(Debug)]
pub enum Node {
    Number(f64),
    UnaryExpr {
        op: OperatorType,
        child: Box<Node>,
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

                // Calculate the prefix for the child
                let extension = if is_last { "    " } else { "│   " };
                let child_prefix = format!("{}{}", prefix, extension);

                result.push_str(&child.tree_string_internal(&child_prefix, true));
            }

            Node::BinaryExpr { op, lvalue, rvalue } => {
                result.push_str(&format!("BinaryExpr({})\n", op));

                // Calculate the prefix for children
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
}
