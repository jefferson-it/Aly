# Math
Mathematical functions and constants.

| Function | Parameters | Returns | Description |
| ------------------------------------------------- | ---------- | ---------- | ------------------------------------------------- |
| `pow(base, exponent)` | `base: string`, `exponent: string` | `string` (quoted) | Compute `base` raised to the power of `exponent`. Accepts numeric strings; returns quoted result. |
| `sqrt(value)` | `value: string` | `string` (quoted) | Square root of `value`. |
| `random()` | *none* | `string` (quoted) | Pseudorandom float in `[0,1)`. |
| `round(value)` | `value: string` | `string` (quoted) | Round to nearest integer. |
| `roundUp(value)` | `value: string` | `string` (quoted) | Round up (ceil). |
| `roundDown(value)` | `value: string` | `string` (quoted) | Round down (floor). |
| `to_fixed(value, precision)` | `value: string`, `precision: string` | `string` (quoted) | Format `value` with `precision` decimal places. |
| `abs(value)` | `value: string` | `string` (quoted) | Absolute value. |
| `min(a, b)` | `a: string`, `b: string` | `string` (quoted) | Smaller of `a` and `b`. |
| `max(a, b)` | `a: string`, `b: string` | `string` (quoted) | Larger of `a` and `b`. |
| `sin(value)` | `value: string` | `string` (quoted) | Sine of `value` (radians). |
| `cos(value)` | `value: string` | `string` (quoted) | Cosine of `value` (radians). |
| `tan(value)` | `value: string` | `string` (quoted) | Tangent of `value` (radians). |
| `log(value)` | `value: string` | `string` (quoted) | Base‑10 logarithm. |
| `ln(value)` | `value: string` | `string` (quoted) | Natural logarithm (base e). |
| Constants `PI` and `E` | — | `string` (quoted) | `PI` → `"3.141592653589793"`; `E` → `"2.718281828459045"`. |

```aly
print(pow("2", "3"))          # → "8"
print(sqrt("16"))             # → "4"
print(random())               # → "0.374..."
print(to_fixed("3.14159", "4"))# → "3.1416"
print(sin("3.14159"))         # → "0.001..."
```

### Notes
- All inputs are parsed as `f64`; malformed inputs yield `0.0` and an error message.
- Return values are quoted strings; callers can strip quotes for numeric use.
- Trigonometric functions use Rust’s `std::f64` implementations.