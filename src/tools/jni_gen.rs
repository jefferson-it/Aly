use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};

#[derive(Debug, Clone)]
pub struct JniBinding {
    pub java_class: String,
    pub java_method: String,
    pub java_signature: String,
    pub aly_function: String,
    pub return_type: String,
    pub params: Vec<JniParam>,
}

#[derive(Debug, Clone)]
pub struct JniParam {
    pub name: String,
    pub java_type: String,
    pub aly_type: String,
}

#[derive(Debug, Clone)]
pub struct JniClassBinding {
    pub java_class: String,
    pub package: String,
    pub methods: Vec<JniBinding>,
    pub fields: Vec<JniField>,
}

#[derive(Debug, Clone)]
pub struct JniField {
    pub name: String,
    pub java_type: String,
    pub aly_type: String,
    pub is_static: bool,
}

pub struct JniGenerator {
    bindings: Vec<JniClassBinding>,
    output_dir: PathBuf,
    package_name: String,
}

impl JniGenerator {
    pub fn new(output_dir: &str, package_name: &str) -> Self {
        Self {
            bindings: Vec::new(),
            output_dir: PathBuf::from(output_dir),
            package_name: package_name.to_string(),
        }
    }

    pub fn add_class(&mut self, binding: JniClassBinding) {
        self.bindings.push(binding);
    }

    pub fn generate(&self) -> Result<(), Box<dyn std::error::Error>> {
        fs::create_dir_all(&self.output_dir)?;

        for binding in &self.bindings {
            self.generate_class_binding(binding)?;
        }

        self.generate_bridge_file()?;
        self.generate_gradle_config()?;
        self.generate_android_manifest()?;
        self.generate_main_activity()?;
        self.generate_aly_runtime_class()?;

        Ok(())
    }

    fn generate_class_binding(&self, binding: &JniClassBinding) -> Result<(), Box<dyn std::error::Error>> {
        let class_name = binding.java_class.split('/').last().unwrap_or(&binding.java_class);
        let file_name = format!("{}.rs", class_name.to_lowercase());
        let file_path = self.output_dir.join("bindings").join(&file_name);

        fs::create_dir_all(file_path.parent().unwrap())?;

        let mut content = String::new();
        content.push_str(&format!("// Auto-generated JNI bindings for {}\n", binding.java_class));
        content.push_str("// DO NOT EDIT MANUALLY\n\n");
        content.push_str("use crate::native::jni::*;\nuse crate::native::jni::android::*;\nuse crate::error::AlyError;\nuse crate::native::types::ValueData;\n\n");

        content.push_str(&format!("pub struct {};\n\n", class_name));
        content.push_str(&format!("impl {} {{\n", class_name));
        content.push_str(&format!("    const CLASS_NAME: &str = \"{}\";\n\n", binding.java_class));

        for method in &binding.methods {
            content.push_str(&self.generate_method_binding(method, class_name));
        }

        for field in &binding.fields {
            content.push_str(&self.generate_field_binding(field, class_name));
        }

        content.push_str("}\n");

        fs::write(&file_path, content)?;
        println!("Generated: {}", file_path.display());
        Ok(())
    }

    fn generate_method_binding(&self, method: &JniBinding, class_name: &str) -> String {
        let params: Vec<String> = method.params.iter()
            .map(|p| format!("{}: {}", p.name, self.java_type_to_rust(&p.java_type)))
            .collect();

        let return_type = self.java_type_to_rust(&method.return_type);
        let jni_args: Vec<String> = method.params.iter()
            .map(|p| self.rust_to_jni_arg(&p.name, &p.java_type))
            .collect();

        let static_mod = if method.java_signature.starts_with("static ") { "static " } else { "" };
        let sig = method.java_signature.replace("static ", "");

        format!(
            "    pub {mod}fn {name}({params}) -> Result<{ret}, AlyError> {{\n        let bridge = JniBridge::new(Self::CLASS_NAME, \"{method_name}\", \"{sig}\");\n        let args = vec![{jni_args}];\n        bridge.call_static(args)\n    }}\n\n",
            mod = static_mod,
            name = method.aly_function,
            params = params.join(", "),
            ret = return_type,
            method_name = method.java_method,
            sig = sig,
            jni_args = jni_args.join(", "),
        )
    }

    fn generate_field_binding(&self, field: &JniField, class_name: &str) -> String {
        let field_name = &field.name;
        let rust_type = self.java_type_to_rust(&field.java_type);
        let jni_sig = self.java_type_to_jni_signature(&field.java_type);

        if field.is_static {
            format!(
                "    pub static fn get_{name}() -> Result<{ret}, AlyError> {{\n        let bridge = JniBridge::new(Self::CLASS_NAME, \"{name}\", \"{sig}\");\n        bridge.get_static_field(\"{name}\", \"{sig}\")\n    }}\n\n    pub static fn set_{name}(value: {ret}) -> Result<(), AlyError> {{\n        let bridge = JniBridge::new(Self::CLASS_NAME, \"{name}\", \"{sig}\");\n        bridge.set_static_field(\"{name}\", \"{sig}\", value)\n    }}\n\n",
                name = field_name,
                ret = rust_type,
                sig = jni_sig,
            )
        } else {
            format!(
                "    pub fn get_{name}(&self) -> Result<{ret}, AlyError> {{\n        // Instance field access requires object reference\n        unimplemented!()\n    }}\n\n    pub fn set_{name}(&self, value: {ret}) -> Result<(), AlyError> {{\n        // Instance field access requires object reference\n        unimplemented!()\n    }}\n\n",
                name = field_name,
                ret = rust_type,
            )
        }
    }

    fn java_type_to_rust(&self, java_type: &str) -> String {
        match java_type {
            "java.lang.String" => "String".to_string(),
            "int" | "java.lang.Integer" => "i32".to_string(),
            "long" | "java.lang.Long" => "i64".to_string(),
            "boolean" | "java.lang.Boolean" => "bool".to_string(),
            "double" | "java.lang.Double" => "f64".to_string(),
            "float" | "java.lang.Float" => "f32".to_string(),
            "byte" | "java.lang.Byte" => "i8".to_string(),
            "short" | "java.lang.Short" => "i16".to_string(),
            "char" | "java.lang.Character" => "char".to_string(),
            "void" => "()".to_string(),
            _ if java_type.ends_with("[]") => format!("Vec<{}>", self.java_type_to_rust(&java_type[..java_type.len()-2])),
            _ if java_type.starts_with("java.util.List") => "Vec<ValueData>".to_string(),
            _ if java_type.starts_with("java.util.Map") => "HashMap<String, ValueData>".to_string(),
            _ if java_type.starts_with("java.util.Set") => "Vec<ValueData>".to_string(),
            _ => format!("JObject<{}>", java_type.replace('/', "_")),
        }
    }

    fn java_type_to_jni_signature(&self, java_type: &str) -> String {
        match java_type {
            "java.lang.String" => "Ljava/lang/String;".to_string(),
            "int" | "java.lang.Integer" => "I".to_string(),
            "long" | "java.lang.Long" => "J".to_string(),
            "boolean" | "java.lang.Boolean" => "Z".to_string(),
            "double" | "java.lang.Double" => "D".to_string(),
            "float" | "java.lang.Float" => "F".to_string(),
            "byte" | "java.lang.Byte" => "B".to_string(),
            "short" | "java.lang.Short" => "S".to_string(),
            "char" | "java.lang.Character" => "C".to_string(),
            "void" => "V".to_string(),
            _ if java_type.ends_with("[]") => format!("[{}", self.java_type_to_jni_signature(&java_type[..java_type.len()-2])),
            _ => format!("L{};", java_type.replace('.', "/")),
        }
    }

    fn rust_to_jni_arg(&self, name: &str, java_type: &str) -> String {
        match java_type {
            "java.lang.String" => format!("ValueData::String({}.to_string())", name),
            "int" | "java.lang.Integer" => format!("ValueData::Number({} as f64)", name),
            "long" | "java.lang.Long" => format!("ValueData::Number({} as f64)", name),
            "boolean" | "java.lang.Boolean" => format!("ValueData::Bool({})", name),
            "double" | "java.lang.Double" => format!("ValueData::Number({})", name),
            "float" | "java.lang.Float" => format!("ValueData::Number({} as f64)", name),
            _ => format!("ValueData::String({}.to_string())", name),
        }
    }

    fn generate_bridge_file(&self) -> Result<(), Box<dyn std::error::Error>> {
        let mut content = String::new();
        content.push_str("// Auto-generated JNI bridge\n");
        content.push_str("// DO NOT EDIT MANUALLY\n\n");
        content.push_str("use crate::native::jni::*;\nuse crate::error::AlyError;\nuse crate::native::types::ValueData;\n\n");

        content.push_str("pub struct AlyJniBridge;\n\n");
        content.push_str("impl AlyJniBridge {\n");

        for binding in &self.bindings {
            let class_name = binding.java_class.split('/').last().unwrap_or(&binding.java_class);
            content.push_str(&format!("    pub fn {}() -> {} {{\n", class_name.to_lowercase(), class_name));
            content.push_str(&format!("        {}::new()\n", class_name));
            content.push_str("    }\n\n");
        }

        content.push_str("}\n");

        let file_path = self.output_dir.join("bridge.rs");
        fs::write(file_path, content)?;
        Ok(())
    }

    fn generate_gradle_config(&self) -> Result<(), Box<dyn std::error::Error>> {
        let gradle_content = format!(
            r#"// build.gradle.kts (app module)
plugins {{
    id("com.android.application")
    id("org.jetbrains.kotlin.android")
    id("com.android.library")
}}

android {{
    namespace = "{}"
    compileSdk = 34

    defaultConfig {{
        applicationId = "{}"
        minSdk = 24
        targetSdk = 34
        versionCode = 1
        versionName = "1.0"

        testInstrumentationRunner = "androidx.test.runner.AndroidJUnitRunner"
        
        externalNativeBuild {{
            cmake {{
                cppFlags.add("-std=c++17")
                arguments("-DANDROID_STL=c++_shared")
            }}
        }}
        
        ndk {{
            abiFilters.addAll(listOf("arm64-v8a", "x86_64"))
        }}
    }}

    buildTypes {{
        release {{
            isMinifyEnabled = false
            proguardFiles(
                getDefaultProguardFile("proguard-android-optimize.txt"),
                "proguard-rules.pro"
            )
        }}
    }}
    
    compileOptions {{
        sourceCompatibility = JavaVersion.VERSION_17
        targetCompatibility = JavaVersion.VERSION_17
    }}
    
    kotlinOptions {{
        jvmTarget = "17"
    }}
    
    externalNativeBuild {{
        cmake {{
            path = "CMakeLists.txt"
            version = "3.22.1"
        }}
    }}
    
    packagingOptions {{
        jniLibs {{
            useLegacyPackaging = true
        }}
    }}
}}

dependencies {{
    implementation("androidx.core:core-ktx:1.12.0")
    implementation("androidx.appcompat:appcompat:1.6.1")
    implementation("com.google.android.material:material:1.11.0")
    implementation("androidx.constraintlayout:constraintlayout:2.1.4")
    implementation("androidx.lifecycle:lifecycle-runtime-ktx:2.7.0")
    implementation("androidx.activity:activity-compose:1.8.2")
    implementation(platform("androidx.compose:compose-bom:2024.02.00"))
    implementation("androidx.compose.ui:ui")
    implementation("androidx.compose.ui:ui-graphics")
    implementation("androidx.compose.ui:ui-tooling-preview")
    implementation("androidx.compose.material3:material3")
    
    testImplementation("junit:junit:4.13.2")
    androidTestImplementation("androidx.test.ext:junit:1.1.5")
    androidTestImplementation("androidx.test.espresso:espresso-core:3.5.1")
    androidTestImplementation(platform("androidx.compose:compose-bom:2024.02.00"))
    androidTestImplementation("androidx.compose.ui:ui-test-junit4")
    debugImplementation("androidx.compose.ui:ui-tooling")
    debugImplementation("androidx.compose.ui:ui-test-manifest")
}}

// Rust dependencies (in Cargo.toml)
/*
[dependencies]
jni = {{ version = "0.21", default-features = false }}
android-activity = {{ version = "0.5", default-features = false }}
android_logger = "0.16"
jni-sys = {{ version = "0.3", default-features = false }}
obtain = "0.5"
*/

// CMakeLists.txt
/*
cmake_minimum_required(VERSION 3.22.1)
project("aly_android" LANGUAGES CXX)

set(CMAKE_CXX_STANDARD 17)
set(CMAKE_CXX_STANDARD_REQUIRED ON)

add_library(
    aly_rust
    SHARED
    IMPORTED
)

set_target_properties(
    aly_rust
    PROPERTIES
    IMPORTED_LOCATION ${{CMAKE_SOURCE_DIR}}/../target/{{ANDROID_ABI}}/libaly_rust.so
)

add_library(
    aly_jni
    SHARED
    src/main/cpp/aly_jni.cpp
)

target_link_libraries(
    aly_jni
    aly_rust
    log
    android
)
*/
"#,
            self.package_name, self.package_name
        );

        let gradle_dir = self.output_dir.join("android").join("app");
        fs::create_dir_all(&gradle_dir)?;
        fs::write(gradle_dir.join("build.gradle.kts"), gradle_content)?;
        Ok(())
    }

    fn generate_android_manifest(&self) -> Result<(), Box<dyn std::error::Error>> {
        let manifest_content = format!(
            r#"<?xml version="1.0" encoding="utf-8"?>
<manifest xmlns:android="http://schemas.android.com/apk/res/android"
    package="{}">

    <uses-permission android:name="android.permission.INTERNET" />
    <uses-permission android:name="android.permission.ACCESS_NETWORK_STATE" />
    <uses-permission android:name="android.permission.ACCESS_WIFI_STATE" />
    <uses-permission android:name="android.permission.BLUETOOTH" />
    <uses-permission android:name="android.permission.BLUETOOTH_ADMIN" />
    <uses-permission android:name="android.permission.BLUETOOTH_CONNECT" />
    <uses-permission android:name="android.permission.BLUETOOTH_SCAN" />
    <uses-permission android:name="android.permission.NFC" />
    <uses-permission android:name="android.permission.CAMERA" />
    <uses-permission android:name="android.permission.ACCESS_FINE_LOCATION" />
    <uses-permission android:name="android.permission.ACCESS_COARSE_LOCATION" />
    <uses-permission android:name="android.permission.READ_EXTERNAL_STORAGE" />
    <uses-permission android:name="android.permission.WRITE_EXTERNAL_STORAGE" />
    <uses-permission android:name="android.permission.RECORD_AUDIO" />
    <uses-permission android:name="android.permission.MODIFY_AUDIO_SETTINGS" />
    <uses-permission android:name="android.permission.VIBRATE" />
    <uses-permission android:name="android.permission.WAKE_LOCK" />
    <uses-permission android:name="android.permission.FOREGROUND_SERVICE" />
    <uses-permission android:name="android.permission.RECEIVE_BOOT_COMPLETED" />
    <uses-permission android:name="android.permission.SEND_SMS" />
    <uses-permission android:name="android.permission.RECEIVE_SMS" />
    <uses-permission android:name="android.permission.READ_SMS" />
    <uses-permission android:name="android.permission.CALL_PHONE" />
    <uses-permission android:name="android.permission.READ_PHONE_STATE" />
    <uses-permission android:name="android.permission.READ_CONTACTS" />
    <uses-permission android:name="android.permission.WRITE_CONTACTS" />
    <uses-permission android:name="android.permission.GET_ACCOUNTS" />
    <uses-permission android:name="android.permission.USE_FINGERPRINT" />
    <uses-permission android:name="android.permission.USE_BIOMETRIC" />
    <uses-permission android:name="android.permission.BODY_SENSORS" />
    <uses-permission android:name="android.permission.HIGH_SAMPLING_RATE_SENSORS" />

    <application
        android:name=".AlyApplication"
        android:allowBackup="true"
        android:dataExtractionRules="@xml/data_extraction_rules"
        android:fullBackupContent="@xml/backup_rules"
        android:icon="@mipmap/ic_launcher"
        android:label="@string/app_name"
        android:roundIcon="@mipmap/ic_launcher_round"
        android:supportsRtl="true"
        android:theme="@style/Theme.AlyApp"
        android:usesCleartextTraffic="true"
        android:hardwareAccelerated="true"
        android:largeHeap="true"
        android:extractNativeLibs="true">
        
        <activity
            android:name=".MainActivity"
            android:exported="true"
            android:launchMode="singleTop"
            android:theme="@style/Theme.AlyApp.NoActionBar"
            android:configChanges="orientation|screenSize|keyboardHidden|density"
            android:windowSoftInputMode="adjustResize">
            <intent-filter>
                <action android:name="android.intent.action.MAIN" />
                <category android:name="android.intent.category.LAUNCHER" />
            </intent-filter>
        </activity>

        <activity
            android:name=".AlyActivity"
            android:exported="false"
            android:theme="@style/Theme.AlyApp.NoActionBar"
            android:configChanges="orientation|screenSize|keyboardHidden|density"
            android:windowSoftInputMode="adjustResize" />

        <meta-data
            android:name="android.app.lib_name"
            android:value="aly_rust" />
            
        <meta-data
            android:name="com.aly.lang.version"
            android:value="1.0.0" />
            
    </application>

</manifest>"#,
            self.package_name
        );

        let manifest_dir = self.output_dir.join("android").join("app").join("src").join("main");
        fs::create_dir_all(&manifest_dir)?;
        fs::write(manifest_dir.join("AndroidManifest.xml"), manifest_content)?;
        Ok(())
    }

    fn generate_main_activity(&self) -> Result<(), Box<dyn std::error::Error>> {
        let activity_content = format!(
            r#"package {};

import android.os.Bundle
import android.util.Log
import androidx.appcompat.app.AppCompatActivity
import androidx.lifecycle.lifecycleScope
import kotlinx.coroutines.launch
import com.aly.lang.AlyRuntime
import com.aly.lang.AlyView

class MainActivity : AppCompatActivity() {{
    private val TAG = "AlyMainActivity"
    private lateinit var alyRuntime: AlyRuntime
    private lateinit var alyView: AlyView

    override fun onCreate(savedInstanceState: Bundle?) {{
        super.onCreate(savedInstanceState)
        
        // Initialize Aly Runtime
        alyRuntime = AlyRuntime.getInstance(this)
        
        // Create Aly View
        alyView = AlyView(this)
        setContentView(alyView)
        
        // Load Aly script
        lifecycleScope.launch {{
            try {{
                val script = loadScript("main.aly")
                alyRuntime.eval(script)
            }} catch (e: Exception) {{
                Log.e(TAG, "Failed to load Aly script", e)
            }}
        }}
    }}

    override fun onDestroy() {{
        super.onDestroy()
        alyRuntime.shutdown()
    }}

    private suspend fun loadScript(name: String): String {{
        return withContext(io.ktor.client.engine.okhttp.OkHttp) {{
            assets.open(name).bufferedReader().readText()
        }}
    }}
}}"#,
            self.package_name
        );

        let activity_dir = self.output_dir.join("android").join("app").join("src").join("main").join("kotlin").join(self.package_name.replace('.', "/"));
        fs::create_dir_all(&activity_dir)?;
        fs::write(activity_dir.join("MainActivity.kt"), activity_content)?;
        Ok(())
    }

    fn generate_aly_runtime_class(&self) -> Result<(), Box<dyn std::error::Error>> {
        let runtime_content = format!(
            r#"package {};

import android.content.Context
import android.util.Log
import com.aly.lang.AlyRuntimeJNI

class AlyRuntime private constructor(context: Context) {{
    companion object {{
        @Volatile private var instance: AlyRuntime? = null
        
        fun getInstance(context: Context): AlyRuntime {{
            return instance ?: synchronized(this) {{
                instance ?: AlyRuntime(context.applicationContext).also {{ instance = it }}
            }}
        }}
    }}
    
    private val jni: AlyRuntimeJNI = AlyRuntimeJNI()
    
    init {{
        jni.init(context)
    }}
    
    fun eval(code: String): String {{
        return jni.eval(code)
    }}
    
    fun callFunction(name: String, vararg args: String): String {{
        return jni.callFunction(name, *args)
    }}
    
    fun shutdown() {{
        jni.shutdown()
        instance = null
    }}
}}"#,
            self.package_name
        );

        let runtime_dir = self.output_dir.join("android").join("app").join("src").join("main").join("kotlin").join(self.package_name.replace('.', "/"));
        fs::create_dir_all(&runtime_dir)?;
        fs::write(runtime_dir.join("AlyRuntime.kt"), runtime_content)?;
        Ok(())
    }
}

pub fn generate_android_project(
    output_dir: &str,
    package_name: &str,
    project_name: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    let mut generator = JniGenerator::new(output_dir, package_name);

    generator.add_class(JniClassBinding {
        java_class: "android/app/Activity".to_string(),
        package: "android.app".to_string(),
        methods: vec![
            JniBinding {
                java_class: "android/app/Activity".to_string(),
                java_method: "getSystemService".to_string(),
                java_signature: "(Ljava/lang/String;)Ljava/lang/Object;".to_string(),
                aly_function: "get_system_service".to_string(),
                return_type: "java.lang.Object".to_string(),
                params: vec![
                    JniParam {
                        name: "name".to_string(),
                        java_type: "java.lang.String".to_string(),
                        aly_type: "String".to_string(),
                    },
                ],
            },
            JniBinding {
                java_class: "android/app/Activity".to_string(),
                java_method: "startActivity".to_string(),
                java_signature: "(Landroid/content/Intent;)V".to_string(),
                aly_function: "start_activity".to_string(),
                return_type: "void".to_string(),
                params: vec![
                    JniParam {
                        name: "intent".to_string(),
                        java_type: "android.content.Intent".to_string(),
                        aly_type: "Intent".to_string(),
                    },
                ],
            },
        ],
        fields: vec![],
    });

    generator.add_class(JniClassBinding {
        java_class: "android/content/Context".to_string(),
        package: "android.content".to_string(),
        methods: vec![
            JniBinding {
                java_class: "android/content/Context".to_string(),
                java_method: "getPackageName".to_string(),
                java_signature: "()Ljava/lang/String;".to_string(),
                aly_function: "get_package_name".to_string(),
                return_type: "java.lang.String".to_string(),
                params: vec![],
            },
            JniBinding {
                java_class: "android/content/Context".to_string(),
                java_method: "getFilesDir".to_string(),
                java_signature: "()Ljava/io/File;".to_string(),
                aly_function: "get_files_dir".to_string(),
                return_type: "java.io.File".to_string(),
                params: vec![],
            },
        ],
        fields: vec![],
    });

    generator.add_class(JniClassBinding {
        java_class: "android/widget/Toast".to_string(),
        package: "android.widget".to_string(),
        methods: vec![
            JniBinding {
                java_class: "android/widget/Toast".to_string(),
                java_method: "makeText".to_string(),
                java_signature: "static (Landroid/content/Context;Ljava/lang/CharSequence;I)Landroid/widget/Toast;".to_string(),
                aly_function: "make_toast".to_string(),
                return_type: "android.widget.Toast".to_string(),
                params: vec![
                    JniParam {
                        name: "context".to_string(),
                        java_type: "android.content.Context".to_string(),
                        aly_type: "Context".to_string(),
                    },
                    JniParam {
                        name: "text".to_string(),
                        java_type: "java.lang.CharSequence".to_string(),
                        aly_type: "String".to_string(),
                    },
                    JniParam {
                        name: "duration".to_string(),
                        java_type: "int".to_string(),
                        aly_type: "Int".to_string(),
                    },
                ],
            },
        ],
        fields: vec![],
    });

    generator.generate()?;
    
    println!("Android project generated at: {}", output_dir);
    Ok(())
}