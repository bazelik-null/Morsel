// Copyright (c) 2026 bazelik-null

use crate::morsel_core::evaluating::variable::{Type, Value};
use std::collections::HashMap;

#[derive(Clone)]
pub struct Variable {
    pub value: Value,
    pub var_type: Type,
    pub mutable: bool,
}

impl Variable {
    pub fn new(value: Value, var_type: Type, mutable: bool) -> Self {
        Variable {
            value,
            var_type,
            mutable,
        }
    }
}

/// Manages variable scopes
pub struct ScopeStack {
    scopes: Vec<HashMap<String, Variable>>,
}

impl ScopeStack {
    /// Create a new scope stack with a global scope
    pub fn new() -> Self {
        ScopeStack {
            scopes: vec![HashMap::new()],
        }
    }

    /// Push a new scope onto the stack
    pub fn push_scope(&mut self) {
        self.scopes.push(HashMap::new());
    }

    /// Pop the current scope
    pub fn pop_scope(&mut self) -> Option<HashMap<String, Variable>> {
        if self.scopes.len() > 1 {
            Some(self.scopes.pop().unwrap())
        } else {
            None
        }
    }

    /// Get the current scope depth
    pub fn depth(&self) -> usize {
        self.scopes.len()
    }

    /// Set a variable in the current (innermost) scope
    pub fn set_local(&mut self, name: String, value: Value, mutable: bool) -> Result<(), String> {
        let value_type = value.type_of();

        if let Some(scope) = self.scopes.last_mut() {
            scope.insert(name, Variable::new(value, value_type, mutable));
            Ok(())
        } else {
            Err("No active scope".to_string())
        }
    }

    /// Update an existing variable (for reassignment)
    /// Returns error if variable is immutable
    pub fn set_existing(&mut self, name: &str, value: Value) -> Result<(), String> {
        let value_type = value.type_of();

        // Search from innermost to outermost
        for scope in self.scopes.iter_mut().rev() {
            if let Some(var) = scope.get_mut(name) {
                // Check if variable is mutable
                if !var.mutable {
                    return Err(format!("Cannot reassign immutable variable: '{}'", name));
                }

                // Type must match exactly for strict checking
                if var.var_type != value_type {
                    return Err(format!(
                        "Type mismatch for variable '{}': expected {}, got {}",
                        name, var.var_type, value_type
                    ));
                }

                var.value = value;
                return Ok(());
            }
        }

        // Variable not found
        Err(format!("Undefined variable: '{}'", name))
    }

    /// Set or update a variable (for initial binding only)
    pub fn set_or_update(
        &mut self,
        name: String,
        value: Value,
        mutable: bool,
    ) -> Result<(), String> {
        let value_type = value.type_of();

        // Search from innermost to outermost
        for scope in self.scopes.iter_mut().rev() {
            if let Some(var) = scope.get_mut(&name) {
                // Type must match exactly for strict checking
                if var.var_type != value_type {
                    return Err(format!(
                        "Type mismatch for variable '{}': expected {}, got {}",
                        name, var.var_type, value_type
                    ));
                }

                // Update mutability and value
                var.value = value;
                var.mutable = mutable;
                return Ok(());
            }
        }

        // If not found, set in current scope
        self.set_local(name, value, mutable)
    }

    /// Get a variable value with type information
    pub fn get(&self, name: &str) -> Option<Value> {
        for scope in self.scopes.iter().rev() {
            if let Some(var) = scope.get(name) {
                return Some(var.value.clone());
            }
        }
        None
    }

    /// Get the type of a variable
    pub fn get_type(&self, name: &str) -> Option<Type> {
        for scope in self.scopes.iter().rev() {
            if let Some(var) = scope.get(name) {
                return Some(var.var_type);
            }
        }
        None
    }

    /// Check if a variable is mutable
    pub fn is_mutable(&self, name: &str) -> Option<bool> {
        for scope in self.scopes.iter().rev() {
            if let Some(var) = scope.get(name) {
                return Some(var.mutable);
            }
        }
        None
    }

    /// Get a reference to a variable
    pub fn get_ref(&self, name: &str) -> Option<&Value> {
        for scope in self.scopes.iter().rev() {
            if let Some(var) = scope.get(name) {
                return Some(&var.value);
            }
        }
        None
    }

    /// Check if a variable exists
    pub fn exists(&self, name: &str) -> bool {
        self.scopes.iter().any(|scope| scope.contains_key(name))
    }

    /// Clear all scopes except global
    pub fn clear_local_scopes(&mut self) {
        self.scopes.truncate(1);
    }
}

impl Default for ScopeStack {
    fn default() -> Self {
        Self::new()
    }
}
