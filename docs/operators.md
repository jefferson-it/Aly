# Operators in Aly

Aly supports arithmetic, comparison, logical, and assignment operators. Comparison operators use text-style names.

---

## 1. Arithmetic Operators

| Operator | Description | Example |
|----------|-------------|---------|
| `+` | Addition | `3 + 5` → `8` |
| `-` | Subtraction | `10 - 4` → `6` |
| `*` | Multiplication | `3 * 4` → `12` |
| `/` | Division | `10 / 3` → `3.333` |
| `%` | Modulo (remainder) | `10 % 3` → `1` |
| `**` | Power | `2 ** 3` → `8` |
| `++` | Increment (postfix) | `i++` → `i = i + 1` |
| `--` | Decrement (postfix) | `i--` → `i = i - 1` |

```aly
let x = 10
x = x + 5       # 15
x++             # 16 (desugared to x = x + 1)
```

---

## 2. Comparison (Relational) Operators

Aly uses **text-based** comparison operators:

| Operator | Meaning | Example |
|----------|---------|---------|
| `eq` | Equal to | `5 eq 5` → `true` |
| `ne` | Not equal to | `5 ne 3` → `true` |
| `gt` | Greater than | `5 gt 3` → `true` |
| `lt` | Less than | `5 lt 3` → `false` |
| `gte` | Greater than or equal | `5 gte 5` → `true` |
| `lte` | Less than or equal | `5 lte 3` → `false` |

```aly
if age gte 18 {
    print("Adult")
}
```

---

## 3. Logical Operators

| Operator | Description | Example |
|----------|-------------|---------|
| `and` | Logical AND | `true and false` → `false` |
| `or` | Logical OR | `true or false` → `true` |
| `not` | Logical NOT | `not true` → `false` |
| `!` | Logical NOT (alias) | `!true` → `false` |

```aly
if age gte 18 and has_id eq true {
    print("Allowed")
}
```

---

## 4. Assignment Operators

| Operator | Description | Example |
|----------|-------------|---------|
| `=` | Assignment | `x = 5` |
| `+=` | Add and assign | `x += 3` |
| `-=` | Subtract and assign | `x -= 2` |
| `*=` | Multiply and assign | `x *= 4` |
| `/=` | Divide and assign | `x /= 2` |

```aly
let score = 100
score += 50    # score = 150
score -= 20    # score = 130
```

---

## 5. String Operators

| Operator | Description | Example |
|----------|-------------|---------|
| `+` | Concatenation | `"Hello " + "World"` |
| `*` | Repetition | `"Ha" * 3` → `"HaHaHa"` |

---

## 6. Operator Precedence (highest to lowest)

1. `**` (power)
2. `++` `--` `not` `!` (unary)
3. `*` `/` `%`
4. `+` `-`
5. `gt` `lt` `gte` `lte` `eq` `ne` (comparisons)
6. `and`
7. `or`
8. `=` `+=` `-=` `*=` `/=` (assignment)