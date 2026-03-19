// Copyright (c) 2026 bazelik-null

use crate::morsel_interpreter::environment::types::Type;
use crate::morsel_interpreter::parser::ast_node::Node;
use std::collections::HashMap;

pub struct FunctionTable {
    functions: HashMap<String, FunctionInfo>,
}

/// Metadata about each function
#[derive(Clone, Debug)]
pub struct FunctionInfo {
    pub name: String,
    pub builtin: bool,

    pub min_args: usize,
    pub max_args: Option<usize>, // None = unlimited

    pub parameters: Vec<FunctionParam>, // Store parameter metadata
    pub implementation: Option<Node>,   // None if builtin
}

#[derive(Clone, Debug)]
pub struct FunctionParam {
    pub name: String,
    pub param_type: Type,
}

impl Default for FunctionTable {
    fn default() -> Self {
        Self::new()
    }
}

impl FunctionTable {
    /// Create a new function table with built-in functions
    pub fn new() -> Self {
        let builtin = init_builtins();

        FunctionTable { functions: builtin }
    }

    /// Register a new user-defined function
    pub fn register_function(&mut self, info: FunctionInfo) -> Result<(), String> {
        if self.functions.contains_key(&info.name) {
            if self.functions[&info.name].builtin {
                return Err(format!("Cannot override builtin function '{}'", info.name));
            }
            return Err(format!("Function '{}' is already registered", info.name));
        }

        self.functions.insert(info.name.clone(), info);
        Ok(())
    }

    /// Update an existing function (overwrite)
    pub fn update_function(&mut self, info: FunctionInfo) -> Result<(), String> {
        if !self.functions.contains_key(&info.name) {
            return Err(format!("Function '{}' does not exist", info.name));
        }

        self.functions.insert(info.name.clone(), info);
        Ok(())
    }

    /// Remove a function from the table
    pub fn remove_function(&mut self, name: &str) -> Result<FunctionInfo, String> {
        self.functions
            .remove(name)
            .ok_or_else(|| format!("Function '{}' not found", name))
    }

    /// Get mutable reference to function info (for direct modification)
    pub fn get_function_mut(&mut self, name: &str) -> Option<&mut FunctionInfo> {
        self.functions.get_mut(name)
    }

    /// Clear all user-defined functions (keeps builtins)
    pub fn clear_user_functions(&mut self) {
        self.functions.retain(|_, info| info.builtin);
    }

    /// Check if a function exists
    pub fn is_function(&self, name: &str) -> bool {
        self.functions.contains_key(name)
    }

    /// Check if a function is builtin
    pub fn is_builtin(&self, name: &str) -> bool {
        self.get_function(name).map(|f| f.builtin).unwrap_or(false)
    }

    /// Get function info
    pub fn get_function(&self, name: &str) -> Option<&FunctionInfo> {
        self.functions.get(name)
    }

    /// Get all function names
    pub fn function_names(&self) -> Vec<String> {
        self.functions.keys().cloned().collect()
    }

    /// Lookup a function, returning an error if not found
    pub fn lookup_function(&self, name: &str) -> Result<&FunctionInfo, String> {
        self.get_function(name)
            .ok_or_else(|| format!("Unknown function: '{}'", name))
    }

    /// Validate argument count
    pub fn validate_args(&self, name: &str, arg_count: usize) -> Result<(), String> {
        let info = self.lookup_function(name)?;

        if arg_count < info.min_args {
            return Err(format!(
                "Function '{}' requires at least {} argument(s), got {}",
                name, info.min_args, arg_count
            ));
        }

        if let Some(max) = info.max_args
            && arg_count > max
        {
            return Err(format!(
                "Function '{}' accepts at most {} argument(s), got {}",
                name, max, arg_count
            ));
        }

        Ok(())
    }
}

fn init_builtins() -> HashMap<String, FunctionInfo> {
    [
        // Single-argument functions
        ("sqrt", 1, Some(1)),
        ("cbrt", 1, Some(1)),
        ("ln", 1, Some(1)),
        ("sin", 1, Some(1)),
        ("cos", 1, Some(1)),
        ("tan", 1, Some(1)),
        ("asin", 1, Some(1)),
        ("acos", 1, Some(1)),
        ("atan", 1, Some(1)),
        ("abs", 1, Some(1)),
        ("round", 1, Some(1)),
        ("floor", 1, Some(1)),
        ("ceil", 1, Some(1)),
        // Multi-argument functions
        ("root", 2, Some(2)),
        ("log", 2, Some(2)),
        ("min", 1, None),
        ("max", 1, None),
        // I/O functions
        ("print", 1, None),
        ("println", 1, None),
    ]
    .into_iter()
    .map(|(name, min, max)| {
        (
            name.to_string(),
            FunctionInfo {
                name: name.to_string(),
                builtin: true,
                min_args: min,
                max_args: max,
                implementation: None,
                parameters: Vec::new(),
            },
        )
    })
    .collect()
}
