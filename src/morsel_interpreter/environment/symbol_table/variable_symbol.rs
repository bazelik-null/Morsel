// Copyright (c) 2026 bazelik-null

use crate::morsel_interpreter::environment::types::Type;
use crate::morsel_interpreter::environment::value::Value;
use std::collections::HashMap;

/// Represents a variable symbol with RTTI
#[derive(Clone, Debug)]
pub struct VariableSymbol {
    pub name: String,
    pub type_annotation: Type,
    pub mutable: bool,
    pub scope_depth: usize,
    pub initialized: bool,
    pub value: Option<Value>,
}

impl VariableSymbol {
    pub fn new(name: String, type_annotation: Type, mutable: bool, scope_depth: usize) -> Self {
        VariableSymbol {
            name,
            type_annotation,
            mutable,
            scope_depth,
            initialized: true,
            value: None,
        }
    }

    pub fn uninitialized(
        name: String,
        type_annotation: Type,
        mutable: bool,
        scope_depth: usize,
    ) -> Self {
        VariableSymbol {
            name,
            type_annotation,
            mutable,
            scope_depth,
            initialized: false,
            value: None,
        }
    }

    /// Set the runtime value
    pub fn set_value(&mut self, val: Value) {
        self.value = Some(val);
        self.initialized = true;
    }

    /// Get the runtime value
    pub fn get_value(&self) -> Option<&Value> {
        self.value.as_ref()
    }

    /// Take ownership of value (for returns)
    pub fn take_value(&mut self) -> Option<Value> {
        self.value.take()
    }
}

/// Scoped symbol table for variables
#[derive(Clone)]
pub struct VariableSymbolTable {
    scopes: Vec<HashMap<String, VariableSymbol>>,
    current_depth: usize,
}

impl VariableSymbolTable {
    pub fn new() -> Self {
        VariableSymbolTable {
            scopes: vec![HashMap::new()],
            current_depth: 0,
        }
    }

    /// Define a variable with initial value
    pub fn define_with_value(
        &mut self,
        mut symbol: VariableSymbol,
        value: Value,
    ) -> Result<(), String> {
        symbol.set_value(value);
        self.define(symbol)
    }

    /// Get a variable's value (searches all scopes from inner to outer)
    pub fn get_value(&self, name: &str) -> Option<Value> {
        self.scopes
            .iter()
            .rev()
            .find_map(|scope| scope.get(name)?.get_value().cloned())
    }

    /// Set a variable's value (searches all scopes from inner to outer)
    pub fn set_value(&mut self, name: &str, value: Value) -> Result<(), String> {
        for scope in self.scopes.iter_mut().rev() {
            if let Some(symbol) = scope.get_mut(name) {
                symbol.set_value(value);
                return Ok(());
            }
        }
        Err(format!("Variable '{}' not found in any scope", name))
    }

    /// Get metadata without value
    pub fn get_metadata(&self, name: &str) -> Option<VariableSymbol> {
        self.scopes
            .iter()
            .rev()
            .find_map(|scope| scope.get(name).cloned())
    }

    /// Check if variable is initialized
    pub fn is_initialized(&self, name: &str) -> bool {
        self.scopes
            .iter()
            .rev()
            .find_map(|scope| scope.get(name).map(|s| s.initialized))
            .unwrap_or(false)
    }

    /// Push a new scope
    pub fn push_scope(&mut self) {
        self.current_depth += 1;
        self.scopes.push(HashMap::new());
    }

    /// Pop the current scope (keeps global scope)
    pub fn pop_scope(&mut self) -> Option<HashMap<String, VariableSymbol>> {
        if self.scopes.len() > 1 {
            self.current_depth -= 1;
            self.scopes.pop()
        } else {
            None
        }
    }

    /// Get current scope depth
    pub fn depth(&self) -> usize {
        self.current_depth
    }

    /// Define a variable in the current scope
    pub fn define(&mut self, symbol: VariableSymbol) -> Result<(), String> {
        let current_scope = self
            .scopes
            .last_mut()
            .ok_or("No active scope".to_string())?;

        if current_scope.contains_key(&symbol.name) {
            return Err(format!(
                "Variable '{}' is already defined in the current scope",
                symbol.name
            ));
        }

        current_scope.insert(symbol.name.clone(), symbol);
        Ok(())
    }

    /// Lookup a variable (searches from innermost to outermost scope)
    pub fn lookup(&self, name: &str) -> Option<&VariableSymbol> {
        self.scopes.iter().rev().find_map(|scope| scope.get(name))
    }

    /// Lookup mutable reference (for updating initialized status)
    pub fn lookup_mut(&mut self, name: &str) -> Option<&mut VariableSymbol> {
        self.scopes
            .iter_mut()
            .rev()
            .find_map(|scope| scope.get_mut(name))
    }

    /// Check if variable exists in any scope
    pub fn exists(&self, name: &str) -> bool {
        self.scopes.iter().any(|scope| scope.contains_key(name))
    }

    /// Check if variable exists in current scope only
    pub fn exists_in_current_scope(&self, name: &str) -> bool {
        self.scopes
            .last()
            .map(|scope| scope.contains_key(name))
            .unwrap_or(false)
    }

    /// Get all variables in current scope
    pub fn current_scope_vars(&self) -> Vec<&VariableSymbol> {
        self.scopes
            .last()
            .map(|scope| scope.values().collect())
            .unwrap_or_default()
    }

    /// Get all variables (all scopes)
    pub fn all_vars(&self) -> Vec<&VariableSymbol> {
        self.scopes
            .iter()
            .flat_map(|scope| scope.values())
            .collect()
    }

    /// Clear all scopes except global
    pub fn clear_local_scopes(&mut self) {
        self.scopes.truncate(1);
        self.current_depth = 0;
    }
}

impl Default for VariableSymbolTable {
    fn default() -> Self {
        Self::new()
    }
}
