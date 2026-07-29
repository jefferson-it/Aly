
pub struct RustBindings {
    crate_name: String,
    module_name: String,
    functions: Vec<BoundFunction>,
}

struct BoundFunction {
    name: String,
    params: Vec<RustType>,
    return_type: RustType,
}

pub enum RustType {
    I32,
    I64,
    F64,
    Bool,
    String,
    Void,
    Object,
}

impl RustType {
    fn rust_ty(&self) -> &str {
        match self {
            RustType::I32 => "i32",
            RustType::I64 => "i64",
            RustType::F64 => "f64",
            RustType::Bool => "bool",
            RustType::String => "String",
            RustType::Void => "()",
            RustType::Object => "*mut std::ffi::c_void",
        }
    }
}

impl RustBindings {
    pub fn new(crate_name: &str, module_name: &str) -> Self {
        RustBindings {
            crate_name: crate_name.to_string(),
            module_name: module_name.to_string(),
            functions: Vec::new(),
        }
    }

    pub fn add_function(&mut self, name: &str, params: Vec<RustType>, return_type: RustType) {
        self.functions.push(BoundFunction {
            name: name.to_string(),
            params,
            return_type,
        });
    }

    pub fn generate_lib_rs(&self) -> String {
        let mut s = String::new();
        s.push_str("//! Rust bindings generated for Aly interop
");
        s.push_str("//! Bridge Rust <-> Aly via C plugin ABI.

");
        s.push_str("use std::ffi::{CStr, CString};
");
        s.push_str("use std::os::raw::{c_char, c_int, c_longlong, c_double, c_void};

");
        s.push_str("#[repr(C)]
pub struct AlyPluginInfo {    pub api_version: i32,    pub name: *const c_char,\n    pub version: *const c_char,\n    pub description: *const c_char,\n}\n
");
        s.push_str("extern "C" {
    pub fn aly_plugin_init() -> *mut AlyPluginInfo;
");
        s.push_str("    pub fn aly_plugin_call(func_name: *const c_char, args: *const c_char) -> *mut c_char;
");
        s.push_str("    pub fn aly_plugin_free_string(ptr: *mut c_char);
");
        s.push_str("    pub fn aly_plugin_functions() -> *mut c_char;
}

");

        let module_fn = self.module_name.replace("-", "_");
        s.push_str(&format!("pub mod {} {{
", module_fn));
        s.push_str("    use super::*;

");

        for func in &self.functions {
            let ret = func.return_type.rust_ty();
            let params: Vec<String> = func.params.iter().map(|t| t.rust_ty().to_string()).collect();
            s.push_str(&format!("    pub fn {}({}) -> {} {{
",
                func.name,
                params.join(", "),
                ret
            ));
            s.push_str("        let c_name = CString::new(");
            s.push_str(&format!(""{}"", func.name));
            s.push_str(").unwrap();
");
            s.push_str("        let c_args = CString::new("").unwrap();
");
            s.push_str("        let ret_ptr = unsafe { aly_plugin_call(c_name.as_ptr(), c_args.as_ptr()) };
");
            s.push_str("        if ret_ptr.is_null() {
");
            if let RustType::String = func.return_type {
                s.push_str("            return String::new();
");
            } else if let RustType::Void = func.return_type {
                s.push_str("            return;
");
            } else if let RustType::Bool = func.return_type {
                s.push_str("            return false;
");
            } else {
                s.push_str("            return Default::default();
");
            }
            s.push_str("        }
");
            if let RustType::String = func.return_type {
                s.push_str("        let cstr = unsafe { CStr::from_ptr(ret_ptr) };
        let out = cstr.to_string_lossy().into_owned();
        unsafe { aly_plugin_free_string(ret_ptr) };
        out
    }
");
            } else if let RustType::Void = func.return_type {
                s.push_str("        unsafe { aly_plugin_free_string(ret_ptr) };
    }
");
            } else if let RustType::Bool = func.return_type {
                s.push_str("        let cstr = unsafe { CStr::from_ptr(ret_ptr) };
        let out = cstr.to_string_lossy().parse::<i32>().unwrap_or(0) != 0;
        unsafe { aly_plugin_free_string(ret_ptr) };
        out
    }
");
            } else {
                s.push_str(&format!("        let cstr = unsafe { CStr::from_ptr(ret_ptr) };
        let out: {} = cstr.to_string_lossy().parse().unwrap_or_default();
        unsafe { aly_plugin_free_string(ret_ptr) };
        out
    }

", ret));
            }
        }
        s.push_str("}

}
");
        s
    }

    pub fn generate_cargo_toml(&self) -> String {
        format!(
            "[package]\nname = "{}"\nversion = "0.1.0\"\nedition = "2021\"\n\n[dependencies]\n",
            self.crate_name
        )
    }
}
