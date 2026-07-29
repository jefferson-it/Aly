// Android FFI layer for Aly runtime
// This is compiled as a cdylib and loaded via JNI

#![cfg(feature = "android")]

use std::ffi::{CStr, CString};
use std::os::raw::{c_char, c_int, c_void};
use std::sync::{OnceLock, Mutex};
use jni::{
    objects::{JObject, JValue},
    JavaVM,
};

use crate::vm::execute;
use crate::native::jni::{init_jvm, get_jvm, with_env};

pub mod hardware;

// Global runtime state
static RUNTIME_INITIALIZED: std::sync::atomic::AtomicBool = std::sync::atomic::AtomicBool::new(false);
static LAST_ERROR: Mutex<String> = Mutex::new(String::new());
static CALLBACKS: once_cell::sync::Lazy<Mutex<std::collections::HashMap<String, jni::objects::GlobalRef>>> = once_cell::sync::Lazy::new(|| Mutex::new(std::collections::HashMap::new()));

fn set_last_error(msg: &str) {
    if let Ok(mut err) = LAST_ERROR.lock() {
        *err = msg.to_string();
    }
}

fn clear_last_error() {
    if let Ok(mut err) = LAST_ERROR.lock() {
        err.clear();
    }
}

#[no_mangle]
pub extern "C" fn aly_runtime_init(env: *mut c_void) {
    let env = env as *mut jni::sys::JNIEnv;
    clear_last_error();
    
    // Get JavaVM from the JNIEnv
    let vm: JavaVM = unsafe {
        let mut vm: *mut jni::sys::JavaVM = std::ptr::null_mut();
        let result = (*(*env)).GetJavaVM.unwrap()(env, &mut vm);
        if result != jni::sys::JNI_OK {
            set_last_error("Failed to get JavaVM");
            return;
        }
        JavaVM::from_raw(vm).expect("Failed to create JavaVM from raw pointer")
    };
    
    init_jvm(vm);
    
    RUNTIME_INITIALIZED.store(true, std::sync::atomic::Ordering::SeqCst);
}

#[no_mangle]
pub extern "C" fn aly_android_init(env: *mut c_void, context: *mut c_void) {
    let env = env as *mut jni::sys::JNIEnv;
    let context = context as jni::sys::jobject;
    
    clear_last_error();
    
    // Store the Android context for later use
    if let Some(jvm) = get_jvm() {
        let mut jni_env = match jvm.get_env() {
            Ok(e) => e,
            Err(_) => {
                set_last_error("Failed to get JNIEnv");
                return;
            }
        };
        
        // We could store the context here for later use
        // For now, just verify we can access it
        let _context_obj = unsafe { JObject::from_raw(context as *mut _) };
    } else {
        set_last_error("JVM not initialized");
    }
}

#[no_mangle]
pub extern "C" fn aly_eval(code: *const c_char) -> *mut c_char {
    clear_last_error();
    
    if !RUNTIME_INITIALIZED.load(std::sync::atomic::Ordering::SeqCst) {
        set_last_error("Runtime not initialized");
        return CString::new("Error: Runtime not initialized").unwrap().into_raw();
    }
    
    let code_str = unsafe {
        if code.is_null() {
            set_last_error("Null code pointer");
            return CString::new("Error: Null code").unwrap().into_raw();
        }
        match CStr::from_ptr(code).to_str() {
            Ok(s) => s,
            Err(_) => {
                set_last_error("Invalid UTF-8 in code");
                return CString::new("Error: Invalid UTF-8").unwrap().into_raw();
            }
        }
    };
    
    // Execute the Aly code using the VM
    match execute(code_str) {
        Ok(_) => {
            // Return success indicator
            CString::new("OK").unwrap().into_raw()
        }
        Err(e) => {
            set_last_error(&e);
            CString::new(format!("Error: {}", e)).unwrap().into_raw()
        }
    }
}

#[no_mangle]
pub extern "C" fn aly_call_function(name: *const c_char, argc: c_int, argv: *mut *mut c_char) -> *mut c_char {
    clear_last_error();
    
    if !RUNTIME_INITIALIZED.load(std::sync::atomic::Ordering::SeqCst) {
        set_last_error("Runtime not initialized");
        return CString::new("Error: Runtime not initialized").unwrap().into_raw();
    }
    
    let name_str = unsafe {
        if name.is_null() {
            set_last_error("Null function name");
            return CString::new("Error: Null function name").unwrap().into_raw();
        }
        match CStr::from_ptr(name).to_str() {
            Ok(s) => s,
            Err(_) => {
                set_last_error("Invalid UTF-8 in function name");
                return CString::new("Error: Invalid UTF-8").unwrap().into_raw();
            }
        }
    };
    
    // Build the function call as Aly code
    let mut call_code = format!("{}(", name_str);
    
    for i in 0..argc {
        let arg_ptr = unsafe { *argv.offset(i as isize) };
        let arg_str = unsafe {
            if arg_ptr.is_null() {
                "null"
            } else {
                match CStr::from_ptr(arg_ptr).to_str() {
                    Ok(s) => s,
                    Err(_) => {
                        set_last_error("Invalid UTF-8 in argument");
                        return CString::new("Error: Invalid UTF-8 in argument").unwrap().into_raw();
                    }
                }
            }
        };
        
        if i > 0 {
            call_code.push_str(", ");
        }
        // Escape string arguments
        if arg_str.contains('"') || arg_str.contains('\\') {
            call_code.push_str(&format!("\"{}\"", arg_str.replace('\\', "\\\\").replace('"', "\\\"")));
        } else {
            call_code.push_str(&format!("\"{}\"", arg_str));
        }
    }
    call_code.push(')');
    
    // Execute the function call
    match execute(&call_code) {
        Ok(_) => CString::new("OK").unwrap().into_raw(),
        Err(e) => {
            set_last_error(&e);
            CString::new(format!("Error: {}", e)).unwrap().into_raw()
        }
    }
}

#[no_mangle]
pub extern "C" fn aly_register_callback(env: *mut c_void, name: *const c_char, callback: *mut c_void) {
    clear_last_error();
    
    let env = env as *mut jni::sys::JNIEnv;
    let name_str = unsafe {
        if name.is_null() {
            set_last_error("Null callback name");
            return;
        }
        match CStr::from_ptr(name).to_str() {
            Ok(s) => s.to_string(),
            Err(_) => {
                set_last_error("Invalid UTF-8 in callback name");
                return;
            }
        }
    };
    
    if callback.is_null() {
        // Remove callback
        if let Ok(mut callbacks) = CALLBACKS.lock() {
            callbacks.remove(&name_str);
        }
        return;
    }
    
    // Store the global reference to the callback
    let callback_obj = unsafe { JObject::from_raw(callback as *mut _) };
    
    // Convert to global reference
    let jvm = match get_jvm() {
        Some(vm) => vm,
        None => {
            set_last_error("JVM not initialized");
            return;
        }
    };
    
    let mut jni_env = match jvm.get_env() {
        Ok(e) => e,
        Err(_) => {
            set_last_error("Failed to get JNIEnv");
            return;
        }
    };
    
    let global_ref = match jni_env.new_global_ref(&callback_obj) {
        Ok(r) => r,
        Err(_) => {
            set_last_error("Failed to create global ref");
            return;
        }
    };
    
    if let Ok(mut callbacks) = CALLBACKS.lock() {
        callbacks.insert(name_str, global_ref);
    }
}

#[no_mangle]
pub extern "C" fn aly_runtime_shutdown() {
    clear_last_error();
    
    // Clean up callbacks
    if let Ok(mut callbacks) = CALLBACKS.lock() {
        callbacks.clear();
    }
    
    RUNTIME_INITIALIZED.store(false, std::sync::atomic::Ordering::SeqCst);
}

#[no_mangle]
pub extern "C" fn aly_get_last_error() -> *const c_char {
    if let Ok(err) = LAST_ERROR.lock() {
        if err.is_empty() {
            return std::ptr::null();
        }
        // Note: This leaks memory, but it's a C API requirement
        CString::new(err.clone()).unwrap().into_raw()
    } else {
        std::ptr::null()
    }
}

#[no_mangle]
pub extern "C" fn aly_is_initialized() -> c_int {
    if RUNTIME_INITIALIZED.load(std::sync::atomic::Ordering::SeqCst) {
        1
    } else {
        0
    }
}

// Helper to call Java callbacks from Rust
pub fn call_java_callback(name: &str, result: &str) -> Result<(), String> {
    let callback = {
        let callbacks = CALLBACKS.lock().map_err(|_| "Lock poisoned")?;
        callbacks.get(name).map(|gref| unsafe { jni::objects::JObject::from_raw(gref.as_obj().as_raw()) })
    };
    
    let Some(callback_obj) = callback else {
        return Ok(()); // No callback registered
    };
    
    let jvm = get_jvm().ok_or("JVM not initialized")?;
    let mut env = jvm.get_env().map_err(|e| format!("Failed to get env: {:?}", e))?;
    
    // Call the callback with the result string
    let j_result = env.new_string(result).map_err(|e| format!("Failed to create string: {:?}", e))?;
    
    env.call_method(&callback_obj, "onResult", "(Ljava/lang/String;)V", &[JValue::Object(&j_result)])
        .map_err(|e| format!("Failed to call callback: {:?}", e))?;
    
    Ok(())
}