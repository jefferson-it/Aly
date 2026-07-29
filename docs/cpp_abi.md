# C++ ABI in Aly

The C++ ABI module enables Aly to call C++ functions directly and handle C++ calling conventions.

---

## 1. C++ Function Calls

```aly
import cpp_abi

let result = cpp_abi.call("_Z3addii", [3, 4])
# Calls the mangled C++ function `add(int, int)`
```

---

## 2. C++ Name Mangling

The `cpp_abi` module handles platform-specific name mangling:

| Compiler | Mangling Pattern |
|----------|-----------------|
| GCC/Clang (Itanium) | `_Z3addii` |
| MSVC | `?add@@YAHHH@Z` |

---

## 3. Struct Layout

C++ structs are accessed through the ABI layout:

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

## 4. Calling Conventions

Supports `thiscall` (MSVC), `fastcall`, and `cdecl` conventions:

```aly
cpp_abi.call_with_convention("method", [this_ptr, arg1], "thiscall")
```