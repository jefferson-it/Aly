# Operators

## Arithmetic

| Operator | Meaning        | Example      | Result |
| -------- | -------------- | ------------ | ------ |
| `+`      | addition       | `2 + 3`      | `5`    |
| `-`      | subtraction    | `5 - 2`      | `3`    |
| `*`      | multiplication | `4 * 2`      | `8`    |
| `/`      | division       | `10 / 4`     | `2.5`  |
| `\|`     | modulus        | `10 \| 3`    | `1`    |
| `%`      | percent        | `50 % 200`   | `100`  |

Notes:

- The modulus operator is `|` (not `%`).
- `%` computes a percentage: `a % b` means "a percent of b". `50 % 200` is
  `100`. On its own, `n %` becomes `n / 100`.

```aly
print(10 | 3)     # 1   (modulus)
print(20 % 150)   # 30  (20 percent of 150)
```

## Relational

Relational operators use word forms and produce a boolean:

| Operator | Meaning                  |
| -------- | ------------------------ |
| `eq`     | equal                    |
| `neq`    | not equal                |
| `lt`     | less than                |
| `lte`    | less than or equal       |
| `gt`     | greater than             |
| `gte`    | greater than or equal    |

```aly
if age gte 18 {
    print("adult")
}
```

Uppercase forms (`EQ`, `NEQ`, `LT`, `LTE`, `GT`, `GTE`) are also accepted.

## Logical

| Operator | Meaning       |
| -------- | ------------- |
| `and`    | logical AND   |
| `or`     | logical OR    |
| `xor`    | logical XOR   |
| `not`    | logical NOT   |

```aly
if x gt 0 and x lt 10 {
    print("between 1 and 9")
}
```

Uppercase forms (`AND`, `OR`, `XOR`, `NOT`) are also accepted.

## References

The `&` prefix takes a reference to a variable, used with `tomb` and
pointer-style arguments:

```aly
tomb &value
```
