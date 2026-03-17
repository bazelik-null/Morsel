// Copyright (c) 2026 bazelik-null

use crate::morsel_core::lexing::operators::{OperatorType, Precedence};
use crate::morsel_core::lexing::token::Token;
use crate::morsel_core::parsing::node::Node;

pub struct Parser {
    tokens: Vec<Token>,
    pos: usize,
}

/* Parser flow:
 * parse()                 [Entry point]
 * * parse_precedence()    [Handles binary operators with precedence]
 * * * parse_primary()     [Handles unary/function/atoms]
 * * * * parse_function()  [Function calls like cos(x) or max(x, y, z)]
 * * * * parse_unary()     [Unary operators like -x]
 * * * * parse_atom()      [Numbers and parentheses]
 * * * * parse_arguments() [Comma-separated argument lists]
 */
impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Self { tokens, pos: 0 }
    }

    /// Entry point for parsing. Initiates parsing from the lowest precedence level.
    pub fn parse(&mut self) -> Result<Node, String> {
        let expr = self.parse_precedence(Precedence::Additive)?;
        if self.pos < self.tokens.len() {
            return Err(format!("Unexpected token at {}", self.pos));
        }

        Ok(expr)
    }

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

            // Build binary expression node
            left = Node::BinaryExpr {
                op,
                lvalue: Box::new(left),
                rvalue: Box::new(right),
            };
        }

        Ok(left)
    }

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

    fn parse_function(&mut self, func: String) -> Result<Node, String> {
        // Consume function token
        self.advance();

        // Check for parenthesis and parse arguments inside them
        self.expect_operator(OperatorType::LParen)?;
        let args = self.parse_arguments()?;
        self.expect_operator(OperatorType::RParen)?;

        // Build function call node with multiple arguments
        Ok(Node::FunctionCall { func, args })
    }

    /// Parse comma-separated arguments. Returns at least one argument.
    fn parse_arguments(&mut self) -> Result<Vec<Node>, String> {
        let mut args = Vec::new();

        // Handle empty argument list (for functions that take no arguments)
        if self.peek_operator() == Some(OperatorType::RParen) {
            return Ok(args);
        }

        // Parse first argument
        args.push(self.parse_precedence(Precedence::Additive)?);

        // Parse remaining arguments separated by commas
        while self.peek_operator() == Some(OperatorType::Comma) {
            // Consume comma
            self.advance();

            // Parse next argument
            args.push(self.parse_precedence(Precedence::Additive)?);
        }

        Ok(args)
    }

    fn parse_unary(&mut self, op: OperatorType) -> Result<Node, String> {
        // Consume unary operator
        self.advance();

        // Parse child
        let child = self.parse_primary()?;

        // Build unary expression node
        Ok(Node::UnaryExpr {
            op,
            child: Box::new(child),
        })
    }

    fn parse_atom(&mut self) -> Result<Node, String> {
        match self.peek() {
            // Parse number
            Some(Token::Number(value)) => {
                let value = *value;
                self.advance();

                Ok(Node::Number(value))
            }

            // Parse parenthesis
            Some(Token::Operator(OperatorType::LParen)) => {
                // Consume left opening bracket, parse operands inside and check for closing bracket
                self.advance();
                let expr = self.parse_precedence(Precedence::Additive)?;
                self.expect_operator(OperatorType::RParen)?;

                Ok(expr)
            }

            Some(Token::Operator(op)) => Err(format!(
                "Unexpected operator '{}' in primary expression at token {}",
                op, self.pos
            )),
            Some(Token::Function(func)) => Err(format!(
                "Unexpected function '{}' in primary expression at token {}",
                func, self.pos
            )),
            None => Err("Unexpected end of input".to_string()),
        }
    }

    fn expect_operator(&mut self, expected: OperatorType) -> Result<(), String> {
        match self.peek_operator() {
            Some(op) if op == expected => {
                self.advance();

                Ok(())
            }
            Some(op) => Err(format!(
                "Expected '{}', found '{}' at token {} (token: {:?})",
                expected,
                op,
                self.pos,
                self.peek()
            )),
            None => Err(format!(
                "Expected '{}', found end of input at token {}",
                expected, self.pos
            )),
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

    fn advance(&mut self) {
        self.pos += 1;
    }
}
