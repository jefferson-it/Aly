use std::fs;
use std::path::{Path, PathBuf};
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct TemplateConfig {
    pub package_name: String,
    pub app_name: String,
    pub project_name: String,
    pub min_sdk: i32,
    pub target_sdk: i32,
    pub compile_sdk: i32,
    pub kotlin_version: String,
    pub use_compose: bool,
    pub use_ktx: bool,
    pub use_coroutines: bool,
    pub use_room: bool,
    pub use_retrofit: bool,
    pub use_hilt: bool,
    pub use_navigation: bool,
    pub use_paging: bool,
    pub use_workmanager: bool,
    pub use_datastore: bool,
    pub use_camera_x: bool,
    pub use_ml_kit: bool,
    pub use_firebase: bool,
    pub firebase_services: Vec<String>,
    pub features: Vec<String>,
}

impl Default for TemplateConfig {
    fn default() -> Self {
        Self {
            package_name: "com.example.alyapp".to_string(),
            app_name: "AlyApp".to_string(),
            project_name: "AlyApp".to_string(),
            min_sdk: 24,
            target_sdk: 34,
            compile_sdk: 34,
            kotlin_version: "1.9.20".to_string(),
            use_compose: true,
            use_ktx: true,
            use_coroutines: true,
            use_room: false,
            use_retrofit: false,
            use_hilt: false,
            use_navigation: true,
            use_paging: false,
            use_workmanager: false,
            use_datastore: false,
            use_camera_x: false,
            use_ml_kit: false,
            use_firebase: false,
            firebase_services: vec![],
            features: vec![],
        }
    }
}

pub struct AndroidTemplateGenerator {
    config: TemplateConfig,
    output_dir: PathBuf,
}

impl AndroidTemplateGenerator {
    pub fn new(config: TemplateConfig, output_dir: &str) -> Self {
        Self {
            config,
            output_dir: PathBuf::from(output_dir),
        }
    }

    pub fn generate(&self) -> Result<(), Box<dyn std::error::Error>> {
        fs::create_dir_all(&self.output_dir)?;

        self.generate_settings_gradle()?;
        self.generate_root_build_gradle()?;
        self.generate_app_build_gradle()?;
        self.generate_gradle_properties()?;
        self.generate_android_manifest()?;
        self.generate_main_activity()?;
        self.generate_aly_application()?;
        self.generate_aly_view()?;
        self.generate_aly_runtime()?;
        self.generate_aly_jni()?;
        self.generate_cmake_lists()?;
        self.generate_aly_jni_cpp()?;
        self.generate_resources()?;
        self.generate_aly_script_template()?;
        self.generate_readme()?;
        self.generate_proguard_rules()?;
        self.generate_gitignore()?;

        println!("Android template generated at: {}", self.output_dir.display());
        Ok(())
    }

    fn generate_settings_gradle(&self) -> Result<(), Box<dyn std::error::Error>> {
        let content = format!(
            r#"pluginManagement {{
    repositories {{
        gradlePluginPortal()
        google()
        mavenCentral()
    }}
}}
dependencyResolutionManagement {{
    repositoriesMode.set(RepositoriesMode.FAIL_ON_PROJECT_REPOS)
    repositories {{
        google()
        mavenCentral()
    }}
}}
rootProject.name = "{}"
include(":app")"#,
            self.config.project_name
        );
        fs::write(self.output_dir.join("settings.gradle.kts"), content)?;
        Ok(())
    }

    fn generate_root_build_gradle(&self) -> Result<(), Box<dyn std::error::Error>> {
        let content = format!(
            r#"// Top-level build file where you can add configuration options common to all sub-projects/modules.
plugins {{
    id("com.android.application") version "8.2.0" apply false
    id("org.jetbrains.kotlin.android") version "{}" apply false
    id("com.google.gms.google-services") version "4.4.0" apply false
    id("com.google.firebase.crashlytics") version "2.9.9" apply false
}}"#,
            self.config.kotlin_version
        );
        fs::write(self.output_dir.join("build.gradle.kts"), content)?;
        Ok(())
    }

    fn generate_app_build_gradle(&self) -> Result<(), Box<dyn std::error::Error>> {
        let mut deps = vec![
            "implementation(\"androidx.core:core-ktx:1.12.0\")",
            "implementation(\"androidx.appcompat:appcompat:1.6.1\")",
            "implementation(\"com.google.android.material:material:1.11.0\")",
            "implementation(\"androidx.constraintlayout:constraintlayout:2.1.4\")",
            "implementation(\"androidx.lifecycle:lifecycle-runtime-ktx:2.7.0\")",
            "implementation(\"androidx.activity:activity-compose:1.8.2\")",
            "implementation(platform(\"androidx.compose:compose-bom:2024.02.00\"))",
            "implementation(\"androidx.compose.ui:ui\")",
            "implementation(\"androidx.compose.ui:ui-graphics\")",
            "implementation(\"androidx.compose.ui:ui-tooling-preview\")",
            "implementation(\"androidx.compose.material3:material3\")",
        ];

        if self.config.use_navigation {
            deps.push("implementation(\"androidx.navigation:navigation-compose:2.7.6\")");
        }
        if self.config.use_room {
            deps.push("implementation(\"androidx.room:room-runtime:2.6.1\")");
            deps.push("kapt(\"androidx.room:room-compiler:2.6.1\")");
            deps.push("implementation(\"androidx.room:room-ktx:2.6.1\")");
        }
        if self.config.use_retrofit {
            deps.push("implementation(\"com.squareup.retrofit2:retrofit:2.9.0\")");
            deps.push("implementation(\"com.squareup.retrofit2:converter-gson:2.9.0\")");
            deps.push("implementation(\"com.squareup.okhttp3:logging-interceptor:4.12.0\")");
        }
        if self.config.use_hilt {
            deps.push("implementation(\"com.google.dagger:hilt-android:2.48\")");
            deps.push("kapt(\"com.google.dagger:hilt-compiler:2.48\")");
            deps.push("implementation(\"androidx.hilt:hilt-navigation-compose:1.2.0\")");
        }
        if self.config.use_paging {
            deps.push("implementation(\"androidx.paging:paging-compose:3.2.1\")");
        }
        if self.config.use_workmanager {
            deps.push("implementation(\"androidx.work:work-runtime-ktx:2.9.0\")");
        }
        if self.config.use_datastore {
            deps.push("implementation(\"androidx.datastore:datastore-preferences:1.0.0\")");
        }
        if self.config.use_camera_x {
            deps.push("implementation(\"androidx.camera:camera-core:1.3.1\")");
            deps.push("implementation(\"androidx.camera:camera-camera2:1.3.1\")");
            deps.push("implementation(\"androidx.camera:camera-lifecycle:1.3.1\")");
            deps.push("implementation(\"androidx.camera:camera-view:1.3.1\")");
        }
        if self.config.use_ml_kit {
            deps.push("implementation(\"com.google.mlkit:text-recognition:16.0.0\")");
            deps.push("implementation(\"com.google.mlkit:face-detection:16.1.6\")");
            deps.push("implementation(\"com.google.mlkit:barcode-scanning:16.2.5\")");
        }
        let mut firebase_deps = Vec::new();
        if self.config.use_firebase {
            deps.push("implementation(platform(\"com.google.firebase:firebase-bom:32.7.4\"))");
            deps.push("implementation(\"com.google.firebase:firebase-analytics-ktx\")");
            for service in &self.config.firebase_services {
                firebase_deps.push(format!("implementation(\"com.google.firebase:firebase-{}-ktx\")", service));
            }
            for dep in &firebase_deps {
                deps.push(dep);
            }
        }
        if self.config.use_coroutines {
            deps.push("implementation(\"org.jetbrains.kotlinx:kotlinx-coroutines-android:1.7.3\")");
            deps.push("implementation(\"org.jetbrains.kotlinx:kotlinx-coroutines-play-services:1.7.3\")");
        }

        let deps_str = deps.join("\n    ");

        let content = format!(
            r#"plugins {{
    id("com.android.application")
    id("org.jetbrains.kotlin.android")
    id("com.google.gms.google-services")
    id("kotlin-kapt")
}}

android {{
    namespace = "{}"
    compileSdk = {}

    defaultConfig {{
        applicationId = "{}"
        minSdk = {}
        targetSdk = {}
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
        
        vectorDrawables {{
            useSupportLibrary = true
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
        debug {{
            isDebuggable = true
            isMinifyEnabled = false
        }}
    }}
    
    compileOptions {{
        sourceCompatibility = JavaVersion.VERSION_17
        targetCompatibility = JavaVersion.VERSION_17
    }}
    
    kotlinOptions {{
        jvmTarget = "17"
        freeCompilerArgs.addAll(listOf("-Xopt-in=kotlin.RequiresOptIn"))
    }}
    
    composeOptions {{
        kotlinCompilerExtensionVersion = "1.5.10"
    }}
    
    packagingOptions {{
        resources {{
            excludes.add("/META-INF/*")
        }}
        jniLibs {{
            useLegacyPackaging = true
        }}
    }}
    
    externalNativeBuild {{
        cmake {{
            path = "CMakeLists.txt"
            version = "3.22.1"
        }}
    }}
}}

dependencies {{
    {}
    
    testImplementation("junit:junit:4.13.2")
    androidTestImplementation("androidx.test.ext:junit:1.1.5")
    androidTestImplementation("androidx.test.espresso:espresso-core:3.5.1")
    androidTestImplementation(platform("androidx.compose:compose-bom:2024.02.00"))
    androidTestImplementation("androidx.compose.ui:ui-test-junit4")
    debugImplementation("androidx.compose.ui:ui-tooling")
    debugImplementation("androidx.compose.ui:ui-test-manifest")
}}"#,
            self.config.package_name,
            self.config.compile_sdk,
            self.config.package_name,
            self.config.min_sdk,
            self.config.target_sdk,
            deps_str
        );

        let app_dir = self.output_dir.join("app");
        fs::create_dir_all(&app_dir)?;
        fs::write(app_dir.join("build.gradle.kts"), content)?;
        Ok(())
    }

    fn generate_gradle_properties(&self) -> Result<(), Box<dyn std::error::Error>> {
        let content = r#"org.gradle.jvmargs=-Xmx4096m -Dfile.encoding=UTF-8
android.useAndroidX=true
android.enableJetifier=true
android.nonTransitiveRClass=true
android.enableR8.fullMode=true
kotlin.code.style=official
"#;
        fs::write(self.output_dir.join("gradle.properties"), content)?;
        Ok(())
    }

    fn generate_android_manifest(&self) -> Result<(), Box<dyn std::error::Error>> {
        let mut permissions = vec![
            "android.permission.INTERNET",
            "android.permission.ACCESS_NETWORK_STATE",
            "android.permission.ACCESS_WIFI_STATE",
        ];

        if self.config.features.contains(&"bluetooth".to_string()) {
            permissions.extend(vec![
                "android.permission.BLUETOOTH",
                "android.permission.BLUETOOTH_ADMIN",
                "android.permission.BLUETOOTH_CONNECT",
                "android.permission.BLUETOOTH_SCAN",
            ]);
        }
        if self.config.features.contains(&"nfc".to_string()) {
            permissions.push("android.permission.NFC");
        }
        if self.config.features.contains(&"camera".to_string()) {
            permissions.push("android.permission.CAMERA");
        }
        if self.config.features.contains(&"location".to_string()) {
            permissions.extend(vec![
                "android.permission.ACCESS_FINE_LOCATION",
                "android.permission.ACCESS_COARSE_LOCATION",
            ]);
        }
        if self.config.features.contains(&"storage".to_string()) {
            permissions.extend(vec![
                "android.permission.READ_EXTERNAL_STORAGE",
                "android.permission.WRITE_EXTERNAL_STORAGE",
            ]);
        }
        if self.config.features.contains(&"audio".to_string()) {
            permissions.extend(vec![
                "android.permission.RECORD_AUDIO",
                "android.permission.MODIFY_AUDIO_SETTINGS",
            ]);
        }
        if self.config.features.contains(&"vibrate".to_string()) {
            permissions.push("android.permission.VIBRATE");
        }
        if self.config.features.contains(&"wake_lock".to_string()) {
            permissions.push("android.permission.WAKE_LOCK");
        }
        if self.config.features.contains(&"foreground_service".to_string()) {
            permissions.push("android.permission.FOREGROUND_SERVICE");
        }
        if self.config.features.contains(&"boot_completed".to_string()) {
            permissions.push("android.permission.RECEIVE_BOOT_COMPLETED");
        }
        if self.config.features.contains(&"sms".to_string()) {
            permissions.extend(vec![
                "android.permission.SEND_SMS",
                "android.permission.RECEIVE_SMS",
                "android.permission.READ_SMS",
            ]);
        }
        if self.config.features.contains(&"phone".to_string()) {
            permissions.extend(vec![
                "android.permission.CALL_PHONE",
                "android.permission.READ_PHONE_STATE",
            ]);
        }
        if self.config.features.contains(&"contacts".to_string()) {
            permissions.extend(vec![
                "android.permission.READ_CONTACTS",
                "android.permission.WRITE_CONTACTS",
                "android.permission.GET_ACCOUNTS",
            ]);
        }
        if self.config.features.contains(&"biometric".to_string()) {
            permissions.extend(vec![
                "android.permission.USE_FINGERPRINT",
                "android.permission.USE_BIOMETRIC",
            ]);
        }
        if self.config.features.contains(&"sensors".to_string()) {
            permissions.push("android.permission.BODY_SENSORS");
            permissions.push("android.permission.HIGH_SAMPLING_RATE_SENSORS");
        }

        let perms_xml = permissions.iter()
            .map(|p| format!("    <uses-permission android:name=\"{}\" />", p))
            .collect::<Vec<_>>()
            .join("\n");

        let content = format!(
            r#"<?xml version="1.0" encoding="utf-8"?>
<manifest xmlns:android="http://schemas.android.com/apk/res/android"
    package="{}">

{}

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
            self.config.package_name,
            perms_xml
        );

        let manifest_dir = self.output_dir.join("app").join("src").join("main");
        fs::create_dir_all(&manifest_dir)?;
        fs::write(manifest_dir.join("AndroidManifest.xml"), content)?;
        Ok(())
    }

    fn generate_main_activity(&self) -> Result<(), Box<dyn std::error::Error>> {
        let package_path = self.config.package_name.replace('.', "/");
        let content = format!(
            r#"package {}

import android.os.Bundle
import android.util.Log
import androidx.appcompat.app.AppCompatActivity
import androidx.compose.ui.platform.ComposeView
import androidx.lifecycle.lifecycleScope
import androidx.lifecycle.repeatOnLifecycle
import androidx.lifecycle.Lifecycle
import kotlinx.coroutines.launch
import com.aly.lang.AlyRuntime
import com.aly.lang.AlyView

class MainActivity : AppCompatActivity() {{
    private val TAG = "AlyMainActivity"
    private lateinit var alyRuntime: AlyRuntime
    private lateinit var alyView: AlyView
    private lateinit var composeView: ComposeView

    override fun onCreate(savedInstanceState: Bundle?) {{
        super.onCreate(savedInstanceState)
        
        // Initialize Aly Runtime
        alyRuntime = AlyRuntime.getInstance(this)
        
        // Create ComposeView for Aly UI
        composeView = ComposeView(this)
        setContentView(composeView)
        
        // Create Aly View
        alyView = AlyView(this)
        
        // Load and run Aly script
        lifecycleScope.launch {{
            repeatOnLifecycle(Lifecycle.State.STARTED) {{
                try {{
                    val script = loadScript("main.aly")
                    alyRuntime.eval(script)
                }} catch (e: Exception) {{
                    Log.e(TAG, "Failed to load Aly script", e)
                }}
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
            self.config.package_name
        );

        let activity_dir = self.output_dir.join("app").join("src").join("main").join("kotlin").join(package_path);
        fs::create_dir_all(&activity_dir)?;
        fs::write(activity_dir.join("MainActivity.kt"), content)?;
        Ok(())
    }

    fn generate_aly_application(&self) -> Result<(), Box<dyn std::error::Error>> {
        let content = format!(
            r#"package {}

import android.app.Application
import android.util.Log
import com.aly.lang.AlyRuntime

class AlyApplication : Application() {{
    override fun onCreate() {{
        super.onCreate()
        
        // Initialize Aly Runtime early
        AlyRuntime.getInstance(this)
        
        // Initialize any native libraries
        System.loadLibrary("aly_rust")
        
        Log.d("AlyApplication", "Aly runtime initialized")
    }}
}}"#,
            self.config.package_name
        );

        let app_dir = self.output_dir.join("app").join("src").join("main").join("kotlin").join(self.config.package_name.replace('.', "/"));
        fs::create_dir_all(&app_dir)?;
        fs::write(app_dir.join("AlyApplication.kt"), content)?;
        Ok(())
    }

    fn generate_aly_view(&self) -> Result<(), Box<dyn std::error::Error>> {
        let content = format!(
            r#"package {}

import android.content.Context
import android.graphics.Canvas
import android.util.AttributeSet
import android.view.View
import com.aly.lang.AlyRuntime

class AlyView @JvmOverloads constructor(
    context: Context,
    attrs: AttributeSet? = null,
    defStyleAttr: Int = 0
) : View(context, attrs, defStyleAttr) {{
    
    private val runtime = AlyRuntime.getInstance(context)
    
    init {{
        setWillNotDraw(false)
        isFocusable = true
        isFocusableInTouchMode = true
    }}
    
    override fun onDraw(canvas: Canvas) {{
        super.onDraw(canvas)
        
        // Call Aly render function if exists
        runtime.callFunction("onDraw", width.toString(), height.toString())
    }}
    
    override fun onTouchEvent(event: android.view.MotionEvent): Boolean {{
        val action = when (event.action) {{
            android.view.MotionEvent.ACTION_DOWN -> "down"
            android.view.MotionEvent.ACTION_UP -> "up"
            android.view.MotionEvent.ACTION_MOVE -> "move"
            android.view.MotionEvent.ACTION_CANCEL -> "cancel"
            else -> "unknown"
        }}
        
        runtime.callFunction("onTouch", action, event.x.toString(), event.y.toString())
        return true
    }}
    
    override fun onKeyDown(keyCode: Int, event: android.view.KeyEvent?): Boolean {{
        runtime.callFunction("onKeyDown", keyCode.toString())
        return super.onKeyDown(keyCode, event)
    }}
    
    override fun onKeyUp(keyCode: Int, event: android.view.KeyEvent?): Boolean {{
        runtime.callFunction("onKeyUp", keyCode.toString())
        return super.onKeyUp(keyCode, event)
    }}
}}"#,
            self.config.package_name
        );

        let view_dir = self.output_dir.join("app").join("src").join("main").join("kotlin").join(self.config.package_name.replace('.', "/"));
        fs::create_dir_all(&view_dir)?;
        fs::write(view_dir.join("AlyView.kt"), content)?;
        Ok(())
    }

    fn generate_aly_runtime(&self) -> Result<(), Box<dyn std::error::Error>> {
        let content = format!(
            r#"package {}

import android.content.Context
import android.util.Log
import kotlinx.coroutines.Dispatchers
import kotlinx.coroutines.withContext

class AlyRuntime private constructor(private val context: Context) {{
    companion object {{
        @Volatile private var instance: AlyRuntime? = null
        
        fun getInstance(context: Context): AlyRuntime {{
            return instance ?: synchronized(this) {{
                instance ?: AlyRuntime(context.applicationContext).also {{ instance = it }}
            }}
        }}
    }}
    
    private val jni = AlyRuntimeJNI()
    
    init {{
        jni.init(context)
    }}
    
    fun eval(code: String): String {{
        return jni.eval(code)
    }}
    
    fun callFunction(name: String, vararg args: String): String {{
        return jni.callFunction(name, *args)
    }}
    
    fun callFunctionAsync(name: String, vararg args: String): kotlinx.coroutines.Deferred<String> {{
        return kotlinx.coroutines.CoroutineScope(Dispatchers.IO).async {{
            callFunction(name, *args)
        }}
    }}
    
    fun registerCallback(name: String, callback: (String) -> Unit) {{
        jni.registerCallback(name, callback)
    }}
    
    fun shutdown() {{
        jni.shutdown()
        instance = null
    }}
    
    fun getContext(): Context = context
    
    fun getAssets() = context.assets
    
    fun getResources() = context.resources
    
    fun getPackageName(): String = context.packageName
}}"#,
            self.config.package_name
        );

        let runtime_dir = self.output_dir.join("app").join("src").join("main").join("kotlin").join(self.config.package_name.replace('.', "/"));
        fs::create_dir_all(&runtime_dir)?;
        fs::write(runtime_dir.join("AlyRuntime.kt"), content)?;
        Ok(())
    }

    fn generate_aly_jni(&self) -> Result<(), Box<dyn std::error::Error>> {
        let content = format!(
            r#"package {}

import android.content.Context
import android.util.Log

class AlyRuntimeJNI {{
    companion object {{
        init {{
            System.loadLibrary("aly_rust")
        }}
    }}
    
    private var context: Context? = null
    
    fun init(context: Context) {{
        this.context = context
        nativeInit(context)
    }}
    
    fun eval(code: String): String {{
        return nativeEval(code)
    }}
    
    fun callFunction(name: String, vararg args: String): String {{
        return nativeCallFunction(name, args)
    }}
    
    fun registerCallback(name: String, callback: (String) -> Unit) {{
        nativeRegisterCallback(name, callback)
    }}
    
    fun shutdown() {{
        nativeShutdown()
    }}
    
    external fun nativeInit(context: Context)
    external fun nativeEval(code: String): String
    external fun nativeCallFunction(name: String, args: Array<String>): String
    external fun nativeRegisterCallback(name: String, callback: (String) -> Unit)
    external fun nativeShutdown()
}}"#,
            self.config.package_name
        );

        let jni_dir = self.output_dir.join("app").join("src").join("main").join("kotlin").join(self.config.package_name.replace('.', "/"));
        fs::create_dir_all(&jni_dir)?;
        fs::write(jni_dir.join("AlyRuntimeJNI.kt"), content)?;
        Ok(())
    }

    fn generate_cmake_lists(&self) -> Result<(), Box<dyn std::error::Error>> {
        let content = r#"cmake_minimum_required(VERSION 3.22.1)
project("aly_android" LANGUAGES CXX)

set(CMAKE_CXX_STANDARD 17)
set(CMAKE_CXX_STANDARD_REQUIRED ON)

# Include directories
include_directories(${CMAKE_SOURCE_DIR}/../rust/include)

# Rust library (built separately)
add_library(
    aly_rust
    SHARED
    IMPORTED
)

set_target_properties(
    aly_rust
    PROPERTIES
    IMPORTED_LOCATION ${CMAKE_SOURCE_DIR}/../target/${ANDROID_ABI}/libaly_rust.so
    INTERFACE_INCLUDE_DIRECTORIES ${CMAKE_SOURCE_DIR}/../rust/include
)

# JNI bridge library
add_library(
    aly_jni
    SHARED
    src/main/cpp/aly_jni.cpp
)

target_link_libraries(
    aly_jni
    aly_rust
    ${log-lib}
    android
)

find_library(log-lib log)
"#;
        let cpp_dir = self.output_dir.join("app").join("src").join("main").join("cpp");
        fs::create_dir_all(&cpp_dir)?;
        fs::write(cpp_dir.join("CMakeLists.txt"), content)?;
        Ok(())
    }

    fn generate_aly_jni_cpp(&self) -> Result<(), Box<dyn std::error::Error>> {
        let content = r#"#include <jni.h>
#include <string>
#include <android/log.h>
#include "aly_rust.h"

#define LOG_TAG "AlyJNI"
#define LOGD(...) __android_log_print(ANDROID_LOG_DEBUG, LOG_TAG, __VA_ARGS__)
#define LOGE(...) __android_log_print(ANDROID_LOG_ERROR, LOG_TAG, __VA_ARGS__)

static JavaVM* g_jvm = nullptr;
static jobject g_context = nullptr;

extern "C" JNIEXPORT jint JNICALL JNI_OnLoad(JavaVM* vm, void* reserved) {
    g_jvm = vm;
    JNIEnv* env;
    if (vm->GetEnv(reinterpret_cast<void**>(&env), JNI_VERSION_1_6) != JNI_OK) {
        return JNI_ERR;
    }
    
    // Initialize Aly runtime
    aly_runtime_init(env);
    
    return JNI_VERSION_1_6;
}

extern "C" JNIEXPORT void JNICALL JNI_OnUnload(JavaVM* vm, void* reserved) {
    aly_runtime_shutdown();
    g_jvm = nullptr;
}

static JNIEnv* getEnv() {
    JNIEnv* env;
    if (g_jvm->GetEnv(reinterpret_cast<void**>(&env), JNI_VERSION_1_6) != JNI_OK) {
        if (g_jvm->AttachCurrentThread(&env, nullptr) != JNI_OK) {
            return nullptr;
        }
    }
    return env;
}

extern "C" JNIEXPORT void JNICALL
Java_com_aly_lang_AlyRuntimeJNI_nativeInit(JNIEnv* env, jobject thiz, jobject context) {
    g_context = env->NewGlobalRef(context);
    aly_android_init(env, context);
}

extern "C" JNIEXPORT jstring JNICALL
Java_com_aly_lang_AlyRuntimeJNI_nativeEval(JNIEnv* env, jobject thiz, jstring code) {
    const char* code_str = env->GetStringUTFChars(code, nullptr);
    char* result = aly_eval(code_str);
    env->ReleaseStringUTFChars(code, code_str);
    
    jstring result_str = env->NewStringUTF(result);
    free(result);
    return result_str;
}

extern "C" JNIEXPORT jstring JNICALL
Java_com_aly_lang_AlyRuntimeJNI_nativeCallFunction(JNIEnv* env, jobject thiz, jstring name, jobjectArray args) {
    const char* name_str = env->GetStringUTFChars(name, nullptr);
    
    jsize argc = env->GetArrayLength(args);
    char** argv = new char*[argc];
    
    for (jsize i = 0; i < argc; i++) {
        jstring arg = (jstring)env->GetObjectArrayElement(args, i);
        const char* arg_str = env->GetStringUTFChars(arg, nullptr);
        argv[i] = strdup(arg_str);
        env->ReleaseStringUTFChars(arg, arg_str);
        env->DeleteLocalRef(arg);
    }
    
    char* result = aly_call_function(name_str, argc, argv);
    
    for (jsize i = 0; i < argc; i++) {
        free(argv[i]);
    }
    delete[] argv;
    
    env->ReleaseStringUTFChars(name, name_str);
    
    jstring result_str = env->NewStringUTF(result);
    free(result);
    return result_str;
}

extern "C" JNIEXPORT void JNICALL
Java_com_aly_lang_AlyRuntimeJNI_nativeRegisterCallback(JNIEnv* env, jobject thiz, jstring name, jobject callback) {
    // Store callback for later use
    aly_register_callback(env, name, callback);
}

extern "C" JNIEXPORT void JNICALL
Java_com_aly_lang_AlyRuntimeJNI_nativeShutdown(JNIEnv* env, jobject thiz) {
    aly_runtime_shutdown();
    
    if (g_context) {
        env->DeleteGlobalRef(g_context);
        g_context = nullptr;
    }
}

// Helper to call Java callbacks from Rust
extern "C" void aly_java_callback(const char* callback_name, const char* result) {
    JNIEnv* env = getEnv();
    if (!env) return;
    
    // Find the callback object and invoke it
    // This would be implemented based on how callbacks are stored
    LOGD("Callback: %s -> %s", callback_name, result);
}
"#;
        let cpp_dir = self.output_dir.join("app").join("src").join("main").join("cpp");
        fs::create_dir_all(&cpp_dir)?;
        fs::write(cpp_dir.join("aly_jni.cpp"), content)?;
        Ok(())
    }

    fn generate_resources(&self) -> Result<(), Box<dyn std::error::Error>> {
        let res_dir = self.output_dir.join("app").join("src").join("main").join("res");
        
        // strings.xml
        let strings_content = format!(
            r#"<resources>
    <string name="app_name">{}</string>
    <string name="aly_runtime_init">Initializing Aly runtime...</string>
    <string name="aly_script_loaded">Aly script loaded successfully</string>
    <string name="aly_error">Error: %s</string>
</resources>"#,
            self.config.app_name
        );
        fs::create_dir_all(res_dir.join("values"))?;
        fs::write(res_dir.join("values").join("strings.xml"), strings_content)?;

        // themes.xml
        let themes_content = r#"<resources xmlns:tools="http://schemas.android.com/tools">
    <!-- Base application theme. -->
    <style name="Theme.AlyApp" parent="Theme.Material3.DayNight.NoActionBar">
        <item name="colorPrimary">@color/purple_500</item>
        <item name="colorPrimaryVariant">@color/purple_700</item>
        <item name="colorOnPrimary">@color/white</item>
        <item name="colorSecondary">@color/teal_200</item>
        <item name="colorSecondaryVariant">@color/teal_700</item>
        <item name="colorOnSecondary">@color/black</item>
        <item name="android:statusBarColor">@color/black</item>
    </style>

    <style name="Theme.AlyApp.NoActionBar">
        <item name="windowActionBar">false</item>
        <item name="windowNoTitle">true</item>
    </style>
</resources>"#;
        fs::write(res_dir.join("values").join("themes.xml"), themes_content)?;

        // colors.xml
        let colors_content = r#"<resources>
    <color name="purple_200">#FFBB86FC</color>
    <color name="purple_500">#FF6200EE</color>
    <color name="purple_700">#FF3700B3</color>
    <color name="teal_200">#FF03DAC5</color>
    <color name="teal_700">#FF018786</color>
    <color name="black">#FF000000</color>
    <color name="white">#FFFFFFFF</color>
</resources>"#;
        fs::write(res_dir.join("values").join("colors.xml"), colors_content)?;

        // data_extraction_rules.xml
        let data_extraction_content = r#"<?xml version="1.0" encoding="utf-8"?>
<data-extraction-rules>
    <cloud-backup>
        <exclude domain="sharedpref" path="."/>
    </cloud-backup>
    <device-transfer>
        <include domain="sharedpref" path="."/>
    </device-transfer>
</data-extraction-rules>"#;
        fs::create_dir_all(res_dir.join("xml"))?;
        fs::write(res_dir.join("xml").join("data_extraction_rules.xml"), data_extraction_content)?;

        // backup_rules.xml
        let backup_rules_content = r#"<?xml version="1.0" encoding="utf-8"?>
<full-backup-content>
    <exclude domain="sharedpref" path="."/>
</full-backup-content>"#;
        fs::write(res_dir.join("xml").join("backup_rules.xml"), backup_rules_content)?;

        Ok(())
    }

    fn generate_aly_script_template(&self) -> Result<(), Box<dyn std::error::Error>> {
        let content = r#"// Aly Android App Template
// This script runs on the Aly runtime embedded in your Android app

// Import Android APIs
import android.app.Activity
import android.content.Context
import android.widget.Toast
import android.util.Log
import android.view.View
import android.graphics.Canvas
import android.graphics.Paint
import android.graphics.Color

// Global variables
var screenWidth = 0
var screenHeight = 0
var isRunning = true

// Main entry point
fun main() {
    Log.d("AlyApp", "Aly script started")
    
    // Initialize UI
    initUI()
    
    // Start render loop
    startRenderLoop()
}

// Initialize UI
fun initUI() {
    // Get screen dimensions
    screenWidth = Activity.getWidth()
    screenHeight = Activity.getHeight()
    
    Log.d("AlyApp", "Screen: ${screenWidth}x${screenHeight}")
    
    // Show welcome toast
    Toast.makeText(Activity.getContext(), "Welcome to Aly on Android!", Toast.LENGTH_LONG).show()
}

// Render loop
fun startRenderLoop() {
    // This would be called from the native view's onDraw
    // For now, just log
    Log.d("AlyApp", "Render loop started")
}

// Called from native onDraw
fun onDraw(width: String, height: String) {
    screenWidth = int(width)
    screenHeight = int(height)
    // Custom drawing code here
}

// Touch handler
fun onTouch(action: String, x: String, y: String) {
    Log.d("AlyTouch", "${action} at ${x}, ${y}")
    
    if (action == "down") {
        // Handle touch down
    }
}

// Key handlers
fun onKeyDown(keyCode: String) {
    Log.d("AlyKey", "Key down: ${keyCode}")
}

fun onKeyUp(keyCode: String) {
    Log.d("AlyKey", "Key up: ${keyCode}")
}

// Utility functions
fun showToast(message: String) {
    Toast.makeText(Activity.getContext(), message, Toast.LENGTH_SHORT).show()
}

fun logInfo(tag: String, message: String) {
    Log.i(tag, message)
}

fun logError(tag: String, message: String) {
    Log.e(tag, message)
}

// Android Intent helpers
fun startActivity(className: String) {
    val intent = Intent(Activity.getContext(), Class.forName(className))
    Activity.startActivity(intent)
}

fun startActivityWithExtras(className: String, extras: Map<String, String>) {
    val intent = Intent(Activity.getContext(), Class.forName(className))
    for ((key, value) in extras) {
        intent.putExtra(key, value)
    }
    Activity.startActivity(intent)
}

// SharedPreferences helpers
fun savePref(key: String, value: String) {
    val prefs = Activity.getSharedPreferences("AlyPrefs", Context.MODE_PRIVATE)
    prefs.edit().putString(key, value).apply()
}

fun getPref(key: String, default: String = ""): String {
    val prefs = Activity.getSharedPreferences("AlyPrefs", Context.MODE_PRIVATE)
    return prefs.getString(key, default)
}

// File I/O helpers
fun readFile(filename: String): String {
    val fis = Activity.openFileInput(filename)
    val reader = BufferedReader(InputStreamReader(fis))
    val content = reader.readText()
    reader.close()
    return content
}

fun writeFile(filename: String, content: String) {
    val fos = Activity.openFileOutput(filename, Context.MODE_PRIVATE)
    fos.write(content.toByteArray())
    fos.close()
}

// Network helpers (requires INTERNET permission)
fun httpGet(url: String): String {
    val urlObj = URL(url)
    val connection = urlObj.openConnection() as HttpURLConnection
    connection.requestMethod = "GET"
    val response = connection.inputStream.bufferedReader().readText()
    connection.disconnect()
    return response
}

fun httpPost(url: String, json: String): String {
    val urlObj = URL(url)
    val connection = urlObj.openConnection() as HttpURLConnection
    connection.requestMethod = "POST"
    connection.setRequestProperty("Content-Type", "application/json")
    connection.doOutput = true
    connection.outputStream.write(json.toByteArray())
    val response = connection.inputStream.bufferedReader().readText()
    connection.disconnect()
    return response
}

// Sensor helpers
fun getSensorManager(): SensorManager {
    return Activity.getSystemService(Context.SENSOR_SERVICE) as SensorManager
}

fun registerSensorListener(sensorType: Int, listener: SensorEventListener) {
    val sensorManager = getSensorManager()
    val sensor = sensorManager.getDefaultSensor(sensorType)
    sensorManager.registerListener(listener, sensor, SensorManager.SENSOR_DELAY_NORMAL)
}

// Camera helpers (requires CAMERA permission)
fun openCamera(cameraId: String = "0"): CameraDevice {
    val cameraManager = Activity.getSystemService(Context.CAMERA_SERVICE) as CameraManager
    // Implementation would use Camera2 API
    return null // Placeholder
}

// Bluetooth helpers (requires BLUETOOTH permissions)
fun getBluetoothAdapter(): BluetoothAdapter {
    val bluetoothManager = Activity.getSystemService(Context.BLUETOOTH_SERVICE) as BluetoothManager
    return bluetoothManager.adapter
}

// Location helpers (requires LOCATION permissions)
fun getLocationManager(): LocationManager {
    return Activity.getSystemService(Context.LOCATION_SERVICE) as LocationManager
}

fun requestLocationUpdates(provider: String, minTime: Long, minDistance: Float, listener: LocationListener) {
    val locationManager = getLocationManager()
    locationManager.requestLocationUpdates(provider, minTime, minDistance, listener)
}

// Run main
main()
"#;
        let assets_dir = self.output_dir.join("app").join("src").join("main").join("assets");
        fs::create_dir_all(&assets_dir)?;
        fs::write(assets_dir.join("main.aly"), content)?;
        Ok(())
    }

    fn generate_readme(&self) -> Result<(), Box<dyn std::error::Error>> {
        let project_name = &self.config.project_name;
        let package_path = self.config.package_name.replace('.', "/");
        
        let mut content = String::new();
        content.push_str(&format!("# {project_name} - Aly Android App\n\n"));
        content.push_str("This is an Android app template for the **Aly programming language**.\n\n");
        content.push_str("## Features\n\n");
        content.push_str("- [x] Native Android integration via JNI\n");
        content.push_str("- [x] Aly runtime embedded in the app\n");
        content.push_str("- [x] Access to Android APIs from Aly scripts\n");
        content.push_str("- [x] Hot reload support (development)\n");
        content.push_str("- [x] Compose UI integration\n");
        content.push_str("- [x] Coroutines support\n");
        content.push_str("- [x] Full access to sensors, camera, Bluetooth, NFC, etc.\n\n");
        content.push_str("## Project Structure\n\n");
        content.push_str(&format!("```\n{project_name}\n+-- app/\n|   +-- src/\n|   |   +-- main/\n|   |   |   +-- cpp/           # JNI C++ bridge\n|   |   |   +-- kotlin/        # Kotlin code\n|   |   |   |   +-- {package_path}/\n|   |   |   |       +-- MainActivity.kt\n|   |   |   |       +-- AlyApplication.kt\n|   |   |   |       +-- AlyView.kt\n|   |   |   |       +-- AlyRuntime.kt\n|   |   |   |       +-- AlyRuntimeJNI.kt\n|   |   |   +-- assets/\n|   |   |   |   +-- main.aly   # Main Aly script\n|   |   |   +-- res/           # Android resources\n|   |   |   +-- AndroidManifest.xml\n|   |   +-- build.gradle.kts\n|   |   +-- CMakeLists.txt\n|   +-- build.gradle.kts\n|   +-- proguard-rules.pro\n+-- build.gradle.kts\n+-- settings.gradle.kts\n+-- gradle.properties\n+-- README.md\n```\n\n"));
        content.push_str("## Building\n\n");
        content.push_str("### Prerequisites\n\n");
        content.push_str("- Android SDK (API 34)\n");
        content.push_str("- Android NDK (r26+)\n");
        content.push_str("- Rust toolchain with Android targets\n");
        content.push_str("- CMake 3.22+\n\n");
        content.push_str("### Setup Rust Targets\n\n");
        content.push_str("```bash\nrustup target add aarch64-linux-android armv7-linux-androideabi x86_64-linux-android i686-linux-android\n```\n\n");
        content.push_str("### Build Commands\n\n");
        content.push_str("```bash\n# Build Rust library for all Android architectures\n./scripts/build.sh\n\n# Or on Windows:\nscripts\\build.bat\n```\n\n");
        content.push_str("### Manual Build\n\n");
        content.push_str("```bash\n# Build Rust library\ncd ../../..  # Aly root directory\nfor target in aarch64-linux-android armv7-linux-androideabi x86_64-linux-android i686-linux-android; do\n    cargo build --target $target --features android --release\ndone\n\n# Copy libraries\nmkdir -p app/src/main/jniLibs/arm64-v8a\nmkdir -p app/src/main/jniLibs/armeabi-v7a\nmkdir -p app/src/main/jniLibs/x86_64\nmkdir -p app/src/main/jniLibs/x86\n\ncp ../../../target/aarch64-linux-android/release/libaly_rust.so app/src/main/jniLibs/arm64-v8a/\ncp ../../../target/armv7-linux-androideabi/release/libaly_rust.so app/src/main/jniLibs/armeabi-v7a/\ncp ../../../target/x86_64-linux-android/release/libaly_rust.so app/src/main/jniLibs/x86_64/\ncp ../../../target/i686-linux-android/release/libaly_rust.so app/src/main/jniLibs/x86/\n\n# Build APK\n./gradlew assembleDebug\n```\n\n");
        content.push_str("## Running\n\n");
        content.push_str("```bash\n# Install on connected device\n./gradlew installDebug\n\n# Or use adb directly\nadb install app/build/outputs/apk/debug/app-debug.apk\n```\n\n");
        content.push_str("## Developing with Aly\n\n");
        content.push_str("Edit `app/src/main/assets/main.aly` to write your Aly code. The script is loaded and executed on app startup.\n\n");
        content.push_str("### Available Android APIs from Aly\n\n");
        content.push_str("```aly\n// Toast\nToast.makeText(context, \"Hello\", Toast.LENGTH_SHORT).show()\n\n// Logging\nLog.d(\"TAG\", \"Message\")\n\n// Intents\nval intent = Intent(context, TargetActivity::class.java)\nstartActivity(intent)\n\n// SharedPreferences\nval prefs = getSharedPreferences(\"name\", MODE_PRIVATE)\nprefs.edit().putString(\"key\", \"value\").apply()\n\n// File I/O\nval content = readFile(\"filename.txt\")\nwriteFile(\"filename.txt\", \"content\")\n\n// Network\nval response = httpGet(\"https://api.example.com/data\")\nval response = httpPost(\"https://api.example.com/data\", \"{\\\"key\\\": \\\"value\\\"}\")\n\n// Sensors\nval sensorManager = getSensorManager()\nregisterSensorListener(Sensor.TYPE_ACCELEROMETER, listener)\n\n// Camera (Camera2 API)\nval cameraManager = getSystemService(Context.CAMERA_SERVICE) as CameraManager\n\n// Bluetooth\nval bluetoothAdapter = getBluetoothAdapter()\n\n// Location\nval locationManager = getLocationManager()\nrequestLocationUpdates(LocationManager.GPS_PROVIDER, 1000, 10f, listener)\n```\n\n");
        content.push_str("## Hot Reload (Development)\n\n");
        content.push_str("For development, you can push updated Aly scripts to the device:\n\n");
        content.push_str("```bash\n# Push updated script\nadb push app/src/main/assets/main.aly /data/local/tmp/main.aly\n\n# The app can reload it at runtime\n```\n\n");
        content.push_str("## Permissions\n\n");
        content.push_str("The template includes common permissions. Add more to `AndroidManifest.xml` as needed:\n\n");
        content.push_str("```xml\n<uses-permission android:name=\"android.permission.CAMERA\" />\n<uses-permission android:name=\"android.permission.ACCESS_FINE_LOCATION\" />\n<uses-permission android:name=\"android.permission.BLUETOOTH_CONNECT\" />\n<!-- etc. -->\n```\n\n");
        content.push_str("## Customization\n\n");
        content.push_str("### Adding Native Modules\n\n");
        content.push_str("1. Create Rust module in Aly core\n");
        content.push_str("2. Export functions via `aly_rust.h`\n");
        content.push_str("3. Call from JNI bridge in `aly_jni.cpp`\n");
        content.push_str("4. Expose to Kotlin in `AlyRuntimeJNI.kt`\n");
        content.push_str("5. Use from Aly scripts\n\n");
        content.push_str("### Adding Kotlin Extensions\n\n");
        content.push_str("Add extension functions to `AlyRuntime.kt`:\n\n");
        content.push_str("```kotlin\nfun AlyRuntime.myCustomFunction(param: String): String {\n    return callFunction(\"myCustomFunction\", param)\n}\n```\n\n");
        content.push_str("## Troubleshooting\n\n");
        content.push_str("### UnsatisfiedLinkError\n");
        content.push_str("- Ensure `libaly_rust.so` is in `jniLibs/<arch>/`\n");
        content.push_str("- Check NDK version compatibility\n");
        content.push_str("- Verify `System.loadLibrary(\"aly_rust\")` name matches\n\n");
        content.push_str("### Build Errors\n");
        content.push_str("- Clean: `./gradlew clean`\n");
        content.push_str("- Rebuild Rust: `cargo clean && cargo build --target ...`\n");
        content.push_str("- Check CMake version: `cmake --version`\n\n");
        content.push_str("### Runtime Crashes\n");
        content.push_str("- Check logcat: `adb logcat *:V | grep Aly`\n");
        content.push_str("- Verify JNI signatures match\n");
        content.push_str("- Ensure context is properly passed\n\n");
        content.push_str("## License\n\n");
        content.push_str("Same as Aly language license.\n");
        
        fs::write(self.output_dir.join("README.md"), content)?;
        Ok(())
    }

    fn generate_proguard_rules(&self) -> Result<(), Box<dyn std::error::Error>> {
        let content = r#"# ProGuard rules for Aly Android app

# Keep Aly runtime classes
-keep class com.aly.lang.** { *; }

# Keep JNI native methods
-keepclasseswithmembernames class * {
    native <methods>;
}

# Keep Aly script classes (if any)
-keep class * extends com.aly.lang.AlyScript { *; }

# Keep Parcelable implementations
-keep class * implements android.os.Parcelable {
    public static final android.os.Parcelable$Creator *;
}

# Keep Enum values
-keepclassmembers enum * {
    public static **[] values();
    public static ** valueOf(java.lang.String);
}

# Keep annotations
-keepattributes *Annotation*

# Keep generic signatures
-keepattributes Signature

# Keep source file and line number info
-keepattributes SourceFile,LineNumberTable

# OkHttp/Retrofit
-keep class okhttp3.** { *; }
-keep class retrofit2.** { *; }

# Coroutines
-keep class kotlinx.coroutines.** { *; }

# Room
-keep class * extends androidx.room.RoomDatabase { *; }

# Hilt
-keep class * extends dagger.hilt.android.HiltAndroidApp { *; }
"#;
        let app_dir = self.output_dir.join("app");
        fs::write(app_dir.join("proguard-rules.pro"), content)?;
        Ok(())
    }

    fn generate_gitignore(&self) -> Result<(), Box<dyn std::error::Error>> {
        let content = r#"# Android
*.apk
*.ap_
*.aab
*.aar

# Gradle
.gradle/
build/
/*/build/
!gradle/wrapper/gradle-wrapper.jar

# Local configuration
local.properties
*.properties

# Log files
*.log

# Android Studio
.idea/
*.iml
*.iws
*.ipr
*.swp
*.sw?

# Rust
target/
**/target/
Cargo.lock

# CMake
.cmake/
*.cmake

# Generated files
*.generated.*

# OS
.DS_Store
Thumbs.db
"#;
        fs::write(self.output_dir.join(".gitignore"), content)?;
        Ok(())
    }
}

pub fn generate_android_template(
    output_dir: &str,
    package_name: &str,
    app_name: &str,
    project_name: Option<&str>,
) -> Result<(), Box<dyn std::error::Error>> {
    let config = TemplateConfig {
        package_name: package_name.to_string(),
        app_name: app_name.to_string(),
        project_name: project_name.unwrap_or(app_name).to_string(),
        ..Default::default()
    };

    let generator = AndroidTemplateGenerator::new(config, output_dir);
    generator.generate()
}

pub fn generate_full_android_template(
    output_dir: &str,
    config: TemplateConfig,
) -> Result<(), Box<dyn std::error::Error>> {
    let generator = AndroidTemplateGenerator::new(config, output_dir);
    generator.generate()
}