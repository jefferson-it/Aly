# Pattern Matching (Match) in Aly

The `match` statement evaluates an expression and matches it against multiple patterns.

---

## 1. Syntax

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

* **`_`**: Acts as the catch-all wildcard pattern (default case).
* Blocks can be single expressions or compound blocks wrapped in `{}`.
