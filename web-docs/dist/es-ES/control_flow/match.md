# Coincidencia de Patrones (Match) en Aly

La sentencia `match` evalúa una expresión y la compara con múltiples patrones.

---

## 1. Sintaxis

```aly
let value = 2

match value {
    1 => print("One"),
    2 => {
        print("Two")
    },
    _ => print("Default fallback")
}
```

* **`_`**: Actúa como el patrón comodín de captura universal (caso por defecto).
* Los bloques pueden ser expresiones únicas o bloques compuestos encapsulados en `{}`.