# Bindings Nativos y Extensibilidad en Aly

Aly admite la extensibilidad en tiempo de ejecución y los enlaces (bindings) nativos de módulos de Rust a través de traits de registro nativos y librerías dinámicas.

---

## 1. Integración con Rust

Para vincular métodos de Rust como funciones nativas dentro de Aly, usted mapea los tipos de Rust utilizando la representación `ValueData` en los módulos de Rust (como `src/native/`):

* **`ValueData::Int(i64)`**: Corresponde a `int`.
* **`ValueData::Float(f64)`**: Corresponde a `float`.
* **`ValueData::Str(String)`**: Corresponde a `string`.
* **`ValueData::Bool(bool)`**: Corresponde a `bool`.
* **`ValueData::Nil`**: Corresponde a `None`.

---

## 2. Plugins de Librerías Dinámicas (`plugin import`)

Los plugins de librerías externas (`.so` / `.dylib` / `.dll`) se pueden cargar dinámicamente en el intérprete utilizando:

```aly
plugin import "ruta/al/plugin"
```

Esto registra los símbolos, métodos y estructuras externos en el espacio de nombres (namespace) activo del tiempo de ejecución de Aly.

---

## 3. Importación de un Plugin

La sintaxis básica de importación es:

```aly
plugin import "ruta/al/plugin.so" as nombre
```

La ruta es relativa al directorio de ejecución actual. La cláusula `as nombre` define el espacio de nombres utilizado para llamar a las funciones del plugin. Si se omite, se utiliza el nombre del archivo (sin extensión) como espacio de nombres.

---

## 4. Llamando a Funciones

Las funciones expuestas por el plugin se acceden mediante `namespace.funcion(args)`:

```aly
plugin import "./mymath.so" as m

let a = m.add(3, 7)        # 10
let b = m.multiply(4, 5)   # 20
let c = m.fibonacci(10)    # 55
```

Los argumentos se pasan separados por comas o espacios.

---

## 5. Creación de un Plugin

### 5.1 C ABI (Application Binary Interface)

Cualquier plugin debe exportar tres funciones obligatorias descritas en el encabezado `plugin_sdk/plugin.h`:

```c
#include "plugin_sdk/plugin.h"

/* ── Metadatos ───────────────────────────────────────────────────── */
AlyPluginInfo* aly_plugin_init(void);

/* ── Ejecución ───────────────────────────────────────────────────── */
char* aly_plugin_call(const char* func_name, const char* args);

/* ── Limpieza ────────────────────────────────────────────────────── */
void aly_plugin_free_string(char* ptr);
```

### 5.2 Estructuras

```c
typedef struct {
    int         api_version;   /* Siempre ALY_PLUGIN_API_VERSION (1) */
    const char* name;          /* Nombre legible del plugin         */
    const char* version;       /* Versión del plugin                */
    const char* description;   /* Descripción corta                 */
} AlyPluginInfo;
```

### 5.3 Funciones Obligatorias

#### `aly_plugin_init`

Devuelve un puntero a una estructura `AlyPluginInfo` estática. Se llama una vez cuando se carga el plugin. Nunca debe devolver NULL.

```c
static AlyPluginInfo info = {
    .api_version = ALY_PLUGIN_API_VERSION,
    .name        = "Math Extension",
    .version     = "1.0.0",
    .description = "Funciones matemáticas adicionales",
};

AlyPluginInfo* aly_plugin_init(void) {
    return &info;
}
```

#### `aly_plugin_call`

Recibe el nombre de la función y una cadena de argumentos. Debe devolver una cadena asignada con `malloc` (o equivalente) que será liberada por el runtime mediante `aly_plugin_free_string`. Devuelve NULL para indicar un error.

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

Libera la cadena devuelta por `aly_plugin_call`. Si el puntero es NULL, se ignora la llamada.

```c
void aly_plugin_free_string(char* ptr) {
    free(ptr);
}
```

### 5.4 Funciones Opcionales

#### `aly_plugin_functions`

Devuelve una cadena separada por comas que contiene los nombres de las funciones expuestas. Si se exporta, cada función se registra individualmente como `namespace.nombre_funcion`, lo que permite llamadas directas como `math.add(3, 7)`. Si no se exporta, el runtime registra solo el espacio de nombres sin enlaces de funciones individuales.

```c
char* aly_plugin_functions(void) {
    return strdup("add,multiply,fibonacci,factorial");
}
```

---

## 6. Ejemplo Completo en C

```c
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include "plugin_sdk/plugin.h"

static AlyPluginInfo info = {
    .api_version = ALY_PLUGIN_API_VERSION,
    .name        = "Math Extension",
    .version     = "1.0.0",
    .description = "Funciones matemáticas adicionales para Aly",
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

Compile con:

```sh
gcc -shared -fPIC -o math_ext.so plugin.c -lm
```

---

## 7. Ejemplo en Rust

Cree un proyecto con `crate-type = ["cdylib"]` en `Cargo.toml`:

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

Implemente la ABI utilizando `extern "C"` y `#[no_mangle]`:

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
    description: c"Funciones SHA-256 para Aly".as_ptr(),
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

Compile con:

```sh
cargo build --release
# El archivo .so estará en target/release/libaly_plugin_crypto.so
```

---

## 8. Ejemplo en Go

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
        description: C.CString("Plugin escrito en Go"),
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

Compile con:

```sh
go build -buildmode=c-shared -o math_go.so plugin.go
```

---

## 9. Ejemplo en C++

```cpp
#include "plugin_sdk/plugin.h"
#include <cstdlib>
#include <cstring>

extern "C" {

static AlyPluginInfo info = {
    ALY_PLUGIN_API_VERSION,
    "C++ Math",
    "1.0.0",
    "Plugin en C++",
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

Compile con:

```sh
g++ -shared -fPIC -o plugin_cpp.so plugin.cpp
```

---

## 10. Ejemplo en Zig

```zig
const c = @cImport({
    @cInclude("plugin_sdk/plugin.h");
});

export fn aly_plugin_init() *c.AlyPluginInfo {
    return &c.AlyPluginInfo{
        .api_version = c.ALY_PLUGIN_API_VERSION,
        .name = "Zig Plugin",
        .version = "1.0.0",
        .description = "Plugin escrito en Zig",
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

Compile con:

```sh
zig build-lib -dynamic plugin.zig -lc
```

---

## 11. Cómo Aly Carga Plugins

Internamente, Aly utiliza `libloading` (Rust) para cargar la librería compartida en tiempo de ejecución a través de `dlopen`/`dlsym` (Linux), `LoadLibrary`/`GetProcAddress` (Windows) o `dlopen`/`dlsym` (macOS).

El flujo de ejecución es:

1. El intérprete encuentra `plugin import "path.so" as nombre`.
2. Llama a `aly_plugin_init()` para obtener metadados y verificar la compatibilidad.
3. Llama a `aly_plugin_functions()` (si se exporta) para descubrir las funciones.
4. Registra cada función como `nombre.funcion` en el runtime de Aly.
5. Cuando se llama a `nombre.funcion(args)`, Aly convierte los argumentos en una cadena y llama a `aly_plugin_call("funcion", "arg1,arg2,...")`.
6. El resultado de la cadena se devuelve y se convierte de nuevo al tipo Aly correspondiente.

---

## 12. Seguridad

Los plugins son código nativo compilado que se ejecuta en el mismo proceso de Aly. Tienen acceso completo al sistema (archivos, red, etc.). Importe únicamente plugins de fuentes de confianza.