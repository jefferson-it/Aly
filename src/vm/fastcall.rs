use super::VM;
use super::value::Value;

/// Fast‑call ABI implementation for native function calls.
///
/// This module defines a low‑latency calling convention that avoids
/// intermediate data structures and reduces interpreter overhead.
pub struct FastCallABI;

impl FastCallABI {
    /// Invoke a native function using the optimized ABI.
    ///
    /// This replaces the generic `Call` path for native functions.
    pub fn invoke(vm: &mut VM, func_name: &str, arg_count: usize) -> Result<(), String> {
        // Look up the native function
        let func = vm
            .globals
            .get(func_name)
            .and_then(|v| if let Value::Native(func, _) = v { Some(func) } else { None })
            .ok_or_else(|| format!("Native function '{}' not found", func_name))?;
        
        // Extract arguments from stack
        let stack_len = vm.stack.len();
        let args_start = stack_len.saturating_sub(arg_count);
        
        // Use slice-based argument passing to avoid vector allocations
        let args = &vm.stack[args_start..];
        let result = (func)(args);
        
        // Truncate args and push result
        vm.stack.truncate(args_start);
        vm.stack.push(result);
        
        Ok(())
    }
}