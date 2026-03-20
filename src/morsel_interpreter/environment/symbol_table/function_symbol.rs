// Copyright (c) 2026 bazelik-null

use crate::morsel_interpreter::environment::types::Type;
use crate::morsel_interpreter::parser::ast_node::Node;
use std::collections::HashMap;

/// Represents a function parameter with RTTI
#[derive(Clone, Debug)]
pub struct FunctionParamSymbol {
    pub name: String,
    pub type_annotation: Type,
}

impl FunctionParamSymbol {
    pub fn new(name: String, param_type: Type) -> Self {
        FunctionParamSymbol {
            name,
            type_annotation: param_type,
        }
    }
}

/// Represents a function symbol with RTTI
#[derive(Clone, Debug)]
pub struct FunctionSymbol {
    pub name: String,
    pub parameters: Vec<FunctionParamSymbol>,
    pub return_type: Type,
    pub scope_depth: usize,
    pub implementation: Option<Box<Node>>, // None for builtins
    pub is_builtin: bool,
    pub is_variadic: bool, // True if function accepts infinite parameters
}

impl FunctionSymbol {
    pub fn new(
        name: String,
        parameters: Vec<FunctionParamSymbol>,
        return_type: Type,
        scope_depth: usize,
        implementation: Option<Box<Node>>,
    ) -> Self {
        FunctionSymbol {
            name,
            parameters,
            return_type,
            scope_depth,
            implementation,
            is_builtin: false,
            is_variadic: false,
        }
    }

    pub fn builtin(name: String, parameters: Vec<FunctionParamSymbol>, return_type: Type) -> Self {
        FunctionSymbol {
            name,
            parameters,
            return_type,
            scope_depth: 0,
            implementation: None,
            is_builtin: true,
            is_variadic: false,
        }
    }

    pub fn builtin_variadic(
        name: String,
        parameters: Vec<FunctionParamSymbol>,
        return_type: Type,
    ) -> Self {
        FunctionSymbol {
            name,
            parameters,
            return_type,
            scope_depth: 0,
            implementation: None,
            is_builtin: true,
            is_variadic: true,
        }
    }

    /// Get the number of parameters
    pub fn param_count(&self) -> usize {
        self.parameters.len()
    }

    /// Get parameter type by index
    pub fn get_param_type(&self, index: usize) -> Option<Type> {
        self.parameters.get(index).map(|p| p.type_annotation)
    }

    /// Get parameter type by name
    pub fn get_param_type_by_name(&self, name: &str) -> Option<Type> {
        self.parameters
            .iter()
            .find(|p| p.name == name)
            .map(|p| p.type_annotation)
    }
}

/// Global symbol table for functions
#[derive(Clone)]
pub struct FunctionSymbolTable {
    functions: HashMap<String, FunctionSymbol>,
}

const BUILTIN_FUNCTIONS: &[(&str, usize, Type, bool)] = &[
    ("sin", 1, Type::Float, false),
    ("cos", 1, Type::Float, false),
    ("tan", 1, Type::Float, false),
    ("asin", 1, Type::Float, false),
    ("acos", 1, Type::Float, false),
    ("atan", 1, Type::Float, false),
    // Roots
    ("sqrt", 1, Type::Float, false),
    ("cbrt", 1, Type::Float, false),
    ("ln", 1, Type::Float, false),
    // Rounding
    ("round", 1, Type::Float, false),
    ("floor", 1, Type::Float, false),
    ("ceil", 1, Type::Float, false),
    // Misc
    ("abs", 1, Type::Float, false),
    // Multi-arg
    ("root", 2, Type::Float, false),
    ("log", 2, Type::Float, false),
    ("max", 1, Type::Float, true), // Min 1 arg
    ("min", 1, Type::Float, true), // Min 1 arg
    // I/O
    ("println", 0, Type::Null, true), // 0+ args
    ("print", 0, Type::Null, true),   // 0+ args
    // Explicit conversion
    ("to_int", 1, Type::Integer, false),
    ("to_float", 1, Type::Float, false),
    ("to_bool", 1, Type::Boolean, false),
    ("to_string", 1, Type::String, false),
];

impl FunctionSymbolTable {
    pub fn new() -> Self {
        let mut table = FunctionSymbolTable {
            functions: HashMap::new(),
        };

        table.register_builtins();

        table
    }

    /// Define a function
    pub fn define(&mut self, symbol: FunctionSymbol) -> Result<(), String> {
        if self.functions.contains_key(&symbol.name) {
            let existing = &self.functions[&symbol.name];
            if existing.is_builtin {
                return Err(format!(
                    "Cannot override builtin function '{}'",
                    symbol.name
                ));
            }
            return Err(format!(
                "Function '{}' is already defined at scope depth {}",
                symbol.name, existing.scope_depth
            ));
        }

        self.functions.insert(symbol.name.clone(), symbol);
        Ok(())
    }

    /// Lookup a function
    pub fn lookup(&self, name: &str) -> Option<&FunctionSymbol> {
        self.functions.get(name)
    }

    /// Lookup mutable reference
    pub fn lookup_mut(&mut self, name: &str) -> Option<&mut FunctionSymbol> {
        self.functions.get_mut(name)
    }

    /// Check if function exists
    pub fn exists(&self, name: &str) -> bool {
        self.functions.contains_key(name)
    }

    /// Get all function names
    pub fn function_names(&self) -> Vec<String> {
        self.functions.keys().cloned().collect()
    }

    /// Get all functions
    pub fn all_functions(&self) -> Vec<&FunctionSymbol> {
        self.functions.values().collect()
    }

    /// Remove a function
    pub fn remove(&mut self, name: &str) -> Option<FunctionSymbol> {
        self.functions.remove(name)
    }

    /// Clear all user-defined functions (keeps builtins)
    pub fn clear_user_functions(&mut self) {
        self.functions.retain(|_, func| func.is_builtin);
    }

    /// Get function parameter count
    pub fn get_param_count(&self, name: &str) -> Option<usize> {
        self.functions.get(name).map(|f| f.param_count())
    }

    /// Validate function arguments count
    pub fn validate_arg_count(&self, name: &str, arg_count: usize) -> Result<(), String> {
        match self.lookup(name) {
            Some(func) => {
                if func.is_variadic {
                    // For variadic functions, check minimum required args
                    if arg_count < func.param_count() {
                        return Err(format!(
                            "Function '{}' expects at least {} argument(s), got {}",
                            name,
                            func.param_count(),
                            arg_count
                        ));
                    }
                    Ok(())
                } else {
                    // For fixed functions, exact match required
                    if arg_count != func.param_count() {
                        Err(format!(
                            "Function '{}' expects {} argument(s), got {}",
                            name,
                            func.param_count(),
                            arg_count
                        ))
                    } else {
                        Ok(())
                    }
                }
            }
            None => Err(format!("Function '{}' not found", name)),
        }
    }

    /// Validate parameter types
    pub fn validate_param_types(&self, name: &str, arg_types: &[Type]) -> Result<(), String> {
        match self.lookup(name) {
            Some(func) => {
                if func.is_variadic {
                    // For variadic functions, check minimum required args
                    if arg_types.len() < func.param_count() {
                        return Err(format!(
                            "Function '{}' expects at least {} argument(s), got {}",
                            name,
                            func.param_count(),
                            arg_types.len()
                        ));
                    }

                    // Validate types for all arguments against the first parameter type
                    if let Some(first_param_type) = func.get_param_type(0) {
                        for (i, arg_type) in arg_types.iter().enumerate() {
                            if !arg_type.is_compatible_with(&first_param_type) {
                                return Err(format!(
                                    "Function '{}' argument {} expects type {}, got {}",
                                    name, i, first_param_type, arg_type
                                ));
                            }
                        }
                    }
                    Ok(())
                } else {
                    // For fixed functions, exact match required
                    if arg_types.len() != func.param_count() {
                        return Err(format!(
                            "Function '{}' expects {} argument(s), got {}",
                            name,
                            func.param_count(),
                            arg_types.len()
                        ));
                    }

                    for (i, arg_type) in arg_types.iter().enumerate() {
                        if let Some(param_type) = func.get_param_type(i)
                            && !arg_type.is_compatible_with(&param_type)
                        {
                            return Err(format!(
                                "Function '{}' parameter {} expects type {}, got {}",
                                name, i, param_type, arg_type
                            ));
                        }
                    }
                    Ok(())
                }
            }
            None => Err(format!("Function '{}' not found", name)),
        }
    }

    /// Check if function is builtin
    pub fn is_builtin(&self, name: &str) -> bool {
        self.lookup(name)
            .map(|func| func.is_builtin)
            .unwrap_or(false)
    }

    /// Check if function is variadic
    pub fn is_variadic(&self, name: &str) -> bool {
        self.lookup(name)
            .map(|func| func.is_variadic)
            .unwrap_or(false)
    }

    fn register_builtins(&mut self) {
        for &(name, param_count, return_type, is_variadic) in BUILTIN_FUNCTIONS {
            let params = (0..param_count)
                .map(|i| FunctionParamSymbol::new(format!("arg{}", i), Type::Float))
                .collect();

            let mut func = FunctionSymbol::builtin(name.to_string(), params, return_type);
            func.is_variadic = is_variadic;

            let _ = self.define(func);
        }
    }
}

impl Default for FunctionSymbolTable {
    fn default() -> Self {
        Self::new()
    }
}
