# ABI C++ em Aly

O módulo ABI C++ permite a Aly chamar funções C++ directamente e gerir convenções de chamada C++.

---

## 1. Chamadas de Funções C++

```aly
import cpp_abi

let result = cpp_abi.call("_Z3addii", [3, 4])
# Chama a função C++ mangled `add(int, int)`
```

---

## 2. Mangling de Nomes C++

O módulo `cpp_abi` gere o mangling de nomes específico da plataforma:

| Compiler | Padrão de Mangling |
|----------|---------------------|
| GCC/Clang (Itanium) | `_Z3addii` |
| MSVC | `?add@@YAHHH@Z` |

---

## 3. Disposição de Structs

As structs C++ são acedidas através da disposição ABI:

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

## 4. Convenções de Chamada

Suporta convenções `thiscall` (MSVC), `fastcall` e `cdecl`:

```aly
cpp_abi.call_with_convention("method", [this_ptr, arg1], "thiscall")
```
