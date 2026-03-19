// Copyright (c) 2026 bazelik-null

use crate::morsel_interpreter::environment::types::Type;
use crate::morsel_interpreter::environment::variable::Value;
use crate::morsel_interpreter::lexer::syntax_operator::{Precedence, SyntaxOperator};
use crate::morsel_interpreter::lexer::token::{LiteralValue, Token};
use crate::morsel_interpreter::parser::ast_node::Node;

pub struct AstBuilder {
    tokens: Vec<Token>,
    pos: usize,
}

impl AstBuilder {
    pub fn new(tokens: Vec<Token>) -> Self {
        Self { tokens, pos: 0 }
    }

    /// Entry point for parser. Parses all statements and returns a single AST root.
    /// Also parses functions.
    pub fn parse(&mut self) -> Result<Node, String> {
        let mut statements = Vec::new();

        // Parse all statements
        while self.pos < self.tokens.len() && !self.peek_curly() {
            statements.push(self.parse_statement()?);
        }

        // Wrap all statements into a root block
        Ok(Node::Block(statements))
    }

    /// Parse a single statement (let binding, assignment, or expression).
    fn parse_statement(&mut self) -> Result<Node, String> {
        let node = if self.peek_keyword("let") {
            self.parse_let_binding()?
        } else if self.peek_keyword("fn") {
            self.parse_func_binding()?
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

        // Get type (if explicit)
        let type_annotation = self.get_explicit_type(name.as_str())?;

        // Get assigment
        self.expect_operator(SyntaxOperator::Assign)?;
        let value = Box::new(self.parse_expression()?);

        // If no explicit type, infer from the value
        let final_type_annotation = match type_annotation {
            Some(t) => t,
            None => self.infer_type_from_node(&value)?,
        };

        // Build node
        Ok(Node::LetBinding {
            name,
            mutability,
            value,
            type_annotation: final_type_annotation,
        })
    }

    /// Parse function binding
    fn parse_func_binding(&mut self) -> Result<Node, String> {
        self.advance(); // consume 'fn'

        // Get function name
        let name = match self.peek() {
            Some(Token::Identifier(n)) => {
                let name = n.clone();
                self.advance();
                name
            }
            _ => {
                return Err(format!("Expected identifier after 'fn' at {}", self.pos));
            }
        };

        // Check for parenthesis and parse arguments inside them
        self.expect_operator(SyntaxOperator::LParen)?;
        let args = self.parse_func_arguments()?;
        self.expect_operator(SyntaxOperator::RParen)?;

        // Parse all statements inside
        self.expect_operator(SyntaxOperator::CurlyLParen)?;
        let implementation = Box::new(self.parse()?);
        self.expect_operator(SyntaxOperator::CurlyRParen)?;

        // Build node
        Ok(Node::FuncBinding {
            name,
            args,
            implementation,
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
        self.expect_operator(SyntaxOperator::Assign)?;
        let value = Box::new(self.parse_expression()?);

        // Build node
        Ok(Node::Assignment { name, value })
    }

    /// Check if current position is an assignment (identifier followed by =)
    fn is_assignment(&self) -> bool {
        matches!(self.peek(), Some(Token::Identifier(_)))
            && self.tokens.get(self.pos + 1).and_then(|t| t.as_operator())
                == Some(&SyntaxOperator::Assign)
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

    /// Parse primary expression: unary operator or atom
    fn parse_primary(&mut self) -> Result<Node, String> {
        if let Some(op) = self.peek_operator()
            && op.is_unary()
        {
            return self.parse_unary(op);
        }

        self.parse_atom()
    }

    /// Parse comma-separated argument list
    fn parse_arguments(&mut self) -> Result<Vec<Node>, String> {
        if self.peek_operator() == Some(SyntaxOperator::RParen) {
            return Ok(Vec::new());
        }

        let mut args = vec![self.parse_expression()?];

        while self.peek_operator() == Some(SyntaxOperator::Comma) {
            // Consume comma
            self.advance();

            // Parse argument
            args.push(self.parse_expression()?);
        }

        Ok(args)
    }

    /// Parse comma-separated list of function parameters (without let keyword)
    fn parse_func_arguments(&mut self) -> Result<Vec<Node>, String> {
        if self.peek_operator() == Some(SyntaxOperator::RParen) {
            return Ok(Vec::new());
        }

        let mut args = vec![self.parse_func_parameter()?];

        while self.peek_operator() == Some(SyntaxOperator::Comma) {
            // Consume comma
            self.advance();

            // Parse parameter
            args.push(self.parse_func_parameter()?);
        }

        Ok(args)
    }

    /// Parse a single function parameter: 'name: type'
    fn parse_func_parameter(&mut self) -> Result<Node, String> {
        // Get parameter name
        let name = match self.peek() {
            Some(Token::Identifier(n)) => {
                let name = n.clone();
                self.advance();
                name
            }
            _ => {
                return Err(format!("Expected parameter name at position {}", self.pos));
            }
        };

        // Get type annotation (required)
        let type_annotation = match self.get_explicit_type(name.as_str())? {
            Some(t) => t,
            None => Err(format!(
                "Expected parameter type annotation at {}",
                self.pos
            ))?,
        };

        // Build node without initialization
        Ok(Node::LetBinding {
            name,
            mutability: false,
            value: Box::new(Node::Literal(Value::Null)),
            type_annotation,
        })
    }

    /// Parse unary operator: -x, !x, etc.
    fn parse_unary(&mut self, op: SyntaxOperator) -> Result<Node, String> {
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

    /// Parse atomic expression: number, variable, parenthesized expression, function call
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

            // Parse variable reference or function call
            Some(Token::Identifier(name)) => {
                let name = name.clone();
                self.advance();

                // Check if followed by '(' for function call
                if self.peek_operator() == Some(SyntaxOperator::LParen) {
                    self.advance(); // consume '('
                    let args = self.parse_arguments()?;
                    self.expect_operator(SyntaxOperator::RParen)?;
                    Ok(Node::Call { name, args })
                } else {
                    Ok(Node::Reference(name))
                }
            }

            // Parse parenthesis
            Some(Token::SyntaxToken(SyntaxOperator::LParen)) => {
                // Consume '('
                self.advance();

                // Parse expression inside
                let expr = self.parse_expression()?;
                // Check for ')'
                self.expect_operator(SyntaxOperator::RParen)?;

                Ok(expr)
            }

            _ => self.error_unexpected_token(),
        }
    }

    /// Infer type from a node. Returns error if type cannot be inferred.
    fn infer_type_from_node(&self, node: &Node) -> Result<Type, String> {
        match node {
            Node::Literal(val) => match val {
                Value::Integer(_) => Ok(Type::Integer),
                Value::Float(_) => Ok(Type::Float),
                Value::String(_) => Ok(Type::String),
                Value::Boolean(_) => Ok(Type::Boolean),
                Value::Null => Ok(Type::Null),
            },
            Node::Reference(_) => Err(format!(
                "Cannot infer type from variable reference. Please provide explicit type annotation at {}.",
                self.pos
            )),
            Node::Call { name, args: _args } => Err(format!(
                "Cannot infer type from expression '{}'. Please provide explicit type annotation at {}.",
                name, self.pos
            )),
            Node::LetBinding { .. } => Err(format!(
                "Cannot infer type from nested statement. Please provide explicit type annotation at {}.",
                self.pos
            )),
            Node::FuncBinding { .. } => Err(format!(
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

    fn expect_operator(&mut self, expected: SyntaxOperator) -> Result<(), String> {
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

    /// Try to parse explicit type annotation (only if colon is present)
    fn get_explicit_type(&mut self, name: &str) -> Result<Option<Type>, String> {
        if self.peek_operator() == Some(SyntaxOperator::Colon) {
            self.advance(); // consume ':'

            match self.peek() {
                Some(Token::Type(n)) => {
                    let type_annotation = n.clone().parse::<Type>()?;
                    self.advance();
                    Ok(Some(type_annotation))
                }
                _ => Err(format!(
                    "Expected type after ':' in 'let {}' at {}",
                    name, self.pos
                )),
            }
        } else {
            // No colon, so no explicit type annotation
            Ok(None)
        }
    }

    fn consume_semicolon(&mut self) {
        if self.peek_operator() == Some(SyntaxOperator::Semicolon) {
            self.advance();
        }
    }

    /// Returns true if the next operator is curly brace ('}')
    fn peek_curly(&mut self) -> bool {
        matches!(self.peek_operator(), Some(SyntaxOperator::CurlyRParen))
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

    fn peek_operator(&self) -> Option<SyntaxOperator> {
        self.peek().and_then(|t| t.as_operator().cloned())
    }

    fn peek_keyword(&self, keyword: &str) -> bool {
        matches!(self.peek(), Some(Token::Keyword(kw)) if kw == keyword)
    }

    fn advance(&mut self) {
        self.pos += 1;
    }
}
