# Operadores en Aly

Aly admite operadores aritméticos, de comparación, lógicos y de asignación. Los operadores de comparación usan nombres basados en texto.

---

## 1. Operadores Aritméticos

| Operador | Descripción | Ejemplo |
|----------|-------------|---------|
| `+` | Suma | `3 + 5` → `8` |
| `-` | Resta | `10 - 4` → `6` |
| `*` | Multiplicación | `3 * 4` → `12` |
| `/` | División | `10 / 3` → `3.333` |
| `%` | Módulo (resto) | `10 % 3` → `1` |
| `**` | Potencia | `2 ** 3` → `8` |
| `++` | Incremento (postfijo) | `i++` → `i = i + 1` |
| `--` | Decremento (postfijo) | `i--` → `i = i - 1` |

```aly
let x = 10
x = x + 5       # 15
x++             # 16 (descompilado a x = x + 1)
```

---

## 2. Operadores de Comparación (Relacionales)

Aly utiliza operadores de comparación basados en **texto**:

| Operador | Significado | Ejemplo |
|----------|-------------|---------|
| `eq` | Igual a | `5 eq 5` → `true` |
| `ne` | No igual a | `5 ne 3` → `true` |
| `gt` | Mayor que | `5 gt 3` → `true` |
| `lt` | Menor que | `5 lt 3` → `false` |
| `gte` | Mayor o igual que | `5 gte 5` → `true` |
| `lte` | Menor o igual que | `5 lte 3` → `false` |

```aly
if age gte 18 {
    print("Adult")
}
```

---

## 3. Operadores Lógicos

| Operador | Descripción | Ejemplo |
|----------|-------------|---------|
| `and` | AND Lógico | `true and false` → `false` |
| `or` | OR Lógico | `true or false` → `true` |
| `not` | NOT Lógico | `not true` → `false` |
| `!` | NOT Lógico (alias) | `!true` → `false` |

```aly
if age gte 18 and has_id eq true {
    print("Allowed")
}
```

---

## 4. Operadores de Asignación

| Operador | Descripción | Ejemplo |
|----------|-------------|---------|
| `=` | Asignación | `x = 5` |
| `+=` | Sumar y asignar | `x += 3` |
| `-=` | Restar y asignar | `x -= 2` |
| `*=` | Multiplicar y asignar | `x *= 4` |
| `/=` | Dividir y asignar | `x /= 2` |

```aly
let score = 100
score += 50    # score = 150
score -= 20    # score = 130
```

---

## 5. Operadores de Cadenas

| Operador | Descripción | Ejemplo |
|----------|-------------|---------|
| `+` | Concatenación | `"Hello " + "World"` |
| `*` | Repetición | `"Ha" * 3` → `"HaHaHa"` |

---

## 6. Precedencia de Operadores (de mayor a menor)

1. `**` (potencia)
2. `++` `--` `not` `!` (unario)
3. `*` `/` `%`
4. `+` `-`
5. `gt` `lt` `gte` `lte` `eq` `ne` (comparaciones)
6. `and`
7. `or`
8. `=` `+=` `-=` `*=` `/=` (asignación)