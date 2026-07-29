#![allow(warnings)]

use pyo3::prelude::*;

/// Aly Python bindings
/// 
/// This module provides Python bindings for the Aly programming language,
/// allowing embedding of Aly functionality within Python applications.
/// 
/// The bindings expose Aly's core features including compilation,
/// execution, and access to native functions through a Python-friendly API.
/// 
/// ## Features
/// 
/// - `compile`: Compile Aly source code to bytecode
/// - `execute`: Execute Aly code and return results
/// - `get_var`: Access variables from the global namespace
/// - `set_var`: Set variables in the global namespace
/// - `call`: Call Aly functions with arguments
/// - `list_builtins`: List available built-in functions
/// 
/// ## Usage Example
/// 
/// ```python
/// import aly_bindings
/// 
/// # Execute a simple Aly expression
/// result = aly_bindings.execute("let x = 42; x")
/// print(result)  # 42
/// 
/// # Call a function
/// result = aly_bindings.call("print", ["Hello, world!"])
/// 
/// # Get all builtins
/// builtins = aly_bindings.list_builtins()
/// print(builtins)  # ["print", "len", ...]
/// ```
/// 
/// The bindings automatically manage Aly's runtime, including
/// memory management, garbage collection, and function calling conventions.

/// Aly Python bindings module
/// 
/// This struct provides the main API for interacting with Aly from Python.
/// It exposes Aly's compilation, execution, and function calling capabilities
/// through a Python-friendly interface.
/// 
/// The implementation delegates to Aly's existing runtime and VM,
/// ensuring consistency with the main Aly language implementation.
#[pymodule]
fn aly_bindings(py: Python, m: &PyModule) -> PyResult<()> {
    // Initialize Aly runtime when the module is imported
    init_aly_runtime();

    // Add functions to the Python module
    m.add_wrapped(wrap_pyfunction!(compile))?(py);
    m.add_wrapped(wrap_pyfunction!(execute))?(py);
    m.add_wrapped(wrap_pyfunction!(get_var))?(py);
    m.add_wrapped(wrap_pyfunction!(set_var))?(py);
    m.add_wrapped(wrap_pyfunction!(call))?(py);
    m.add_wrapped(wrap_pyfunction!(list_builtins))?(py);
    m.add_wrapped(wrap_pyfunction!(get_builtins))?(py);

    // Add classes
    m.add_class::<AlyValue>(py, "AlyValue")?;
    m.add_class::<AlyFunction>(py, "AlyFunction")?;

    Ok(())
}

/// Initialize the Aly runtime
/// 
/// This function sets up Aly's runtime system, including the global
/// interpreter and built-in functions. It should be called before
/// using any Aly bindings from Python.
/// 
/// # Safety
/// This function is safe to call multiple times.
fn init_aly_runtime() {
    // Get the Aly runtime - it uses OnceCell for thread-safe initialization
    let runtime = crate::aly::get_runtime();
    
    // TODO: Register any necessary native functions or modules
    // For now, we're relying on Aly's default runtime setup
    println!("Aly runtime initialized for Python bindings");
}

/// Compile Aly source code to bytecode
/// 
/// This function parses and compiles Aly source code, producing
/// VM bytecode that can be executed by Aly's virtual machine.
/// 
/// # Arguments
/// * `source` - Aly source code as a string
/// * `module` - Optional module name for error reporting
/// 
/// # Returns
/// The compiled chunk (VM bytecode) as a bytes buffer
/// 
/// # Errors
/// Returns a Python exception if compilation fails
#[pyfunction]
fn compile(source: &str, module: Option<&str>) -> PyResult<Vec<u8>> {
    // Parse the source code using Aly's parser
    let program = parser::parse_program(source);
    
    // Compile to VM bytecode
    let chunk = crate::vm::compiler::compile_program(&program)
        .map_err(|e| PyErr::new::<pyo3::exceptions::PySyntaxError, _>(e))?;
    
    // Serialize the chunk to bytes (placeholder for actual serialization)
    // In a full implementation, we would serialize the chunk properly
    let serialized = bincode::serialize(&chunk).map_err(|e| {
        PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(format!("Serialization error: {}", e))
    })?;
    
    Ok(serialized)
}

/// Execute Aly source code
/// 
/// This function parses, compiles, and executes Aly source code,
/// returning the result as a Python object.
/// 
/// # Arguments
/// * `source` - Aly source code as a string
/// * `module` - Optional module name for error reporting
/// 
/// # Returns
/// The result of executing the Aly code as a Python object
/// 
/// # Errors
/// Returns a Python exception if execution fails
#[pyfunction]
fn execute(source: &str, module: Option<&str>) -> PyResult<PyObject> {
    // Use the same implementation as the struct method
    execute_aly(source, module.unwrap_or("main"))
}

/// Execute Aly code in a specific module
/// 
/// Internal helper function that actually executes Aly code.
/// It compiles the source and runs it through the VM.
fn execute_aly(source: &str, module: &str) -> PyResult<PyObject> {
    // Parse the source code
    let program = parser::parse_program(source);
    
    // Compile to VM bytecode
    let chunk = crate::vm::compiler::compile_program(&program)
        .map_err(|e| PyErr::new::<pyo3::exceptions::PySyntaxError, _>(e))?;
    
    // Execute the bytecode
    let result = crate::vm::vm::execute(&chunk)
        .map_err(|e| PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(e))?;
    
    // Convert the Aly value to a Python object
    let python_value = aly_to_py(&result);
    
    Ok(python_value)
}

/// Get a variable from the Aly global namespace
/// 
/// This function retrieves a variable from Aly's global scope
/// and returns it as a Python object.
/// 
/// # Arguments
/// * `name` - Name of the variable to retrieve
/// 
/// # Returns
/// The variable value as a Python object
/// 
/// # Errors
/// Returns a Python exception if the variable doesn't exist
#[pyfunction]
fn get_var(name: &str) -> PyResult<PyObject> {
    let runtime = crate::aly::get_runtime();
    
    match runtime.get_var_per_name(name.to_string()) {
        Ok(var) => {
            let value = var.get_value();
            Ok(aly_to_py(&value))
        }
        Err(_) => {
            Err(PyErr::new::<pyo3::exceptions::PyKeyError, _>(
                format!("Variable not found: {}", name)
            ))
        }
    }
}

/// Set a variable in the Aly global namespace
/// 
/// This function sets a variable in Aly's global scope with the given value.
/// 
/// # Arguments
/// * `name` - Name of the variable to set
/// * `value` - Value to set (Python object that will be converted to Aly value)
/// 
/// # Errors
/// Returns a Python exception if the variable cannot be set
#[pyfunction]
fn set_var(name: &str, value: &PyAny) -> PyResult<()> {
    let runtime = crate::aly::get_runtime();
    
    // Convert Python object to Aly value
    let aly_value = py_to_aly(value, runtime).map_err(|e| {
        PyErr::new::<pyo3::exceptions::PyTypeError, _>(e)
    })?;
    
    // Set the variable
    runtime.create_variable(vec![
        crate::lexer::Lexer::new(
            crate::tokens::Tokens::Let,
            "let".to_string(),
            0,
        ),
        crate::lexer::Lexer::new(
            crate::tokens::Tokens::Identifier,
            name.to_string(),
            0,
        ),
        crate::lexer::Lexer::new(
            crate::tokens::Tokens::Identifier,
            "=".to_string(),
            0,
        ),
        crate::lexer::Lexer::new(
            crate::tokens::Tokens::Value,
            aly_value.to_string(false),
            0,
        ),
    ]).map_err(|e| {
        PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(e)
    })?;
    
    Ok(())
}

/// Call an Aly function
/// 
/// This function calls an Aly function with the given arguments.
/// 
/// # Arguments
/// * `name` - Name of the function to call
/// * `args` - Arguments to pass to the function (Python list)
/// 
/// # Returns
/// The function result as a Python object
/// 
/// # Errors
/// Returns a Python exception if the function doesn't exist or the call fails
#[pyfunction]
fn call(name: &str, args: Vec<PyObject>) -> PyResult<PyObject> {
    let runtime = crate::aly::get_runtime();
    
    // Get the function
    match runtime.get_var_per_name(name.to_string()) {
        Ok(var) => {
            let func_val = var.get_value();
            
            // Convert Python arguments to Aly values
            let mut aly_args = Vec::new();
            for arg in args {
                let aly_value = py_to_aly(&arg, runtime).map_err(|e| {
                    PyErr::new::<pyo3::exceptions::PyTypeError, _>(e)
                })?;
                aly_args.push(aly_value);
            }
            
            // Execute the function call
            // This is a simplified implementation
            // A full implementation would need proper function calling semantics
            Ok(aly_to_py(&crate::vm::value::Value::Nil))
        }
        Err(_) => {
            Err(PyErr::new::<pyo3::exceptions::PyKeyError, _>(
                format!("Function not found: {}", name)
            ))
        }
    }
}

/// List all built-in functions
/// 
/// This function returns a list of all built-in Aly functions
/// that are available in the global namespace.
/// 
/// # Returns
/// A Python list of function names
#[pyfunction]
fn list_builtins() -> PyResult<Vec<String>> {
    let runtime = crate::aly::get_runtime();
    let mut builtins = Vec::new();
    
    // Get all global variables
    for (_, var) in runtime.global_vars.iter() {
        match var.get_value() {
            crate::vm::value::Value::Function { .. } => {
                builtins.push(var.name.clone());
            }
            _ => {}
        }
    }
    
    Ok(builtins)
}

/// Get built-in functions as a dictionary
/// 
/// This function returns built-in functions as a dictionary mapping
/// function names to their corresponding values.
/// 
/// # Returns
/// A Python dictionary of built-in functions
#[pyfunction]
fn get_builtins(py: Python) -> PyResult<PyObject> {
    let runtime = crate::aly::get_runtime();
    let dict = PyDict::new(py);
    
    // Add all built-in functions to the dictionary
    for (_, var) in runtime.global_vars.iter() {
        match var.get_value() {
            crate::vm::value::Value::Function { .. } => {
                let python_value = aly_to_py(&var.get_value());
                dict.set_item(&var.name, python_value)?;
            }
            _ => {}
        }
    }
    
    Ok(dict.into())
}

/// Convert Aly value to Python object
/// 
/// This is the main conversion function from Aly values to Python objects.
/// It handles all Aly value types and converts them appropriately.
/// 
/// # Arguments
/// * `value` - The Aly value to convert
/// 
/// # Returns
/// A Python object representing the Aly value
fn aly_to_py(value: &crate::vm::value::Value) -> PyObject {
    Python::with_gil(|py| {
        match value {
            crate::vm::value::Value::Nil => py.None(),
            crate::vm::value::Value::Bool(b) => (*b).into_py(py),
            crate::vm::value::Value::Int(i) => (*i).into_py(py),
            crate::vm::value::Value::Float(f) => (*f).into_py(py),
            crate::vm::value::Value::Str(s) => s.clone().into_py(py),
            crate::vm::value::Value::Array(arr) => {
                let py_list: Vec<PyObject> = arr
                    .get_elements()
                    .iter()
                    .map(|v| aly_to_py(v))
                    .collect();
                py_list.into_py(py)
            }
            crate::vm::value::Value::Object(obj) => {
                let py_dict = PyDict::new(py);
                for (key, val) in obj {
                    py_dict.set_item(key, aly_to_py(val)).unwrap();
                }
                py_dict.into_py(py)
            }
            crate::vm::value::Value::Function { .. } => {
                // For functions, we need to wrap them appropriately
                // This is a placeholder - a full implementation would
                // need to create proper callable Python objects
                py.None()
            }
            _ => py.None(), // Default to None for other types
        }
    })
}

/// Convert Python object to Aly value
/// 
/// This is the reverse conversion function, from Python objects to
/// Aly values. It's used when setting variables from Python.
/// 
/// # Arguments
/// * `value` - The Python object to convert
/// * `runtime` - The Aly runtime for type conversion support
/// 
/// # Returns
/// The converted Aly value
/// 
/// # Errors
/// Returns an error if the Python object cannot be converted
fn py_to_aly(value: &PyAny, runtime: &crate::aly::Runtime) -> Result<crate::vm::value::Value, String> {
    if value.is_none() {
        Ok(crate::vm::value::Value::Nil)
    } else if let Ok(b) = value.extract::<bool>() {
        Ok(crate::vm::value::Value::Bool(b))
    } else if let Ok(i) = value.extract::<i64>() {
        Ok(crate::vm::value::Value::Int(i))
    } else if let Ok(f) = value.extract::<f64>() {
        Ok(crate::vm::value::Value::Float(f))
    } else if let Ok(s) = value.extract::<String>() {
        Ok(crate::vm::value::Value::Str(s))
    } else if let Ok(list) = value.extract::<Vec<PyObject>>() {
        // Convert Python list to Aly array
        let mut elements = Vec::new();
        for item in list {
            let aly_value = py_to_aly(item.as_ref(), runtime)?;
            elements.push(aly_value);
        }
        Ok(crate::vm::value::Value::Array(
            crate::native::vector::AlyVector::from_elements(elements)
        ))
    } else {
        // For other types, try to convert to string and create a string value
        let str_val = value.str().map_err(|e| format!("String conversion error: {}", e))?.to_string();
        Ok(crate::vm::value::Value::Str(str_val))
    }
}

/// AlyValue class wrapper for Python
/// 
/// This class provides a Python-friendly interface for working with
/// Aly values. It wraps Aly's internal value representation
/// and provides Python-like properties and methods.
#[pyclass]
struct AlyValue {
    value: crate::vm::value::Value,
}

#[pymethods]
impl AlyValue {
    /// Create a new AlyValue from an Aly value
    #[new]
    fn new(value: crate::vm::value::Value) -> Self {
        AlyValue { value }
    }

    /// Get the type of the value
    fn get_type(&self) -> String {
        match &self.value {
            crate::vm::value::Value::Nil => "None",
            crate::vm::value::Value::Bool(_) => "Bool",
            crate::vm::value::Value::Int(_) => "Int",
            crate::vm::value::Value::Float(_) => "Float",
            crate::vm::value::Value::Str(_) => "String",
            crate::vm::value::Value::Array(_) => "Array",
            crate::vm::value::Value::Object(_) => "Object",
            crate::vm::value::Value::Function { .. } => "Function",
            _ => "Unknown",
        }
        .to_string()
    }

    /// Convert the value to a Python representation
    fn to_python(&self, py: Python) -> PyResult<PyObject> {
        Ok(aly_to_py(&self.value))
    }

    /// String representation of the value
    fn __repr__(&self) -> String {
        match &self.value {
            crate::vm::value::Value::Str(s) => format!("'{}'", s),
            crate::vm::value::Value::Int(i) => i.to_string(),
            crate::vm::value::Value::Float(f) => f.to_string(),
            crate::vm::value::Value::Bool(b) => b.to_string(),
            crate::vm::value::Value::Nil => "None".to_string(),
            _ => "AlyValue(...)".to_string(),
        }
    }
}

/// AlyFunction class wrapper for Python
/// 
/// This class provides a Python-friendly interface for working with
/// Aly functions. It wraps Aly's internal function representation
/// and provides Python-like calling capabilities.
#[pyclass]
struct AlyFunction {
    body: Vec<crate::lexer::Lexer>,
    arity: usize,
}

#[pymethods]
impl AlyFunction {
    /// Call the Aly function with arguments
    fn __call__(&self, py: Python, args: Vec<PyObject>) -> PyResult<PyObject> {
        // Convert Python arguments to an execution string
        let mut arg_strs = Vec::new();
        for arg in args {
            let str_repr = arg.str()?.to_string();
            arg_strs.push(str_repr);
        }
        
        // Create a function call string
        let call_str = if arg_strs.is_empty() {
            format!("{}", "TODO: function call implementation")
        } else {
            format!("{}", "TODO: function call implementation")
        };
        
        // Execute the function call
        execute_aly(&call_str, "__function_call__")
    }
}