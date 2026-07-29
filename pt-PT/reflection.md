# Reflexão no Aly

O Aly fornece capacidades básicas de reflexão para inspecionar o estado em tempo de execução.

---

## 1. Listar Variáveis

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

## 3. Inspeção de Tipos em Tempo de Execução

A função `type()` retorna o nome do tipo de um valor:

```aly
let x = 42
print(type(x))      # Outputs: int
print(type(3.14))   # Outputs: float
print(type("hi"))   # Outputs: string
print(type(true))   # Outputs: bool
print(type([1,2]))  # Outputs: vector
```
