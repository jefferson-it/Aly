# Reflexão em Aly

Aly fornece capacidades básicas de reflexão para inspeccionar o estado em tempo de execução.

---

## 1. Listagem de Variáveis

```aly
import reflection

let x = 10
let y = "hello"

let vars = reflection.list_variables()
print(vars)  # Resultado: ["x", "y"]
```

---

## 2. Listagem de Schemas

```aly
import reflection

schema Person { name: "John" }
schema Car { model: "Sedan" }

let schemas = reflection.list_schemas()
print(schemas)  # Resultado: ["Person", "Car"]
```

---

## 3. Inspecção de Tipos em Tempo de Execução

A função `type()` devolve o nome do tipo de um valor:

```aly
let x = 42
print(type(x))      # Resultado: int
print(type(3.14))   # Resultado: float
print(type("hi"))   # Resultado: string
print(type(true))   # Resultado: bool
print(type([1,2]))  # Resultado: vector
```
