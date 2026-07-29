use super::VM;
use super::value::Value;

/// Exception‑free calling conventions for the Aly VM.
///
/// This module refactors the call path to propagate errors 
/// through a standard Result channel rather than panicking.
pub struct CallConventions;

impl CallConventions {
    /// Execute a call without panicking on errors.
    pub fn call_without_panic(vm: &mut VM, arg_count: usize) -> Result<(), String> {
        // Extract the callee from the stack
        let stack_len = vm.stack.len();
        let callee_idx = stack_len.saturating_sub(arg_count + 1);
        if callee_idx >= stack_len {
            return Err("Stack underflow: insufficient arguments for call".to_string());
        }
        
        let callee = vm.stack[callee_idx].clone();
        match callee {
            Value::Fun(_) => {
                // Normal function call path
                vm.call(arg_count + 1, 0).map(|_| ())
            },
            Value::Native(func, _name) => {
                // Native function call
                let args_start = callee_idx + 1;
                let args = &vm.stack[args_start..];
                let result = func(args);
                vm.stack.truncate(callee_idx);
                vm.stack.push(result);
                Ok(())
            },
            _ => Err(format!("CallError: {} is not callable", callee.type_name()))
        }
    }
}