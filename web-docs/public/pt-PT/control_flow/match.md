# Correspondência de Padrões (Match) no Aly

A declaração `match` avalia uma expressão e compara-a com múltiplos padrões.

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

* **`_`**: Atua como o padrão de correspondência universal (caso padrão).
* Os blocos podem ser expressões únicas ou blocos compostos envolvidos em `{}`.
