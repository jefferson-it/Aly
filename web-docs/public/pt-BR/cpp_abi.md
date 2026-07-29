# ABI C++ no Aly

O módulo de ABI C++ permite que o Aly chame funções C++ diretamente e lide com convenções de chamada C++.

---

## 1. Chamadas de Função C++

```aly
import cpp_abi

let result = cpp_abi.call("_Z3addii", [3, 4])
# Chama a função C++ compilada (mangled) `add(int, int)`
```

---

## 2. Name Mangling C++

O módulo `cpp_abi` lida com name mangling específico da plataforma:

| Compilador | Padrão de Mangling |
|----------|-----------------|
| GCC/Clang (Itanium) | `_Z3addii` |
| MSVC | `?add@@YAHHH@Z` |

---

## 3. Layout de Structs

As structs C++ são acessadas através do layout da ABI:

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