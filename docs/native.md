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
