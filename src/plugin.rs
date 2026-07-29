mod plugin {
    use std::cell::RefCell;
    use std::collections::HashMap;
    use std::ffi::{CStr, CString};

    use libloading::{Library, Symbol};

    /// C ABI types for Aly plugins.

    /// Plugin metadata returned by `aly_plugin_init`.
    #[repr(C)]
    pub struct AlyPluginInfo {
        pub api_version: i32,
        pub name: *const std::ffi::c_char,
        pub version: *const std::ffi::c_char,
        pub description: *const std::ffi::c_char,
    }

    type PluginInitFn = unsafe extern "C" fn() -> *mut AlyPluginInfo;
    type PluginCallFn = unsafe extern "C" fn(*const std::ffi::c_char, *const std::ffi::c_char) -> *mut std::ffi::c_char;
    type PluginFreeStringFn = unsafe extern "C" fn(*mut std::ffi::c_char);
    type PluginFunctionsFn = unsafe extern "C" fn() -> *mut std::ffi::c_char;

    /// A loaded plugin instance.
    pub struct LoadedPlugin {
        #[allow(dead_code)]
        lib: Library,
        pub name: String,
        pub version: String,
        pub description: String,
        pub functions: Vec<String>,
        call_fn: PluginCallFn,
        free_fn: PluginFreeStringFn,
    }

    impl LoadedPlugin {
        /// Load a shared library as an Aly plugin.
        ///
        /// Safety: the library must expose the Aly plugin ABI (aly_plugin_init,
        /// aly_plugin_call, aly_plugin_free_string). Loaded from user-specified
        /// paths — only load trusted plugins.
        pub unsafe fn load(path: &str) -> Result<Self, String> {
            let lib = Library::new(path).map_err(|e| {
                format!("Falha ao carregar plugin '{}': {}", path, e)
            })?;

            // Required: aly_plugin_init
            let init_fn: Symbol<PluginInitFn> = lib
                .get(b"aly_plugin_init")
                .map_err(|_| {
                    format!(
                        "Plugin '{}' não exporta a função obrigatória 'aly_plugin_init'.",
                        path
                    )
                })?;

            let info_ptr = init_fn();
            if info_ptr.is_null() {
                return Err(format!(
                    "Plugin '{}' retornou NULL em aly_plugin_init.",
                    path
                ));
            }

            let api_version = (*info_ptr).api_version;
            if api_version != 1 {
                return Err(format!(
                    "Plugin '{}' usa API versão {}, mas a versão esperada é 1.",
                    path, api_version
                ));
            }

            let name = cstr_to_string((*info_ptr).name);
            let version = cstr_to_string((*info_ptr).version);
            let description = cstr_to_string((*info_ptr).description);

            // Required: aly_plugin_call
            let call_fn: Symbol<PluginCallFn> = lib
                .get(b"aly_plugin_call")
                .map_err(|_| {
                    format!(
                        "Plugin '{}' não exporta a função obrigatória 'aly_plugin_call'.",
                        name
                    )
                })?;
            let call_fn_ptr = *call_fn;

            // Required: aly_plugin_free_string
            let free_fn: Symbol<PluginFreeStringFn> = lib
                .get(b"aly_plugin_free_string")
                .map_err(|_| {
                    format!(
                        "Plugin '{}' não exporta a função obrigatória 'aly_plugin_free_string'.",
                        name
                    )
                })?;
            let free_fn_ptr = *free_fn;

            // Optional: aly_plugin_functions — discover exported function names
            let functions = if let Ok(funcs_sym) =
                unsafe { lib.get::<PluginFunctionsFn>(b"aly_plugin_functions") }
            {
                let funcs_fn = *funcs_sym;
                let ptr = funcs_fn();
                if ptr.is_null() {
                    Vec::new()
                } else {
                    let s = cstr_to_string(ptr);
                    unsafe { (free_fn)(ptr) };
                    s.split(',')
                        .map(|f| f.trim().to_string())
                        .filter(|f| !f.is_empty())
                        .collect()
                }
            } else {
                Vec::new()
            };

            Ok(LoadedPlugin {
                lib,
                name,
                version,
                description,
                functions,
                call_fn: call_fn_ptr,
                free_fn: free_fn_ptr,
            })
        }

        /// Call a function within this plugin.
        pub fn call(&self, func_name: &str, args: &str) -> String {
            let c_func = CString::new(func_name).unwrap_or_default();
            let c_args = CString::new(args).unwrap_or_default();

            let ret_ptr = unsafe { (self.call_fn)(c_func.as_ptr(), c_args.as_ptr()) };

            if ret_ptr.is_null() {
                return "None".to_owned();
            }

            let result = cstr_to_string(ret_ptr);
            unsafe { (self.free_fn)(ret_ptr) };
            result
        }
    }

    unsafe impl Send for LoadedPlugin {}

    fn cstr_to_string(ptr: *const std::ffi::c_char) -> String {
        if ptr.is_null() {
            return String::new();
        }
        unsafe { CStr::from_ptr(ptr).to_string_lossy().to_string() }
    }

    // ── Global plugin registry ─────────────────────────────────────────────

    thread_local! {
        static PLUGIN_REGISTRY: RefCell<HashMap<String, LoadedPlugin>> =
            RefCell::new(HashMap::new());
    }

    /// Load a plugin from a shared library path and register it with the given
    /// namespace name. Returns the list of exported function names.
    pub fn load_plugin(path: &str, namespace: &str) -> Result<Vec<String>, String> {
        let plugin = unsafe { LoadedPlugin::load(path) }?;
        let functions = plugin.functions.clone();

        PLUGIN_REGISTRY.with(|reg| {
            reg.borrow_mut()
                .insert(namespace.to_string(), plugin);
        });

        Ok(functions)
    }

    /// Look up a loaded plugin by namespace and call one of its functions.
    pub fn call_plugin(namespace: &str, func_name: &str, args: &str) -> String {
        PLUGIN_REGISTRY.with(|reg| {
            let reg = reg.borrow();
            match reg.get(namespace) {
                Some(plugin) => plugin.call(func_name, args),
                None => format!("None"),
            }
        })
    }

    /// Check if a plugin namespace is loaded.
    pub fn is_plugin_loaded(namespace: &str) -> bool {
        PLUGIN_REGISTRY.with(|reg| reg.borrow().contains_key(namespace))
    }

    /// Get the list of function names for a loaded plugin.
    pub fn plugin_functions(namespace: &str) -> Vec<String> {
        PLUGIN_REGISTRY.with(|reg| {
            reg.borrow()
                .get(namespace)
                .map(|p| p.functions.clone())
                .unwrap_or_default()
        })
    }

    /// Extract the namespace name from a dotted function call like "math.fibonacci"
    /// Returns ("math", "fibonacci").
    pub fn parse_plugin_call(name: &str) -> Option<(String, String)> {
        let dot = name.find('.')?;
        let ns = name[..dot].to_string();
        let func = name[dot + 1..].to_string();
        Some((ns, func))
    }
}

pub use plugin::*;
