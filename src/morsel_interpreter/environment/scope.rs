// Copyright (c) 2026 bazelik-null

use crate::morsel_interpreter::environment::types::Type;
use crate::morsel_interpreter::environment::variable::{Value, Variable};
use std::collections::HashMap;

/// Manages variable scopes
pub struct Scope {
    scopes: Vec<HashMap<String, Variable>>,
}

impl Scope {
    /// Create a new scope stack with a global scope
    pub fn new() -> Self {
        Scope {
            scopes: vec![HashMap::new()],
        }
    }

    /// Push a new scope onto the stack
    pub fn push_scope(&mut self) {
        self.scopes.push(HashMap::new());
    }

    /// Pop the current scope, preserving the global scope
    pub fn pop_scope(&mut self) -> Option<HashMap<String, Variable>> {
        if self.scopes.len() > 1 {
            self.scopes.pop()
        } else {
            None
        }
    }

    /// Get the current scope depth
    pub fn depth(&self) -> usize {
        self.scopes.len()
    }

    /// Set a variable in the current (innermost) scope
    /// This creates a new variable binding in the current scope.
    /// If a variable with the same name exists in an outer scope, it is shadowed.
    pub fn set_local(&mut self, name: String, value: Value, mutable: bool) -> Result<(), String> {
        let value_type = value.type_of();

        self.scopes
            .last_mut()
            .ok_or("No active scope")?
            .insert(name, Variable::new(value, value_type, mutable));

        Ok(())
    }

    /// Update an existing variable (for reassignment)
    /// Searches from innermost to outermost scope.
    /// Returns error if variable is immutable or type doesn't match.
    pub fn set_existing(&mut self, name: &str, value: Value) -> Result<(), String> {
        let value_type = Type::of(&value);

        // Search from innermost to outermost
        for scope in self.scopes.iter_mut().rev() {
            if let Some(var) = scope.get_mut(name) {
                // Check if variable is mutable
                if !var.mutable {
                    return Err(format!("Cannot reassign immutable variable: '{}'", name));
                }

                // Check type compatibility
                if !var.var_type.is_compatible_with(&value_type) {
                    return Err(format!(
                        "Type mismatch for variable '{}': expected {}, got {}",
                        name, var.var_type, value_type
                    ));
                }

                var.value = var.value.implicit_conversion(value, var.var_type)?;
                return Ok(());
            }
        }

        // Variable not found
        Err(format!("Undefined variable: '{}'", name))
    }

    /// Get a variable value, searching from innermost to outermost scope
    pub fn get(&self, name: &str) -> Option<Value> {
        self.scopes
            .iter()
            .rev()
            .find_map(|scope| scope.get(name).map(|var| var.value.clone()))
    }

    /// Get a reference to a variable value without cloning
    pub fn get_ref(&self, name: &str) -> Option<&Value> {
        self.scopes
            .iter()
            .rev()
            .find_map(|scope| scope.get(name).map(|var| &var.value))
    }

    /// Get the type of variable
    pub fn get_type(&self, name: &str) -> Option<Type> {
        self.scopes
            .iter()
            .rev()
            .find_map(|scope| scope.get(name).map(|var| var.var_type))
    }

    /// Check if a variable is mutable
    pub fn is_mutable(&self, name: &str) -> Option<bool> {
        self.scopes
            .iter()
            .rev()
            .find_map(|scope| scope.get(name).map(|var| var.mutable))
    }

    /// Check if a variable exists in any scope
    pub fn exists(&self, name: &str) -> bool {
        self.scopes.iter().any(|scope| scope.contains_key(name))
    }

    /// Clear all scopes except global
    pub fn clear_local_scopes(&mut self) {
        self.scopes.truncate(1);
    }

    /// Get the number of variables in the current scope
    pub fn current_scope_size(&self) -> usize {
        self.scopes.last().map(|s| s.len()).unwrap_or(0)
    }

    /// Get all variables in the current scope (for debugging)
    pub fn current_scope_vars(&self) -> Vec<String> {
        self.scopes
            .last()
            .map(|s| s.keys().cloned().collect())
            .unwrap_or_default()
    }
}

impl Default for Scope {
    fn default() -> Self {
        Self::new()
    }
}
