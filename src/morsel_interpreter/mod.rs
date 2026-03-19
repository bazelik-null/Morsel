// Copyright (c) 2026 bazelik-null

pub mod environment;
pub mod lexer;
pub mod parser;
pub mod runtime;

//
// Interpreter
//

use crate::morsel_interpreter::environment::functions::FunctionTable;
use crate::morsel_interpreter::lexer::token::Token;
use crate::morsel_interpreter::lexer::tokenizer;
use crate::morsel_interpreter::parser::ast_node::Node;
use crate::morsel_interpreter::parser::builder::AstBuilder;
use crate::morsel_interpreter::runtime::executor::Executor;

pub struct Interpreter {
    func_table: FunctionTable,
    debug: bool,
}

impl Interpreter {
    pub fn new(debug: bool) -> Self {
        Interpreter {
            func_table: FunctionTable::new(),
            debug,
        }
    }

    /// Execute expression
    pub fn execute(&mut self, input: &str) -> Result<(), String> {
        if self.debug {
            println!();
            println!("[DEBUG]: Raw input: \n{}", input);
            println!();
        }

        // Tokenize
        let tokens = self.tokenize(input)?;
        /*
        if self.debug {
            println!("[DEBUG]: Tokens: {:?}", tokens);
        }
         */

        // Parse into AST
        let ast = self.parse(tokens)?;
        if self.debug {
            println!("[DEBUG]: Abstract Syntax Tree:\n {}", ast);
            println!();
        }

        // Evaluate AST
        self.evaluate(&ast)
    }

    /// Tokenize input string into tokens
    fn tokenize(&self, input: &str) -> Result<Vec<Token>, String> {
        tokenizer::tokenize(input)
    }

    /// Parse tokens into an Abstract Syntax Tree
    fn parse(&self, tokens: Vec<Token>) -> Result<Node, String> {
        let mut parser = AstBuilder::new(tokens);
        parser.parse()
    }

    /// Evaluate an AST node
    fn evaluate(&mut self, ast: &Node) -> Result<(), String> {
        let mut evaluator = Executor::new(&mut self.func_table);
        evaluator.execute(ast)?;
        Ok(())
    }

    /// Enable or disable debug output
    pub fn set_debug(&mut self, debug: bool) {
        self.debug = debug;
    }
}
