# Reflexión en Aly

Aly proporciona capacidades básicas de reflexión para inspeccionar el estado en tiempo de ejecución.

---

## 1. Listar Variables

```aly
import reflection

let x = 10
let y = "hello"

let vars = reflection.list_variables()
print(vars)  # Outputs: ["x", "y"]
```

---

## 2. Listar Esquemas

```aly
import reflection

schema Person { name: "John" }
schema Car { model: "Sedan" }

let schemas = reflection.list_schemas()
print(schemas)  # Outputs: ["Person", "Car"]
```

---

## 3. Inspección de Tipos en Tiempo de Ejecución

La función `type()` devuelve el nombre del tipo de un valor:

```aly
let x = 42
print(type(x))      # Outputs: int
print(type(3.14))   # Outputs: float
print(type("hi"))   # Outputs: string
print(type(true))   # Outputs: bool
print(type([1,2]))  # Outputs: vector
```