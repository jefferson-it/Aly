use std::collections::HashMap;

pub struct CppABI {
    mangled_classes: HashMap<String, ClassInfo>,
}

struct ClassInfo {
    base: Option<String>,
    vtable: Vec<String>,
    fields: Vec<String>,
}

impl CppABI {
    pub fn new() -> Self {
        CppABI { mangled_classes: HashMap::new() }
    }

    pub fn declare_class(&mut self, name: &str, base: Option<&str>, methods: &[&str], fields: &[&str]) {
        self.mangled_classes.insert(name.to_string(), ClassInfo {
            base: base.map(String::from),
            vtable: methods.iter().map(|m| self.mangle(name, m)).collect(),
            fields: fields.iter().map(|f| f.to_string()).collect(),
        });
    }

    pub fn mangle(&self, class: &str, method: &str) -> String {
        format!("_ZN{len}{class}{mlen}{method}E",
            len = class.len(),
            class = class,
            mlen = method.len(),
            method = method)
    }

    pub fn generate_vtable(&self, class: &str) -> String {
        let info = match self.mangled_classes.get(class) {
            Some(i) => i,
            None => return String::new(),
        };
        let mut c = format!("// Vtable for {}\n", class);
        c.push_str(&format!("void* {}_vtable[] = {{\n", class));
        for method in &info.vtable {
            c.push_str(&format!("    (void*)&{},\n", method));
        }
        c.push_str("};\n");
        c
    }

    pub fn generate_class_struct(&self, class: &str) -> String {
        let info = match self.mangled_classes.get(class) {
            Some(i) => i,
            None => return String::new(),
        };
        let mut c = format!("typedef struct {} {{ struct {}_vtable_t* vtable;\n", class, class);
        for field in &info.fields {
            c.push_str(&format!("    aly_value_t {};\n", field));
        }
        c.push_str(&format!("}} {};\n", class));
        c
    }

    pub fn generate_constructor(&self, class: &str) -> String {
        let info = match self.mangled_classes.get(class) {
            Some(i) => i,
            None => return String::new(),
        };
        let mut c = format!("aly_value_t {}(int arg_count, aly_value_t* args) {{\n",
            self.mangle(class, class));
        c.push_str(&format!("    {}* obj = malloc(sizeof({}));\n", class, class));
        c.push_str(&format!("    obj->vtable = ({}{}_vtable_t*){}_vtable;\n", class, class, class));
        if let Some(ref base) = info.base {
            c.push_str(&format!("    // Call base constructor: {}\n", base));
        }
        for (i, field) in info.fields.iter().enumerate() {
            c.push_str(&format!("    obj->{} = i < arg_count ? aly_clone(args[{}]) : aly_none();\n", field, i));
        }
        c.push_str("    aly_value_t result;\n");
        c.push_str("    result.type = ALY_OBJECT;\n");
        c.push_str("    result.as.object_val = (AlyObject*)obj;\n");
        c.push_str("    result.type_tag = NULL;\n");
        c.push_str("    return result;\n");
        c.push_str("}\n\n");
        c
    }

    pub fn generate_abi_header(&self) -> String {
        r#"#ifndef ALY_CPP_ABI_H
#define ALY_CPP_ABI_H

#include "runtime_aly.h"

// Itanium C++ ABI structures
typedef struct __cxa_exception {
    size_t referenceCount;
    void* exceptionType;
    void* exceptionDestructor;
    void* unexpectedHandler;
    void* terminateHandler;
    struct __cxa_exception* nextException;
    int handlerCount;
    int handlerSwitchValue;
    const char* actionRecord;
    const char* catchTemp;
    void* adjustedPtr;
    void* exceptionObject;
    void* thisPadding;
} __cxa_exception;

typedef void* (*__cxa_throw_fn)(void*, void*, void(*)(void*));
typedef void* (*__cxa_begin_catch_fn)(void*);
typedef void  (*__cxa_end_catch_fn)(void);
typedef void* (*__cxa_allocate_exception_fn)(size_t);

#define ALY_CPP_EXCEPTION_TYPE "AlyException"

// RAII helper for exception-safe code
typedef struct AlyScopeGuard {
    void (*cleanup)(void*);
    void* data;
} AlyScopeGuard;

static inline void aly_scope_guard_run(AlyScopeGuard* g) {
    if (g && g->cleanup) g->cleanup(g->data);
}

#define ALY_SCOPE_EXIT(cb, data) \
    AlyScopeGuard ALY_CONCAT(_aly_guard_, __LINE__) = { cb, data }; \
    __attribute__((cleanup(aly_scope_guard_run))) \
    AlyScopeGuard* ALY_CONCAT(_aly_guard_p_, __LINE__) = &ALY_CONCAT(_aly_guard_, __LINE__)

#define ALY_CONCAT(a, b) a##b

// C++ standard library ABI stubs
extern void* __dso_handle;
extern int __cxa_atexit(void (*destructor)(void*), void* obj, void* dso_handle);
extern void __cxa_finalize(void* dso_handle);

#endif
"#.to_string()
    }

    pub fn generate_exception_wrapper(&self, class: &str, method: &str) -> String {
        let mangled = self.mangle(class, method);
        format!(
            r#"aly_value_t {}_wrapper(int arg_count, aly_value_t* args) {{
    AlyScopeGuard _guard = {{ NULL, NULL }};
    aly_value_t result = aly_none();
    // Call the actual method
    result = {}(arg_count, args);
    return result;
}}
"#,
            mangled, mangled
        )
    }

    pub fn generate_inline_method(&self, class: &str, method: &str, body: &str) -> String {
        let mangled = self.mangle(class, method);
        format!(
            r#"aly_value_t {}(int arg_count, aly_value_t* args) {{
    {}
}}
"#,
            mangled, body
        )
    }
}
