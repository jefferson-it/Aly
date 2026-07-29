#ifndef ALY_RUST_H
#define ALY_RUST_H

#ifdef __cplusplus
extern "C" {
#endif

// Initialize the Aly runtime for Android
// Called from JNI_OnLoad
void aly_runtime_init(void* env);

// Initialize Android-specific functionality
// Called from nativeInit with Context
void aly_android_init(void* env, void* context);

// Evaluate Aly code and return result as string
// Caller must free the returned string
char* aly_eval(const char* code);

// Call a function by name with string arguments
// Caller must free the returned string
char* aly_call_function(const char* name, int argc, char** argv);

// Register a callback from Rust to Java
void aly_register_callback(void* env, const char* callback_name, void* callback_obj);

// Shutdown the runtime
void aly_runtime_shutdown();

// Free a string returned by aly_eval or aly_call_function
void aly_free_string(char* str);

#ifdef __cplusplus
}
#endif

#endif // ALY_RUST_H