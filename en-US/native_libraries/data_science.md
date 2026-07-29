# Data Science Module in Aly

Aly provides basic data science utilities for numerical computing and data analysis.

---

## 1. Statistics

```aly
import data_science

let data = [1.0, 2.0, 3.0, 4.0, 5.0]
print(data_science.mean(data))     # 3.0
print(data_science.median(data))   # 3.0
print(data_science.std_dev(data))  # ~1.414
```

---

## 2. Linear Algebra

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

print(df.head(2))       # First 2 rows
let filtered = df.filter("age gt 25")
print(df.describe())    # Summary statistics
```
