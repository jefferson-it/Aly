use std::ffi::{CStr, CString};
use std::os::raw::c_char;

#[repr(C)]
pub struct AlyPluginInfo {
    pub api_version: i32,
    pub name: *const c_char,
    pub version: *const c_char,
    pub description: *const c_char,
}

static INFO: AlyPluginInfo = AlyPluginInfo {
    api_version: 1,
    name: c"Hash Extension".as_ptr(),
    version: c"1.0.0".as_ptr(),
    description: c"Funções de hash SHA-256 para Aly".as_ptr(),
};

#[no_mangle]
pub extern "C" fn aly_plugin_init() -> *mut AlyPluginInfo {
    &INFO as *const AlyPluginInfo as *mut AlyPluginInfo
}

fn cmd_sha256(args: &str) -> String {
    use sha2::{Digest, Sha256};
    let mut hasher = Sha256::new();
    hasher.update(args.trim());
    hex::encode(hasher.finalize())
}

#[no_mangle]
pub extern "C" fn aly_plugin_call(
    func_name: *const c_char,
    args: *const c_char,
) -> *mut c_char {
    let func = unsafe { CStr::from_ptr(func_name) }.to_string_lossy().to_string();
    let args = unsafe { CStr::from_ptr(args) }.to_string_lossy().to_string();

    let result = match func.as_str() {
        "sha256" => cmd_sha256(&args),
        _ => return std::ptr::null_mut(),
    };

    CString::new(result).unwrap_or_default().into_raw()
}

#[no_mangle]
pub extern "C" fn aly_plugin_free_string(ptr: *mut c_char) {
    if ptr.is_null() {
        return;
    }
    unsafe {
        let _ = CString::from_raw(ptr);
    }
}

#[no_mangle]
pub extern "C" fn aly_plugin_functions() -> *mut c_char {
    CString::new("sha256").unwrap_or_default().into_raw()
}
