# Pattern Matching (Match) no Aly

A declaração `match` avalia uma expressão e a compara com múltiplos padrões.

---

## 1. Sintaxe

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

* **`_`**: Atua como o padrão curinga que captura todos os casos (caso padrão).
* Blocos podem ser expressões únicas ou blocos compostos envoltos em `{}`.
