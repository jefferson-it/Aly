# Conditionals in Aly

Conditionals allow you to steer the execution flow based on boolean expressions.

---

## 1. `if` / `elif` / `else` Statements

The `if` statement evaluates a logical condition. Curly braces `{}` are required to enclose block scopes.

```aly
let age = 18

if age gt 60 {
    print("Senior")
} elif age gte 18 {
    print("Adult")
} else {
    print("Minor")
}
```

---

## 2. Operadores de comparação

Aly uses prefix-style text names for standard comparison operators:
* **`eq`**: Equal (`==`)
* **`ne`**: Not Equal (`!=`)
* **`gt`**: Greater Than (`>`)
* **`lt`**: Less Than (`<`)
* **`gte`**: Greater Than or Equal (`>=`)
* **`lte`**: Less Than or Equal (`<=`)

---

## 3. Operadores lógicos

* **`and`**: Logical AND. Returns `true` only if both expressions are true.
* **`or`**: Logical OR. Returns `true` if at least one expression is true.
* **`not` / `!`**: Logical NOT. Inverts the boolean value of the expression.
