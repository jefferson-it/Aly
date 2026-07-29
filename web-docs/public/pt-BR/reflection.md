# Reflexão in Aly

Aly provides basic reflection capabilities to inspect runtime state.

---

## 1. Listing Variables

```aly
import reflection

let x = 10
let y = "hello"

let vars = reflection.list_variables()
print(vars)  # Outputs: ["x", "y"]
```

---

## 2. Listing Schemas

```aly
import reflection

schema Person { name: "John" }
schema Car { model: "Sedan" }

let schemas = reflection.list_schemas()
print(schemas)  # Outputs: ["Person", "Car"]
```

---

## 3. Runtime Type Inspection

The `type()` function returns the type name of a value:

```aly
let x = 42
print(type(x))      # Outputs: int
print(type(3.14))   # Outputs: float
print(type("hi"))   # Outputs: string
print(type(true))   # Outputs: bool
print(type([1,2]))  # Outputs: vector
```