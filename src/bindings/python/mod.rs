// src/bindings/python/mod.rs
//! Python bindings for Aly
//! 
//! This module provides Python bindings for the Aly programming language,
//! allowing Aly code to be embedded and executed within Python applications.
//! 
//! # Overview
//! The Python bindings expose Aly's core functionality through a Python-friendly API,
//! enabling:
//! - Execution of Aly code snippets
//! - Access to Aly's VM and runtime
//! - Integration with Python data structures
//! - Platform-specific features

use pyo3::prelude::*;
use pyo3::types::PyDict;

/// Aly Python bindings module
/// 
/// This struct provides the main API for interacting with Aly from Python.
/// It exposes core Aly functionality including compilation, execution,
/// and access to built-in functions and modules.
pub struct AlyBindings;

impl AlyBindings {
    /// Execute Aly source code
    /// 
    /// This function compiles and executes Aly source code, returning the result
    /// as a Python object. It's the primary entry point for running Aly code.
    /// 
    /// # Arguments
    /// * `py` - The PyO3 Python interpreter
    /// * `source` - Aly source code as a string
    /// * `module` - Optional module name for error reporting
    /// 
    /// # Returns
    /// A PyO3 result containing the execution result or an error
    /// 
    /// # Example
    /// ```python
    /// import aly_bindings
    /// result = aly_bindings.execute("let x = 42; x")
    /// print(result)  # 42
    /// ```
    pub fn execute(
        py: Python,
        source: &str,
        module: Option<&str>,
    ) -> PyResult<PyObject> {
        // Parse source using Aly's parser
        let program = parser::parse_program(source);
        
        // Compile to VM bytecode
        let chunk = crate::vm::compiler::compile_program(&program)
            .map_err(|e| PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(e))?;
        
        // Execute bytecode
        let result = crate::vm::vm::execute(&chunk)
            .map_err(|e| PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(e))?;
        
        // Convert Aly value to Python object
        Ok(aly_to_py(&result, py))
    }

    /// Get a reference to a built-in Aly function
    /// 
    /// This allows access to Aly's built-in functions (print, len, etc.)
    /// from Python code.
    /// 
    /// # Arguments
    /// * `py` - The PyO3 Python interpreter
    /// * `name` - Name of the function to retrieve
    /// 
    /// # Returns
    /// A callable Python object representing the Aly function
    pub fn get_builtin_function(
        py: Python,
        name: &str,
    ) -> PyResult<PyObject> {
        let runtime = crate::aly::get_runtime();
        match runtime.get_var_per_name(name) {
            Ok(var) => {
                let func_val = var.get_value();
                Ok(aly_to_py(&func_val, py))
            }
            Err(_) => {
                Err(PyErr::new::<pyo3::exceptions::PyAttributeError, _>(
                    format!("Unknown built-in function: {}", name)
                ))
            }
        }
    }

    /// List all available built-in functions
    /// 
    /// Returns a list of names of all built-in Aly functions
    /// 
    /// # Returns
    /// A Vec<String> containing function names
    pub fn list_builtin_functions(&self) -> Vec<String> {
        let runtime = crate::aly::get_runtime();
        let mut functions = Vec::new();
        
        // Get all global variables from Aly runtime
        for (_, var) in runtime.global_vars.iter() {
            match var.get_value() {
                crate::vm::value::Value::Function { .. } => {
                    functions.push(var.name.clone());
                }
                _ => {}
            }
        }
        
        functions
    }

    /// Check if an Aly value is truthy
    /// 
    /// This helper function determines whether an Aly value evaluates to true
    /// in a boolean context.
    /// 
    /// # Arguments
    /// * `value` - The Aly value to check
    /// 
    /// # Returns
    /// true if the value is truthy, false otherwise
    pub fn is_truthy(&self, value: &crate::vm::value::Value) -> bool {
        crate::validators::is_conditional_exp(
            crate::tokens::Tokens::Value,
            value.to_string(false)
        )
    }

    /// Convert Aly value to Python object
    /// 
    /// This internal function converts Aly values to Python objects for returning
    /// results from Python functions.
    /// 
    /// # Arguments
    /// * `value` - The Aly value to convert
    /// * `py` - The PyO3 Python interpreter
    /// 
    /// # Returns
    /// A Python object representing the Aly value
    fn aly_to_py(value: &crate::vm::value::Value, py: Python) -> PyObject {
        match value {
            crate::vm::value::Value::Nil => py.None(),
            crate::vm::value::Value::Bool(b) => b.into_py(py),
            crate::vm::value::Value::Int(i) => (*i).into_py(py),
            crate::vm::value::Value::Float(f) => (*f).into_py(py),
            crate::vm::value::Value::Str(s) => s.clone().into_py(py),
            crate::vm::value::Value::Function { .. } => {
                // Convert function to Python callable
                // This is a simplified implementation
                PyObject::from(py.None())
            }
            crate::vm::value::Value::Array(arr) => {
                let py_list: Vec<PyObject> = arr
                    .get_elements()
                    .iter()
                    .map(|v| Self::aly_to_py(v, py))
                    .collect();
                py_list.into_py(py)
            }
            crate::vm::value::Value::Object(obj) => {
                let py_dict = PyDict::new(py);
                for (key, val) in obj {
                    py_dict.set_item(
                        key.clone().into_py(py),
                        Self::aly_to_py(val, py)
                    ).unwrap();
                }
                py_dict.into_py(py)
            }
            crate::vm::value::Value::Reference { .. } => {
                // Handle reference types
                py.None()
            }
        }
    }
}

/// Initialize Aly bindings for Python
/// 
/// This function must be called before using any Aly bindings from Python.
/// It sets up the Aly runtime and initializes necessary components.
/// 
/// # Panics
/// Panics if Aly runtime initialization fails
pub fn init_aly_bindings() {
    // Initialize Aly runtime
    let runtime = crate::aly::get_runtime();
    
    // Set up default global functions
    runtime.register_function(
        "print",
        crate::vm::value::Value::Function {
            body: Vec::new(),
            arity: 1,
        }
    ).expect("Failed to register print function");
    
    // Initialize other built-ins as needed
}
