pub struct JavaBindings {
    package_name: String,
    class_name: String,
    methods: Vec<JavaMethod>,
}

struct JavaMethod {
    name: String,
    params: Vec<JavaType>,
    return_type: JavaType,
    static_method: bool,
}

pub enum JavaType {
    Int,
    Float,
    Bool,
    String,
    Void,
    Object(String),
}

impl JavaType {
    fn jni_sig(&self) -> &str {
        match self {
            JavaType::Int => "I",
            JavaType::Float => "F",
            JavaType::Bool => "Z",
            JavaType::String => "Ljava/lang/String;",
            JavaType::Void => "V",
            JavaType::Object(s) => {
                // simplified: would need class path
                "Ljava/lang/Object;"
            }
        }
    }

    fn jni_type(&self) -> &str {
        match self {
            JavaType::Int => "jint",
            JavaType::Float => "jfloat",
            JavaType::Bool => "jboolean",
            JavaType::String => "jstring",
            JavaType::Void => "void",
            JavaType::Object(_) => "jobject",
        }
    }
}

impl JavaBindings {
    pub fn new(package: &str, class: &str) -> Self {
        JavaBindings {
            package_name: package.to_string(),
            class_name: class.to_string(),
            methods: Vec::new(),
        }
    }

    pub fn add_method(&mut self, name: &str, params: Vec<JavaType>, return_type: JavaType, static_method: bool) {
        self.methods.push(JavaMethod { name: name.to_string(), params, return_type, static_method });
    }

    pub fn generate_c_header(&self) -> String {
        let mut h = format!(
            "/* JNI header for {}.{} */\n",
            self.package_name.replace('.', "/"),
            self.class_name
        );
        h.push_str("#include <jni.h>\n");
        h.push_str("#include \"runtime_aly.h\"\n\n");
        h.push_str("#ifndef _Included_");
        h.push_str(&self.class_name);
        h.push_str("\n#define _Included_");
        h.push_str(&self.class_name);
        h.push_str("\n\n");

        let jni_path = self.jni_path();
        for method in &self.methods {
            let sig = self.jni_method_signature(method);
            h.push_str(&format!(
                "JNIEXPORT {} JNICALL Java_{}_{}(JNIEnv *, {});\n",
                method.return_type.jni_type(),
                jni_path,
                method.name,
                if method.static_method { "jclass" } else { "jobject" }
            ));
        }
        h.push_str("\n#endif\n");
        h
    }

    pub fn generate_c_source(&self) -> String {
        let mut c = String::new();
        c.push_str(&format!(
            "/* JNI implementation for {}.{} */\n",
            self.package_name.replace('.', "/"),
            self.class_name
        ));
        c.push_str("#include <jni.h>\n");
        c.push_str("#include \"runtime_aly.h\"\n");
        c.push_str(&format!("#include \"{}\"\n\n", self.class_name.to_lowercase()));

        let jni_path = self.jni_path();
        for method in &self.methods {
            c.push_str(&self.generate_method_impl(method, &jni_path));
        }
        c
    }

    fn generate_method_impl(&self, method: &JavaMethod, jni_path: &str) -> String {
        let mut impl_code = format!(
            "JNIEXPORT {} JNICALL Java_{}_{}(JNIEnv *env, {}",
            method.return_type.jni_type(),
            jni_path,
            method.name,
            if method.static_method { "jclass clazz" } else { "jobject this_" }
        );
        for (i, param) in method.params.iter().enumerate() {
            impl_code.push_str(&format!(", {} arg{}", param.jni_type(), i));
        }
        impl_code.push_str(") {\n");

        // Convert JNI types to Aly values
        let mut aly_args = String::new();
        for (i, param) in method.params.iter().enumerate() {
            aly_args.push_str(&format!("    aly_value_t aly_arg{};\n", i));
            match param {
                JavaType::Int => {
                    aly_args.push_str(&format!("    aly_arg{} = aly_int(arg{});\n", i, i));
                }
                JavaType::Float => {
                    aly_args.push_str(&format!("    aly_arg{} = aly_float((double)arg{});\n", i, i));
                }
                JavaType::Bool => {
                    aly_args.push_str(&format!("    aly_arg{} = aly_bool(arg{});\n", i, i));
                }
                JavaType::String => {
                    aly_args.push_str(&format!(
                        "    const char* str{} = (*env)->GetStringUTFChars(env, arg{}, NULL);\n", i, i
                    ));
                    aly_args.push_str(&format!("    aly_arg{} = aly_string(str{});\n", i, i));
                    aly_args.push_str(&format!("    (*env)->ReleaseStringUTFChars(env, arg{}, str{});\n", i, i));
                }
                _ => {
                    aly_args.push_str(&format!("    aly_arg{} = aly_none();\n", i));
                }
            }
        }

        // Build call
        let mut call = format!("    aly_value_t args[{}];\n", method.params.len());
        for i in 0..method.params.len() {
            call.push_str(&format!("    args[{}] = aly_arg{};\n", i, i));
        }
        call.push_str(&format!(
            "    aly_value_t result = {}({}, args);\n",
            method.name, method.params.len()
        ));

        // Convert return
        let mut ret = String::new();
        match method.return_type {
            JavaType::Void => {
                ret.push_str("    return;\n");
            }
            JavaType::Int => {
                ret.push_str("    return result.as.int_val;\n");
            }
            JavaType::Float => {
                ret.push_str("    return (jfloat)result.as.float_val;\n");
            }
            JavaType::Bool => {
                ret.push_str("    return result.as.bool_val ? JNI_TRUE : JNI_FALSE;\n");
            }
            JavaType::String => {
                ret.push_str("    return (*env)->NewStringUTF(env, result.as.str_val);\n");
            }
            JavaType::Object(_) => {
                ret.push_str("    return NULL;\n");
            }
        }

        impl_code.push_str(&aly_args);
        impl_code.push_str(&call);
        impl_code.push_str(&ret);
        impl_code.push_str("}\n\n");
        impl_code
    }

    pub fn generate_java_class(&self) -> String {
        let mut java = format!(
            "package {};\n\n",
            self.package_name
        );
        java.push_str(&format!(
            "public class {} {{\n",
            self.class_name
        ));
        java.push_str("    static {\n");
        java.push_str(&format!(
            "        System.loadLibrary(\"{}_native\");\n",
            self.class_name.to_lowercase()
        ));
        java.push_str("    }\n\n");

        for method in &self.methods {
            let mut sig = "    public ";
            if method.static_method {
                sig = "    public static ";
            }
            java.push_str(&format!(
                "{}native {} {}({})",
                sig,
                self.java_type_str(&method.return_type),
                method.name,
                method.params.iter()
                    .map(|p| self.java_type_str(p))
                    .collect::<Vec<_>>()
                    .join(", ")
            ));
            java.push_str(";\n\n");
        }

        java.push_str("}\n");
        java
    }

    fn jni_path(&self) -> String {
        self.package_name.replace('.', "_") + "_" + &self.class_name
    }

    fn jni_method_signature(&self, method: &JavaMethod) -> String {
        let mut sig = "(".to_string();
        for p in &method.params {
            sig.push_str(p.jni_sig());
        }
        sig.push(')');
        sig.push_str(method.return_type.jni_sig());
        sig
    }

    fn java_type_str(&self, ty: &JavaType) -> &str {
        match ty {
            JavaType::Int => "int",
            JavaType::Float => "float",
            JavaType::Bool => "boolean",
            JavaType::String => "String",
            JavaType::Void => "void",
            JavaType::Object(s) => "Object",
        }
    }

    pub fn generate_build_script(&self) -> String {
        format!(
            r#"#!/bin/bash
# Build script for {} JNI bindings

JAVA_HOME=${{JAVA_HOME:-/usr/lib/jvm/default-java}}
gcc -shared -fPIC -o lib{}_native.so \
    -I"$JAVA_HOME/include" \
    -I"$JAVA_HOME/include/linux" \
    -I. \
    {}_jni.c \
    -lm

echo "Build complete: lib{}_native.so"
"#,
            self.class_name,
            self.class_name.to_lowercase(),
            self.class_name.to_lowercase(),
            self.class_name.to_lowercase()
        )
    }
}
