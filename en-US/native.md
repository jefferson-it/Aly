# Native Bindings & Extensibility in Aly

Aly supports runtime extensibility and native Rust module bindings through native registration traits and dynamic libraries.

---

## 1. Rust Integration

To bind Rust methods as native functions inside Aly, you map Rust types using the `ValueData` representation in Rust modules (like `src/native/`):

* **`ValueData::Int(i64)`**: Corresponds to `int`.
* **`ValueData::Float(f64)`**: Corresponds to `float`.
* **`ValueData::Str(String)`**: Corresponds to `string`.
* **`ValueData::Bool(bool)`**: Corresponds to `bool`.
* **`ValueData::Nil`**: Corresponds to `None`.

---

## 2. Dynamic Library Plugins (`plugin import`)

External `.so` / `.dylib` / `.dll` library plugins can be loaded dynamically into the interpreter using:

```aly
plugin import "path/to/plugin"
```

This registers external symbols, methods, and structures into the active Aly execution runtime namespace.

---

## 3. Importing a Plugin

The basic import syntax is:

```aly
plugin import "path/to/plugin.so" as name
```

The path is relative to the current execution directory. The `as name` clause defines the namespace used to call the plugin's functions. If omitted, the filename (without extension) is used as the namespace.

---

## 4. Calling Functions

Functions exposed by the plugin are accessed via `namespace.function(args)`:

```aly
plugin import "./mymath.so" as m

let a = m.add(3, 7)        # 10
let b = m.multiply(4, 5)   # 20
let c = m.fibonacci(10)    # 55
```

Arguments are passed separated by commas or spaces.

---

## 5. Creating a Plugin

### 5.1 C ABI (Application Binary Interface)

Any plugin must export three mandatory functions described in the header `plugin_sdk/plugin.h`:

```c
#include "plugin_sdk/plugin.h"

/* ── Metadata ────────────────────────────────────────────────────── */
AlyPluginInfo* aly_plugin_init(void);

/* ── Execution ───────────────────────────────────────────────────── */
char* aly_plugin_call(const char* func_name, const char* args);

/* ── Cleanup ─────────────────────────────────────────────────────── */
void aly_plugin_free_string(char* ptr);
```

### 5.2 Structures

```c
typedef struct {
    int         api_version;   /* Always ALY_PLUGIN_API_VERSION (1) */
    const char* name;          /* Readable plugin name              */
    const char* version;       /* Plugin version string             */
    const char* description;   /* Short description                 */
} AlyPluginInfo;
```

### 5.3 Mandatory Functions

#### `aly_plugin_init`

Returns a pointer to a static `AlyPluginInfo` structure. Called once when the plugin is loaded. It must never return NULL.

```c
static AlyPluginInfo info = {
    .api_version = ALY_PLUGIN_API_VERSION,
    .name        = "Math Extension",
    .version     = "1.0.0",
    .description = "Extra mathematical functions",
};

AlyPluginInfo* aly_plugin_init(void) {
    return &info;
}
```

#### `aly_plugin_call`

Receives the function name and an arguments string. It must return a string allocated with `malloc` (or equivalent) which will be freed by the runtime via `aly_plugin_free_string`. Returns NULL to indicate an error.

```c
char* aly_plugin_call(const char* func_name, const char* args) {
    if (strcmp(func_name, "add") == 0) {
        int a, b;
        sscanf(args, "%d,%d", &a, &b);
        char* res = malloc(32);
        snprintf(res, 32, "%d", a + b);
        return res;
    }
    return NULL;
}
```

#### `aly_plugin_free_string`

Frees the string returned by `aly_plugin_call`. If ptr is NULL, the call is ignored.

```c
void aly_plugin_free_string(char* ptr) {
    free(ptr);
}
```

### 5.4 Optional Functions

#### `aly_plugin_functions`

Returns a comma-separated string containing the names of the exposed functions. If exported, each function is registered individually as `namespace.function_name`, allowing direct calls like `math.add(3, 7)`. If not exported, the runtime registers only the namespace without individual function bindings.

```c
char* aly_plugin_functions(void) {
    return strdup("add,multiply,fibonacci,factorial");
}
```

---

## 6. Full Example in C

```c
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include "plugin_sdk/plugin.h"

static AlyPluginInfo info = {
    .api_version = ALY_PLUGIN_API_VERSION,
    .name        = "Math Extension",
    .version     = "1.0.0",
    .description = "Extra mathematical functions for Aly",
};

AlyPluginInfo* aly_plugin_init(void) { return &info; }

static char* cmd_fibonacci(const char* args) {
    int n = atoi(args);
    if (n < 0) return NULL;
    if (n <= 1) {
        char* r = malloc(8);
        sprintf(r, "%d", n);
        return r;
    }
    int a = 0, b = 1;
    for (int i = 2; i <= n; i++) { int c = a + b; a = b; b = c; }
    char* r = malloc(16);
    sprintf(r, "%d", b);
    return r;
}

char* aly_plugin_call(const char* f, const char* a) {
    if (strcmp(f, "fibonacci") == 0) return cmd_fibonacci(a);
    return NULL;
}

void aly_plugin_free_string(char* p) { free(p); }

char* aly_plugin_functions(void) { return strdup("fibonacci"); }
```

Compile with:

```sh
gcc -shared -fPIC -o math_ext.so plugin.c -lm
```

---

## 7. Example in Rust

Create a project with `crate-type = ["cdylib"]` in `Cargo.toml`:

```toml
[package]
name = "aly_plugin_crypto"
version = "0.1.0"
edition = "2021"

[lib]
crate-type = ["cdylib"]

[dependencies]
sha2 = "0.10"
hex = "0.4"
```

Implement the ABI using `extern "C"` and `#[no_mangle]`:

```rust
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
    description: c"SHA-256 functions for Aly".as_ptr(),
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
    if !ptr.is_null() {
        unsafe { let _ = CString::from_raw(ptr); }
    }
}

#[no_mangle]
pub extern "C" fn aly_plugin_functions() -> *mut c_char {
    CString::new("sha256").unwrap_or_default().into_raw()
}
```

Compile with:

```sh
cargo build --release
# The .so will be under target/release/libaly_plugin_crypto.so
```

---

## 8. Example in Go

```go
package main

import "C"
import "unsafe"
import "strconv"

//export aly_plugin_init
func aly_plugin_init() *C.AlyPluginInfo {
    return &C.AlyPluginInfo{
        api_version: 1,
        name:        C.CString("Go Math"),
        version:     C.CString("1.0.0"),
        description: C.CString("Plugin written in Go"),
    }
}

//export aly_plugin_call
func aly_plugin_call(funcName, args *C.char) *C.char {
    f := C.GoString(funcName)
    a := C.GoString(args)
    switch f {
    case "double":
        n, _ := strconv.Atoi(a)
        return C.CString(strconv.Itoa(n * 2))
    }
    return nil
}

//export aly_plugin_free_string
func aly_plugin_free_string(ptr *C.char) {
    C.free(unsafe.Pointer(ptr))
}

//export aly_plugin_functions
func aly_plugin_functions() *C.char {
    return C.CString("double")
}

func main() {}
```

Compile with:

```sh
go build -buildmode=c-shared -o math_go.so plugin.go
```

---

## 9. Example in C++

```cpp
#include "plugin_sdk/plugin.h"
#include <cstdlib>
#include <cstring>

extern "C" {

static AlyPluginInfo info = {
    ALY_PLUGIN_API_VERSION,
    "C++ Math",
    "1.0.0",
    "Plugin in C++",
};

AlyPluginInfo* aly_plugin_init() { return &info; }

char* aly_plugin_call(const char* f, const char* a) {
    if (strcmp(f, "square") == 0) {
        int n = std::atoi(a);
        auto r = std::to_string(n * n);
        return strdup(r.c_str());
    }
    return nullptr;
}

void aly_plugin_free_string(char* p) { std::free(p); }

char* aly_plugin_functions() { return strdup("square"); }

}
```

Compile with:

```sh
g++ -shared -fPIC -o plugin_cpp.so plugin.cpp
```

---

## 10. Example in Zig

```zig
const c = @cImport({
    @cInclude("plugin_sdk/plugin.h");
});

export fn aly_plugin_init() *c.AlyPluginInfo {
    return &c.AlyPluginInfo{
        .api_version = c.ALY_PLUGIN_API_VERSION,
        .name = "Zig Plugin",
        .version = "1.0.0",
        .description = "Plugin written in Zig",
    };
}

export fn aly_plugin_call(func_name: [*c]const u8, args: [*c]const u8) ?[*c]u8 {
    // ...
    return null;
}

export fn aly_plugin_free_string(ptr: ?[*c]u8) void {
    if (ptr) |p| @free(p);
}

export fn aly_plugin_functions() ?[*c]u8 {
    return @as([*c]u8, @ptrCast(@alloc(u8, 1)));
}
```

Compile with:

```sh
zig build-lib -dynamic plugin.zig -lc
```

---

## 11. How Aly Loads Plugins

Internally, Aly uses `libloading` (Rust) to load the shared library at runtime via `dlopen`/`dlsym` (Linux), `LoadLibrary`/`GetProcAddress` (Windows), or `dlopen`/`dlsym` (macOS).

The execution flow is as follows:

1. The interpreter processes `plugin import "path.so" as name`.
2. Calls `aly_plugin_init()` to check metadata and compatibility.
3. Calls `aly_plugin_functions()` (if exported) to discover registered functions.
4. Registers each function under `name.function` in the active scope.
5. When `name.function(args)` is invoked, Aly parses the arguments into a comma-separated string and calls `aly_plugin_call("function", "arg1,arg2,...")`.
6. The string output is returned and converted back into the appropriate Aly types.

---

## 12. Security

Plugins are compiled native code executed inside the same address space as the Aly process. They possess full system access permissions (filesystem, networking, subprocesses, etc.). Never import plugins from untrusted sources.
