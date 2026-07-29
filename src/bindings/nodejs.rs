pub struct NodeJSBindings {
    module_name: String,
    functions: Vec<BoundFunction>,
}

struct BoundFunction {
    name: String,
    param_count: usize,
    variadic: bool,
}

impl NodeJSBindings {
    pub fn new(module_name: &str) -> Self {
        NodeJSBindings {
            module_name: module_name.to_string(),
            functions: Vec::new(),
        }
    }

    pub fn add_function(&mut self, name: &str, param_count: usize, variadic: bool) {
        self.functions.push(BoundFunction {
            name: name.to_string(),
            param_count,
            variadic,
        });
    }

    pub fn generate_c_source(&self) -> String {
        let mut c = String::new();
        c.push_str(&format!("// Node.js N-API binding for '{}' module\n", self.module_name));
        c.push_str("#include <node_api.h>\n");
        c.push_str("#include \"runtime_aly.h\"\n\n");
        c.push_str("#define NAPI_CALL(env, call) \\\n");
        c.push_str("    do { \\\n");
        c.push_str("        napi_status status = (call); \\\n");
        c.push_str("        if (status != napi_ok) { \\\n");
        c.push_str("            napi_throw_error(env, NULL, napi_get_last_error_info(env)->error_message); \\\n");
        c.push_str("            return NULL; \\\n");
        c.push_str("        } \\\n");
        c.push_str("    } while(0)\n\n");

        for func in &self.functions {
            c.push_str(&self.generate_function_wrapper(func));
        }

        c.push_str(&self.generate_init());
        c
    }

    fn generate_function_wrapper(&self, func: &BoundFunction) -> String {
        let mut wrapper = format!(
            "static napi_value js_{}(napi_env env, napi_callback_info info) {{\n",
            func.name
        );
        wrapper.push_str("    size_t argc = 10;\n");
        wrapper.push_str("    napi_value argv[10];\n");
        wrapper.push_str("    napi_value this_arg;\n");
        wrapper.push_str("    void* data;\n");
        wrapper.push_str("    NAPI_CALL(env, napi_get_cb_info(env, info, &argc, argv, &this_arg, &data));\n\n");
        wrapper.push_str("    aly_value_t args[10];\n");
        wrapper.push_str("    for (size_t i = 0; i < argc; i++) {\n");
        wrapper.push_str("        napi_valuetype type;\n");
        wrapper.push_str("        NAPI_CALL(env, napi_typeof(env, argv[i], &type));\n");
        wrapper.push_str("        switch (type) {\n");
        wrapper.push_str("            case napi_number: {\n");
        wrapper.push_str("                double val;\n");
        wrapper.push_str("                NAPI_CALL(env, napi_get_value_double(env, argv[i], &val));\n");
        wrapper.push_str("                args[i] = aly_float(val);\n");
        wrapper.push_str("                break;\n");
        wrapper.push_str("            }\n");
        wrapper.push_str("            case napi_string: {\n");
        wrapper.push_str("                size_t len;\n");
        wrapper.push_str("                NAPI_CALL(env, napi_get_value_string_utf8(env, argv[i], NULL, 0, &len));\n");
        wrapper.push_str("                char* buf = (char*)malloc(len + 1);\n");
        wrapper.push_str("                NAPI_CALL(env, napi_get_value_string_utf8(env, argv[i], buf, len + 1, &len));\n");
        wrapper.push_str("                args[i] = aly_string(buf);\n");
        wrapper.push_str("                free(buf);\n");
        wrapper.push_str("                break;\n");
        wrapper.push_str("            }\n");
        wrapper.push_str("            case napi_boolean: {\n");
        wrapper.push_str("                bool val;\n");
        wrapper.push_str("                NAPI_CALL(env, napi_get_value_bool(env, argv[i], &val));\n");
        wrapper.push_str("                args[i] = aly_bool(val ? 1 : 0);\n");
        wrapper.push_str("                break;\n");
        wrapper.push_str("            }\n");
        wrapper.push_str("            default:\n");
        wrapper.push_str("                args[i] = aly_none();\n");
        wrapper.push_str("        }\n");
        wrapper.push_str("    }\n\n");
        wrapper.push_str(&format!("    aly_value_t result = {}(argc, args);\n", func.name));
        wrapper.push_str("\n    // Convert result back to JS\n");
        wrapper.push_str("    napi_value js_result;\n");
        wrapper.push_str("    switch (result.type) {\n");
        wrapper.push_str("        case ALY_INT:\n");
        wrapper.push_str("            NAPI_CALL(env, napi_create_int32(env, (int32_t)result.as.int_val, &js_result));\n");
        wrapper.push_str("            break;\n");
        wrapper.push_str("        case ALY_FLOAT:\n");
        wrapper.push_str("            NAPI_CALL(env, napi_create_double(env, result.as.float_val, &js_result));\n");
        wrapper.push_str("            break;\n");
        wrapper.push_str("        case ALY_BOOL:\n");
        wrapper.push_str("            NAPI_CALL(env, napi_get_boolean(env, result.as.bool_val, &js_result));\n");
        wrapper.push_str("            break;\n");
        wrapper.push_str("        case ALY_STRING:\n");
        wrapper.push_str("            NAPI_CALL(env, napi_create_string_utf8(env, result.as.str_val, NAPI_AUTO_LENGTH, &js_result));\n");
        wrapper.push_str("            break;\n");
        wrapper.push_str("        default:\n");
        wrapper.push_str("            NAPI_CALL(env, napi_get_undefined(env, &js_result));\n");
        wrapper.push_str("    }\n");
        wrapper.push_str("    return js_result;\n");
        wrapper.push_str("}\n\n");
        wrapper
    }

    fn generate_init(&self) -> String {
        let mut init = format!(
            "napi_value Init(napi_env env, napi_value exports) {{\n"
        );
        for func in &self.functions {
            init.push_str(&format!(
                "    napi_value fn_{};\n", func.name
            ));
            init.push_str(&format!(
                "    NAPI_CALL(env, napi_create_function(env, \"{}\", NAPI_AUTO_LENGTH, js_{}, NULL, &fn_{}));\n",
                func.name, func.name, func.name
            ));
            init.push_str(&format!(
                "    NAPI_CALL(env, napi_set_named_property(env, exports, \"{}\", fn_{}));\n",
                func.name, func.name
            ));
        }
        init.push_str("    return exports;\n");
        init.push_str("}\n\n");
        init.push_str("NAPI_MODULE(NODE_GYP_MODULE_NAME, Init)\n");
        init
    }

    pub fn generate_package_json(&self) -> String {
        format!(
            r#"{{
    "name": "{}",
    "version": "0.1.0",
    "main": "index.js",
    "scripts": {{
        "build": "node-gyp rebuild",
        "test": "node test.js"
    }},
    "gypfile": true
}}
"#,
            self.module_name
        )
    }

    pub fn generate_binding_gyp(&self) -> String {
        format!(
            r#"{{
    "targets": [
        {{
            "target_name": "{}",
            "sources": [ "binding.c" ],
            "include_dirs": [ "<!(node -e \"require('node-api-headers')\")" ],
            "dependencies": [ "<!(node -p \"require('node-api-headers').gyp\"):node-api-headers" ]
        }}
    ]
}}
"#,
            self.module_name
        )
    }

    pub fn generate_js_wrapper(&self) -> String {
        let mut js = format!(
            "const {} = require('bindings')('{}');\n\n",
            self.module_name, self.module_name
        );
        for func in &self.functions {
            js.push_str(&format!(
                "module.exports.{} = function({}) {{\n",
                func.name,
                (0..func.param_count).map(|i| format!("arg{}", i)).collect::<Vec<_>>().join(", ")
            ));
            if func.variadic {
                js.push_str("    // Variadic: pass arguments as array\n");
                js.push_str("    return native.aly_call('");
                js.push_str(&func.name);
                js.push_str("', arguments);\n");
            } else {
                js.push_str("    return native.aly_call('");
                js.push_str(&func.name);
                js.push_str("'");
                for i in 0..func.param_count {
                    js.push_str(&format!(", arg{}", i));
                }
                js.push_str(");\n");
            }
            js.push_str("};\n\n");
        }
        js
    }
}
