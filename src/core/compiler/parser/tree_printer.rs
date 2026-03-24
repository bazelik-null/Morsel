use crate::core::compiler::parser::tree::Node;
use crate::core::compiler::preprocessor::token::{LiteralValue, OperatorValue};
use lasso::Rodeo;

pub struct TreePrinter<'a> {
    rodeo: &'a Rodeo,
}

impl<'a> TreePrinter<'a> {
    pub fn new(rodeo: &'a Rodeo) -> Self {
        Self { rodeo }
    }

    pub fn print(&self, nodes: &[Node]) -> String {
        let mut output = String::new();
        for node in nodes {
            self.print_node(node, &mut output, "", true);
        }
        output
    }

    fn print_node(&self, node: &Node, output: &mut String, prefix: &str, is_last: bool) {
        let connector = if is_last { "└── " } else { "├── " };
        output.push_str(prefix);
        output.push_str(connector);

        match node {
            Node::Literal(lit) => {
                output.push_str(&format!("Literal({})\n", self.format_literal(lit)));
            }
            Node::ArrayLiteral(elements) => {
                output.push_str("ArrayLiteral[\n");
                self.print_children(elements, output, prefix, is_last);
                output.push_str(&format!("{}]\n", self.next_prefix(prefix, is_last)));
            }
            Node::Identifier(spur) => {
                output.push_str(&format!("Identifier({})\n", self.rodeo.resolve(spur)));
            }
            Node::Unary { op, rhs } => {
                output.push_str(&format!("Unary({})\n", self.format_op(op)));
                self.print_node(rhs, output, &self.next_prefix(prefix, is_last), true);
            }
            Node::Binary { lhs, op, rhs } => {
                output.push_str(&format!("Binary({})\n", self.format_op(op)));
                let next_prefix = self.next_prefix(prefix, is_last);
                self.print_node(lhs, output, &next_prefix, false);
                self.print_node(rhs, output, &next_prefix, true);
            }
            Node::Assignment { target, value } => {
                output.push_str("Assignment\n");
                let next_prefix = self.next_prefix(prefix, is_last);
                output.push_str(&format!("{}├── target\n", next_prefix));
                self.print_node(target, output, &format!("{}│   ", next_prefix), false);
                output.push_str(&format!("{}└── value\n", next_prefix));
                self.print_node(value, output, &format!("{}    ", next_prefix), true);
            }
            Node::Block(statements) => {
                output.push_str("Block\n");
                self.print_children(statements, output, prefix, is_last);
            }
            Node::If {
                condition,
                then_branch,
                else_branch,
            } => {
                output.push_str("If\n");
                let next_prefix = self.next_prefix(prefix, is_last);
                output.push_str(&format!("{}├── condition\n", next_prefix));
                self.print_node(condition, output, &format!("{}│   ", next_prefix), false);
                output.push_str(&format!("{}├── then\n", next_prefix));
                self.print_node(then_branch, output, &format!("{}│   ", next_prefix), false);
                if let Some(else_b) = else_branch {
                    output.push_str(&format!("{}└── else\n", next_prefix));
                    self.print_node(else_b, output, &format!("{}    ", next_prefix), true);
                }
            }
            Node::While { condition, body } => {
                output.push_str("While\n");
                let next_prefix = self.next_prefix(prefix, is_last);
                output.push_str(&format!("{}├── condition\n", next_prefix));
                self.print_node(condition, output, &format!("{}│   ", next_prefix), false);
                output.push_str(&format!("{}└── body\n", next_prefix));
                self.print_node(body, output, &format!("{}    ", next_prefix), true);
            }
            Node::VariableDecl {
                name,
                mutable,
                type_annotation,
                value,
            } => {
                let mut_str = if *mutable { "mut " } else { "" };
                output.push_str(&format!(
                    "VarDecl({}{}\n",
                    mut_str,
                    self.rodeo.resolve(name)
                ));
                let next_prefix = self.next_prefix(prefix, is_last);
                if let Some(ty) = type_annotation {
                    output.push_str(&format!("{}├── type: {}\n", next_prefix, ty));
                }
                output.push_str(&format!("{}└── init\n", next_prefix));
                self.print_node(value, output, &format!("{}    ", next_prefix), true);
            }
            Node::FunctionDecl {
                name,
                params,
                body,
                return_type,
            } => {
                output.push_str(&format!("FuncDecl({})\n", self.rodeo.resolve(name)));
                let next_prefix = self.next_prefix(prefix, is_last);

                if !params.is_empty() {
                    output.push_str(&format!("{}├── params\n", next_prefix));
                    for (i, param) in params.iter().enumerate() {
                        let is_last_param = i == params.len() - 1;
                        let param_prefix = format!("{}│   ", next_prefix);
                        let param_connector = if is_last_param {
                            "└── "
                        } else {
                            "├── "
                        };
                        output.push_str(&format!(
                            "{}{}{}: {}\n",
                            param_prefix,
                            param_connector,
                            self.rodeo.resolve(&param.name),
                            param.type_annotation
                        ));
                    }
                }

                if let Some(ret_ty) = return_type {
                    output.push_str(&format!("{}├── return: {}\n", next_prefix, ret_ty));
                }
                output.push_str(&format!("{}└── body\n", next_prefix));
                self.print_node(body, output, &format!("{}    ", next_prefix), true);
            }
            Node::FunctionCall { name, args } => {
                output.push_str("FuncCall\n");
                let next_prefix = self.next_prefix(prefix, is_last);
                output.push_str(&format!("{}├── name\n", next_prefix));
                self.print_node(name, output, &format!("{}│   ", next_prefix), false);
                if !args.is_empty() {
                    output.push_str(&format!("{}└── args\n", next_prefix));
                    for (i, arg) in args.iter().enumerate() {
                        let is_last_arg = i == args.len() - 1;
                        self.print_node(arg, output, &format!("{}    ", next_prefix), is_last_arg);
                    }
                }
            }
            Node::ArrayAccess { array, index } => {
                output.push_str("ArrayAccess\n");
                let next_prefix = self.next_prefix(prefix, is_last);
                output.push_str(&format!("{}├── array\n", next_prefix));
                self.print_node(array, output, &format!("{}│   ", next_prefix), false);
                output.push_str(&format!("{}└── index\n", next_prefix));
                self.print_node(index, output, &format!("{}    ", next_prefix), true);
            }
            Node::Return(expr) => {
                if let Some(e) = expr {
                    output.push_str("Return\n");
                    self.print_node(e, output, &self.next_prefix(prefix, is_last), true);
                } else {
                    output.push_str("Return(void)\n");
                }
            }
        }
    }

    fn print_children(&self, children: &[Node], output: &mut String, prefix: &str, is_last: bool) {
        let next_prefix = self.next_prefix(prefix, is_last);
        for (i, child) in children.iter().enumerate() {
            let is_last_child = i == children.len() - 1;
            self.print_node(child, output, &next_prefix, is_last_child);
        }
    }

    fn next_prefix(&self, prefix: &str, is_last: bool) -> String {
        if is_last {
            format!("{}    ", prefix)
        } else {
            format!("{}│   ", prefix)
        }
    }

    fn format_literal(&self, lit: &LiteralValue) -> String {
        match lit {
            LiteralValue::Integer(n) => n.to_string(),
            LiteralValue::Float(f) => f.to_string(),
            LiteralValue::Boolean(b) => b.to_string(),
            LiteralValue::String(s) => format!("\"{}\"", self.rodeo.resolve(s)),
        }
    }

    fn format_op(&self, op: &OperatorValue) -> String {
        format!("{:?}", op)
    }
}
