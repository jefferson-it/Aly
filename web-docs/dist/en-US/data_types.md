# Data Types in Aly

Aly is dynamically-typed but internally supports a rich set of robust primitive and composite data types.

---

## 1. Primitives

* **Integer (`int`)**: 64-bit signed integers (e.g. `42`, `-10`).
* **Floating-Point (`float`)**: 64-bit IEEE 754 floating-point numbers (e.g. `3.14159`, `-0.001`).
* **Boolean (`bool`)**: Logical values, either `true` or `false`.
* **Character (`char`)**: Single-byte character literals enclosed in single quotes (e.g. `'A'`, `'\n'`, `'\t'`).
* **String (`string`)**: Sequence of characters enclosed in double quotes (e.g. `"Hello, World"`). Supports string template interpolation using `` syntax.
* **None (`null`)**: A special type representing the absence of value, accessed via the `None` keyword.
* **Void**: Signifies a function or expression that returns no value.

---

## 2. List / Vector (`list`)

Vectors are ordered, dynamic arrays that can contain elements of any type (including nested vectors or objects).

```aly
let numbers = [1, 2, 3, 4]
let mixed = ["Aly", 10.5, true, None]

# Index access
print(numbers[0]) # Outputs: 1
```

---

## 3. Objects / Maps (`object`)

Objects are key-value mappings representing dictionary or associative array structures. Keys are identifiers or strings.

```aly
let user = {
    name: "Jefferson",
    role: "Developer",
    active: true
}

# Member access
print(user.name) # Outputs: Jefferson
print(user["role"]) # Outputs: Developer
```
