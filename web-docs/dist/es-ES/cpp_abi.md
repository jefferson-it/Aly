# C++ ABI en Aly

El módulo C++ ABI permite a Aly llamar funciones de C++ directamente y manejar las convenciones de llamada de C++.

---

## 1. Llamadas a Funciones C++

```aly
import cpp_abi

let result = cpp_abi.call("_Z3addii", [3, 4])
# Llama a la función C++ convertida `add(int, int)`
```

---

## 2. Mudanza de Nombres de C++ (Name Mangling)

El módulo `cpp_abi` maneja la mudanza de nombres específica de la plataforma:

| Compilador | Patrón de Mudanza |
|----------|-----------------|
| GCC/Clang (Itanium) | `_Z3addii` |
| MSVC | `?add@@YAHHH@Z` |

---

## 3. Diseño de Structs

Los structs de C++ se acceden a través del diseño de la ABI:

```aly
let obj = cpp_abi.create_struct("Point", [
    {name: "x", type: "int"},
    {name: "y", type: "int"}
])
cpp_abi.set_field(obj, "x", 10)
cpp_abi.set_field(obj, "y", 20)
let x = cpp_abi.get_field(obj, "x")
```

---

## 4. Convenciones de Llamada

Admite convenciones `thiscall` (MSVC), `fastcall` y `cdecl`:

```aly
cpp_abi.call_with_convention("method", [this_ptr, arg1], "thiscall")
```