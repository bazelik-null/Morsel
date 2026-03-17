// Copyright (c) 2026 bazelik-null

use std::collections::HashMap;

pub struct FunctionTable {
    functions: HashMap<String, FunctionInfo>,
}

/// Metadata about each function
#[derive(Clone, Debug)]
pub struct FunctionInfo {
    pub name: String,
    pub min_args: usize,
    pub max_args: Option<usize>, // None = unlimited
}

impl Default for FunctionTable {
    fn default() -> Self {
        Self::new()
    }
}

impl FunctionTable {
    /// Create a new function table with all supported functions
    pub fn new() -> Self {
        let functions = [
            // Single-argument functions
            ("sqrt", 1, Some(1)),
            ("ln", 1, Some(1)),
            ("sin", 1, Some(1)),
            ("cos", 1, Some(1)),
            ("tan", 1, Some(1)),
            ("asin", 1, Some(1)),
            ("acos", 1, Some(1)),
            ("atan", 1, Some(1)),
            ("abs", 1, Some(1)),
            ("round", 1, Some(1)),
            // Multi-argument functions
            ("log", 2, Some(2)),
            ("min", 1, None),
            ("max", 1, None),
        ]
        .into_iter()
        .map(|(name, min, max)| {
            (
                name.to_string(),
                FunctionInfo {
                    name: name.to_string(),
                    min_args: min,
                    max_args: max,
                },
            )
        })
        .collect();

        FunctionTable { functions }
    }

    /// Check if a function exists
    pub fn is_function(&self, name: &str) -> bool {
        self.functions.contains_key(&name.to_lowercase())
    }

    /// Get function info
    pub fn get_function(&self, name: &str) -> Option<&FunctionInfo> {
        self.functions.get(&name.to_lowercase())
    }

    /// Get all function names
    pub fn function_names(&self) -> Vec<String> {
        self.functions.keys().cloned().collect()
    }

    /// Validate argument count
    pub fn validate_args(&self, name: &str, arg_count: usize) -> Result<(), String> {
        match self.get_function(name) {
            Some(info) => {
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
            None => Err(format!("Unknown function: '{}'", name)),
        }
    }
}
