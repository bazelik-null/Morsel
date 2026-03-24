use crate::core::compiler::parser::tree::Type;
use lasso::Spur;
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct Symbol {
    pub name: Spur,
    pub type_annotation: Type,
    pub mutable: bool,
    pub scope_level: usize,
}

#[derive(Clone)]
pub struct Scope {
    pub symbols: HashMap<Spur, Symbol>,
    pub level: usize,
}

impl Scope {
    pub fn new(level: usize) -> Self {
        Self {
            symbols: HashMap::new(),
            level,
        }
    }

    pub fn define(&mut self, symbol: Symbol) {
        self.symbols.insert(symbol.name, symbol);
    }

    pub fn lookup_local(&self, name: Spur) -> Option<Symbol> {
        self.symbols.get(&name).cloned()
    }
}

pub struct ScopeStack {
    scopes: Vec<Scope>,
}

impl ScopeStack {
    pub fn new() -> Self {
        Self {
            scopes: vec![Scope::new(0)],
        }
    }

    pub fn push(&mut self) {
        let level = self.scopes.len();
        self.scopes.push(Scope::new(level));
    }

    pub fn pop(&mut self) {
        if self.scopes.len() > 1 {
            self.scopes.pop();
        }
    }

    pub fn define(&mut self, symbol: Symbol) {
        if let Some(scope) = self.scopes.last_mut() {
            scope.define(symbol);
        }
    }

    pub fn lookup(&self, name: Spur) -> Option<Symbol> {
        // Search from innermost to outermost scope
        for scope in self.scopes.iter().rev() {
            if let Some(symbol) = scope.lookup_local(name) {
                return Some(symbol);
            }
        }
        None
    }

    pub fn current_level(&self) -> usize {
        self.scopes.len().saturating_sub(1)
    }
}

impl Default for ScopeStack {
    fn default() -> Self {
        Self::new()
    }
}
