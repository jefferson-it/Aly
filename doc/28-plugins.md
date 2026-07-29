# Plugins (plugin import)

Plugins permitem estender Aly com código nativo escrito em C, C++, Rust, Go,
Zig, ou qualquer linguagem que compile para uma biblioteca compartilhada
(`.so`, `.dylib`, `.dll`).

```aly
plugin import "./math_ext.so" as math

let r = math.fibonacci(10)
console_log(r)
```

---

## 1. Importando um plugin

A sintaxe é:

```aly
plugin import "caminho/para/plugin.so" as nome
```

O caminho é relativo ao diretório atual de execução. O `as nome` define o
namespace usado para chamar as funções do plugin. Se omitido, o nome do
arquivo (sem extensão) é usado como namespace.

## 2. Chamando funções

Funções expostas pelo plugin são acessadas via `namespace.funcao(args)`:

```aly
plugin import "./mymath.so" as m

let a = m.add(3, 7)        # 10
let b = m.multiply(4, 5)   # 20
let c = m.fibonacci(10)    # 55
```

Argumentos são passados separados por vírgula ou espaço.

## 3. Criando um plugin

### 3.1 C ABI (Application Binary Interface)

Todo plugin deve exportar três funções obrigatórias descritas no header
`plugin_sdk/plugin.h`:

```c
#include "plugin_sdk/plugin.h"

/* ── Metadados ───────────────────────────────────────────────────── */
AlyPluginInfo* aly_plugin_init(void);

/* ── Execução ─────────────────────────────────────────────────────── */
char* aly_plugin_call(const char* func_name, const char* args);

/* ── Limpeza ──────────────────────────────────────────────────────── */
void aly_plugin_free_string(char* ptr);
```

### 3.2 Estruturas

```c
typedef struct {
    int         api_version;   /* Sempre ALY_PLUGIN_API_VERSION (1) */
    const char* name;          /* Nome legível do plugin            */
    const char* version;       /* Versão do plugin                  */
    const char* description;   /* Descrição curta                   */
} AlyPluginInfo;
```

### 3.3 Funções obrigatórias

#### `aly_plugin_init`

Retorna um ponteiro para uma struct `AlyPluginInfo` estática. Chamada uma
única vez ao carregar o plugin. Nunca deve retornar NULL.

```c
static AlyPluginInfo info = {
    .api_version = ALY_PLUGIN_API_VERSION,
    .name        = "Math Extension",
    .version     = "1.0.0",
    .description = "Funções matemáticas extras",
};

AlyPluginInfo* aly_plugin_init(void) {
    return &info;
}
```

#### `aly_plugin_call`

Recebe o nome da função e uma string de argumentos. Deve retornar uma
string alocada com `malloc` (ou equivalente) que será liberada via
`aly_plugin_free_string`. Retorne NULL para indicar erro.

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

Libera a string retornada por `aly_plugin_call`. Se ptr for NULL, a
chamada é ignorada.

```c
void aly_plugin_free_string(char* ptr) {
    free(ptr);
}
```

### 3.4 Função opcional

#### `aly_plugin_functions`

Retorna uma string com os nomes das funções separados por vírgula. Se
exportada, cada função é registrada individualmente como
`namespace.nomefuncao`, permitindo chamadas diretas como `math.add(3, 7)`.
Se não exportada, o runtime registra apenas o namespace sem funções
individuais.

```c
char* aly_plugin_functions(void) {
    return strdup("add,multiply,fibonacci,factorial");
}
```

## 4. Exemplo completo em C

```c
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include "plugin_sdk/plugin.h"

static AlyPluginInfo info = {
    .api_version = ALY_PLUGIN_API_VERSION,
    .name        = "Math Extension",
    .version     = "1.0.0",
    .description = "Funções matemáticas extras para Aly",
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

Compile com:

```sh
gcc -shared -fPIC -o math_ext.so plugin.c -lm
```

## 5. Exemplo em Rust

Crie um projeto com `crate-type = ["cdylib"]` no `Cargo.toml`:

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

Implemente o ABI usando `extern "C"` e `#[no_mangle]`:

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
    description: c"Funções SHA-256 para Aly".as_ptr(),
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

Compile com:

```sh
cargo build --release
# O .so estará em target/release/libaly_plugin_crypto.so
```

## 6. Exemplo em Go

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
        description: C.CString("Plugin escrito em Go"),
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

Compile com:

```sh
go build -buildmode=c-shared -o math_go.so plugin.go
```

## 7. Exemplo em C++

```cpp
#include "plugin_sdk/plugin.h"
#include <cstdlib>
#include <cstring>

extern "C" {

static AlyPluginInfo info = {
    ALY_PLUGIN_API_VERSION,
    "C++ Math",
    "1.0.0",
    "Plugin em C++",
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

Compile com:

```sh
g++ -shared -fPIC -o plugin_cpp.so plugin.cpp
```

## 8. Exemplo em Zig

```zig
const c = @cImport({
    @cInclude("plugin_sdk/plugin.h");
});

export fn aly_plugin_init() *c.AlyPluginInfo {
    return &c.AlyPluginInfo{
        .api_version = c.ALY_PLUGIN_API_VERSION,
        .name = "Zig Plugin",
        .version = "1.0.0",
        .description = "Plugin escrito em Zig",
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

Compile com:

```sh
zig build-lib -dynamic plugin.zig -lc
```

## 9. Como o Aly carrega plugins

Internamente, o Aly usa `libloading` (Rust) para carregar a biblioteca
compartilhada em tempo de execução via `dlopen`/`dlsym` (Linux),
`LoadLibrary`/`GetProcAddress` (Windows) ou `dlopen`/`dlsym` (macOS).

O fluxo é:

1. O interpretador encontra `plugin import "path.so" as nome`
2. Chama `aly_plugin_init()` para obter metadados e verificar a versão da API
3. Chama `aly_plugin_functions()` (se exportada) para descobrir as funções
4. Registra cada função como `nome.funcao` no runtime do Aly
5. Quando `nome.funcao(args)` é chamado, o Aly converte os argumentos para
   string e chama `aly_plugin_call("funcao", "arg1,arg2,...")`
6. O resultado string é convertido de volta para o tipo adequado no Aly

## 10. Segurança

Plugins são código nativo executado no mesmo processo do Aly. Eles têm
acesso total ao sistema (arquivos, rede, etc). Carregue apenas plugins de
fontes confiáveis.
