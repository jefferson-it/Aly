# Correspondência de Padrões (Match) em Aly

A instrução `match` avalia uma expressão e confronta-a com múltiplos padrões.

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

* **`_`**: Funciona como o padrão curinga predefinido (caso por omissão).
* Os blocos podem ser expressões únicas ou blocos compostos envolvidos em `{}`.
