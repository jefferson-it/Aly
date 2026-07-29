# Enlaces Nativos y Extensibilidad en Aly

Aly admite la extensibilidad en tiempo de ejecución y los enlaces de módulos nativos de Rust a través de rasgos de registro nativos y bibliotecas dinámicas.

---

## 1. Integración con Rust

Para vincular métodos de Rust como funciones nativas dentro de Aly, se asignan tipos de Rust usando la representación `ValueData` en módulos de Rust (como `src/native/`):

* **`ValueData::Int(i64)`**: Corresponde a `int`.
* **`ValueData::Float(f64)`**: Corresponde a `float`.
* **`ValueData::Str(String)`**: Corresponde a `string`.
* **`ValueData::Bool(bool)`**: Corresponde a `bool`.
* **`ValueData::Nil`**: Corresponde a `None`.

---

## 2. Plugins de Biblioteca Dinámica (`plugin import`)

Los plugins de biblioteca externos `.so` / `.dylib` / `.dll` se pueden cargar dinámicamente en el intérprete usando:

```aly
plugin import "path/to/plugin"
```

Esto registra símbolos, métodos y estructuras externos en el espacio de nombres del runtime de ejecución activo de Aly.