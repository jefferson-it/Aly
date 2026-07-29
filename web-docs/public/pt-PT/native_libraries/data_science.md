# Módulo de Ciência de Dados em Aly

Aly fornece utilitários básicos de ciência de dados para computação numérica e análise de dados.

---

## 1. Estatística

```aly
import data_science

let data = [1.0, 2.0, 3.0, 4.0, 5.0]
print(data_science.mean(data))     # 3.0
print(data_science.median(data))   # 3.0
print(data_science.std_dev(data))  # ~1.414
```

---

## 2. Álgebra Linear

```aly
let matrix = data_science.matrix([[1, 2], [3, 4]])
let vec = data_science.vector([1, 0])
let result = data_science.matrix_multiply(matrix, vec)
```

---

## 3. Data Frames

```aly
let df = data_science.data_frame({
    "name": ["Alice", "Bob", "Charlie"],
    "age": [25, 30, 35]
})

print(df.head(2))       # Primeiras 2 linhas
let filtered = df.filter("age gt 25")
print(df.describe())    # Estatísticas descritivas
```
