// Copyright (c) 2026 bazelik-null

use crate::morsel_core::evaluating::variable::{Type, Value};
use crate::morsel_core::lexing::operators::{OperatorType, Precedence};
use crate::morsel_core::lexing::token::{LiteralValue, Token};
use crate::morsel_core::parsing::node::Node;

pub struct Parser {
    tokens: Vec<Token>,
    pos: usize,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Self { tokens, pos: 0 }
    }

    /// Entry point for parsing. Parses all statements and returns a single AST root.
    pub fn parse(&mut self) -> Result<Node, String> {
        let mut statements = Vec::new();

        // Parse all statements
        while self.pos < self.tokens.len() {
            statements.push(self.parse_statement()?);
        }

        // Build a single tree
        Ok(match statements.len() {
            0 => Node::Block(vec![]),
            1 => statements.into_iter().next().unwrap(),
            _ => Node::Block(statements),
        })
    }

    /// Parse a single statement (let binding, assignment, or expression).
    fn parse_statement(&mut self) -> Result<Node, String> {
        let node = if self.peek_keyword("let") {
            self.parse_let_binding()?
        } else if self.is_assignment() {
            self.parse_assignment()?
        } else {
            self.parse_expression()?
        };

        self.consume_semicolon();

        Ok(node)
    }

    /// Parse let binding with optional type annotation and type inference
    fn parse_let_binding(&mut self) -> Result<Node, String> {
        self.advance(); // consume 'let'

        // Check for mutability
        let mutability = if self.peek_keyword("mut") {
            self.advance();
            true
        } else {
            false
        };

        // Get variable name
        let name = match self.peek() {
            Some(Token::Identifier(n)) => {
                let name = n.clone();
                self.advance();
                name
            }
            _ => {
                return Err(format!("Expected identifier after 'let' at {}", self.pos));
            }
        };

        // Try to parse explicit type annotation (only if colon is present)
        let type_annotation = if self.peek_operator() == Some(OperatorType::Colon) {
            self.advance(); // consume ':'

            match self.peek() {
                Some(Token::Type(n)) => {
                    let type_annotation = n.clone().parse::<Type>()?;
                    self.advance();
                    Some(type_annotation)
                }
                _ => {
                    return Err(format!(
                        "Expected type after ':' in 'let {}' at {}",
                        name, self.pos
                    ));
                }
            }
        } else {
            // No colon, so no explicit type annotation
            None
        };

        self.expect_operator(OperatorType::Assign)?;
        let value = Box::new(self.parse_expression()?);

        // If no explicit type, infer from the value
        let final_type_annotation = match type_annotation {
            Some(t) => Some(t),
            None => self.infer_type_from_node(&value)?,
        };

        // Build node
        Ok(Node::Statement {
            name,
            mutability,
            value,
            type_annotation: final_type_annotation,
        })
    }

    /// Parse assignments
    fn parse_assignment(&mut self) -> Result<Node, String> {
        let name = match self.peek() {
            Some(Token::Identifier(n)) => n.clone(),
            _ => {
                return Err(format!(
                    "Expected identifier for assignment at {}",
                    self.pos
                ));
            }
        };

        self.advance();
        self.expect_operator(OperatorType::Assign)?;
        let value = Box::new(self.parse_expression()?);

        // Build node
        Ok(Node::Assignment { name, value })
    }

    /// Check if current position is an assignment (identifier followed by =)
    fn is_assignment(&self) -> bool {
        matches!(self.peek(), Some(Token::Identifier(_)))
            && self.tokens.get(self.pos + 1).and_then(|t| t.as_operator())
                == Some(&OperatorType::Assign)
    }

    /// Parse an expression with operator precedence
    fn parse_expression(&mut self) -> Result<Node, String> {
        self.parse_precedence(Precedence::Additive)
    }

    /// Precedence climbing algorithm for binary operators
    fn parse_precedence(&mut self, min_precedence: Precedence) -> Result<Node, String> {
        // Parse left value
        let mut left = self.parse_primary()?;

        // Parse binary operators as long as they have sufficient precedence
        while let Some(op) = self.peek_operator() {
            // Get the precedence of the current operator, or stop if it's not a binary operator
            let Some(precedence) = op.precedence() else {
                break;
            };

            if precedence < min_precedence {
                break;
            }

            // Consume operator
            self.advance();

            // Determine the minimum precedence for the right operand
            let next_min = if op.is_right_associative() {
                precedence
            } else {
                precedence.next_higher()
            };

            // Parse right operands with calculated minimum precedence
            let right = self.parse_precedence(next_min)?;

            // Build node
            left = Node::Call {
                name: op.to_string(),
                args: vec![left, right],
            };
        }

        Ok(left)
    }

    /// Parse primary expression: function, unary operator, or atom
    fn parse_primary(&mut self) -> Result<Node, String> {
        if let Some(func) = self.peek_function() {
            return self.parse_function(func);
        }

        if let Some(op) = self.peek_operator()
            && op.is_unary()
        {
            return self.parse_unary(op);
        }

        self.parse_atom()
    }

    /// Parse function call: name(arg1, arg2, ...)
    fn parse_function(&mut self, func: String) -> Result<Node, String> {
        // Consume function token
        self.advance();

        // Check for parenthesis and parse arguments inside them
        self.expect_operator(OperatorType::LParen)?;
        let args = self.parse_arguments()?;
        self.expect_operator(OperatorType::RParen)?;

        // Build node
        Ok(Node::Call { name: func, args })
    }

    /// Parse comma-separated argument list
    fn parse_arguments(&mut self) -> Result<Vec<Node>, String> {
        if self.peek_operator() == Some(OperatorType::RParen) {
            return Ok(Vec::new());
        }

        let mut args = vec![self.parse_expression()?];

        while self.peek_operator() == Some(OperatorType::Comma) {
            // Consume comma
            self.advance();

            // Parse argument
            args.push(self.parse_expression()?);
        }

        Ok(args)
    }

    /// Parse unary operator: -x, !x, etc.
    fn parse_unary(&mut self, op: OperatorType) -> Result<Node, String> {
        // Consume unary operator
        self.advance();

        // Parse child
        let child = self.parse_primary()?;

        // Build node
        Ok(Node::Call {
            name: op.to_string(),
            args: vec![child],
        })
    }

    /// Parse atomic expression: number, variable, or parenthesized expression
    fn parse_atom(&mut self) -> Result<Node, String> {
        match self.peek() {
            // Parse literal
            Some(Token::Literal(value)) => {
                let value_node = match value {
                    LiteralValue::Integer(value) => Node::Literal(Value::Integer(*value)),
                    LiteralValue::Float(value) => Node::Literal(Value::Float(*value)),
                    LiteralValue::String(value) => Node::Literal(Value::String(value.clone())),
                    LiteralValue::Boolean(value) => Node::Literal(Value::Boolean(*value)),
                };

                self.advance();
                Ok(value_node)
            }

            // Parse variable reference
            Some(Token::Identifier(name)) => {
                let name = name.clone();
                self.advance();
                Ok(Node::Variable(name))
            }

            // Parse parenthesis
            Some(Token::Operator(OperatorType::LParen)) => {
                // Consume '('
                self.advance();

                // Parse expression inside
                let expr = self.parse_expression()?;
                // Check for ')'
                self.expect_operator(OperatorType::RParen)?;

                Ok(expr)
            }

            _ => self.error_unexpected_token(),
        }
    }

    /// Infer type from a node. Returns error if type cannot be inferred.
    fn infer_type_from_node(&self, node: &Node) -> Result<Option<Type>, String> {
        match node {
            Node::Literal(val) => match val {
                Value::Integer(_) => Ok(Some(Type::Integer)),
                Value::Float(_) => Ok(Some(Type::Float)),
                Value::String(_) => Ok(Some(Type::String)),
                Value::Boolean(_) => Ok(Some(Type::Boolean)),
                Value::Null => Ok(None),
            },
            Node::Variable(_) => Err(format!(
                "Cannot infer type from variable reference. Please provide explicit type annotation at {}.",
                self.pos
            )),
            Node::Call { name, args: _args } => Err(format!(
                "Cannot infer type from expression '{}'. Please provide explicit type annotation at {}.",
                name, self.pos
            )),
            Node::Statement { .. } => Err(format!(
                "Cannot infer type from nested statement. Please provide explicit type annotation at {}.",
                self.pos
            )),
            Node::Assignment { .. } => Err(format!(
                "Cannot infer type from assignment. Please provide explicit type annotation at {}.",
                self.pos
            )),
            Node::Block(_) => Err(format!(
                "Cannot infer type from block. Please provide explicit type annotation at {}.",
                self.pos
            )),
        }
    }

    fn expect_operator(&mut self, expected: OperatorType) -> Result<(), String> {
        match self.peek_operator() {
            Some(op) if op == expected => {
                self.advance();
                Ok(())
            }

            Some(op) => Err(format!(
                "Expected '{}', found '{}' at position {}",
                expected, op, self.pos
            )),

            None => Err(format!("Expected '{}', found end of input", expected)),
        }
    }

    fn consume_semicolon(&mut self) {
        if self.peek_operator() == Some(OperatorType::Semicolon) {
            self.advance();
        }
    }

    fn error_unexpected_token(&self) -> Result<Node, String> {
        match self.peek() {
            Some(token) => Err(format!(
                "Unexpected token at position {}: {:?}",
                self.pos, token
            )),

            None => Err("Unexpected end of input".to_string()),
        }
    }

    fn peek(&self) -> Option<&Token> {
        self.tokens.get(self.pos)
    }

    fn peek_operator(&self) -> Option<OperatorType> {
        self.peek().and_then(|t| t.as_operator().cloned())
    }

    fn peek_function(&self) -> Option<String> {
        self.peek().and_then(|t| t.as_function().cloned())
    }

    fn peek_keyword(&self, keyword: &str) -> bool {
        matches!(self.peek(), Some(Token::Keyword(kw)) if kw == keyword)
    }

    fn advance(&mut self) {
        self.pos += 1;
    }
}
