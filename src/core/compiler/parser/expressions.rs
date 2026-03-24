use crate::core::compiler::parser::Parser;
use crate::core::compiler::parser::tree::Node;
use crate::core::compiler::preprocessor::token::{OperatorValue, SyntaxValue, TokenType};

impl<'a> Parser<'a> {
    pub fn parse_expression(&mut self, min_bp: u8) -> Result<Node, ()> {
        let mut lhs = self.parse_prefix()?;
        lhs = self.parse_postfix(lhs)?;

        loop {
            if self.is_eof() {
                break;
            }

            let token = match self.peek() {
                Some(t) => t,
                None => break,
            };

            // Check for assignment
            if matches!(token.token_type, TokenType::Syntax(SyntaxValue::Assign)) {
                self.advance();

                let rhs = self.parse_expression(1)?;

                lhs = Node::Assignment {
                    target: Box::new(lhs),
                    value: Box::new(rhs),
                };

                continue;
            }

            let op = match token.token_type {
                TokenType::Operator(op) => op,
                _ => break,
            };

            let (lbp, rbp) = Self::get_binding_power(op).ok_or(())?;

            if lbp < min_bp {
                break;
            }

            self.advance();
            let rhs = self.parse_expression(rbp)?;

            lhs = Node::Binary {
                lhs: Box::new(lhs),
                op,
                rhs: Box::new(rhs),
            };
        }

        Ok(lhs)
    }

    fn parse_prefix(&mut self) -> Result<Node, ()> {
        let token = self.peek().cloned().ok_or_else(|| {
            self.error_at_current("Unexpected end of input");
        })?;

        match token.token_type {
            TokenType::Literal(lit) => {
                self.advance();

                Ok(Node::Literal(lit))
            }
            TokenType::Identifier(spur) => {
                self.advance();

                Ok(Node::Identifier(spur))
            }
            TokenType::Operator(op) => match op {
                OperatorValue::Plus | OperatorValue::Minus | OperatorValue::Not => {
                    self.advance();

                    let rhs = self.parse_expression(100)?;

                    Ok(Node::Unary {
                        op,
                        rhs: Box::new(rhs),
                    })
                }
                _ => {
                    self.advance();
                    self.error("Unexpected token", token);

                    Err(())
                }
            },
            TokenType::Syntax(SyntaxValue::LParen) => {
                self.advance();

                let expr = self.parse_expression(0)?;
                self.expect_syntax(SyntaxValue::RParen)?;

                Ok(expr)
            }
            _ => {
                self.advance();
                self.error("Unexpected token", token);

                Err(())
            }
        }
    }

    fn parse_postfix(&mut self, mut lhs: Node) -> Result<Node, ()> {
        loop {
            if self.check_syntax(SyntaxValue::LParen) {
                self.advance();
                let args = self.parse_arguments()?;
                self.expect_syntax(SyntaxValue::RParen)?;

                lhs = Node::FunctionCall {
                    name: Box::new(lhs),
                    args,
                };
            } else {
                break;
            }
        }
        Ok(lhs)
    }

    fn parse_arguments(&mut self) -> Result<Vec<Node>, ()> {
        let mut args = Vec::new();

        while !self.check_syntax(SyntaxValue::RParen) {
            args.push(self.parse_expression(0)?);
            if !self.match_syntax(SyntaxValue::Comma) {
                break;
            }
        }

        Ok(args)
    }
}
