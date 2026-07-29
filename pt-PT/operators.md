# Operadores em Aly

Aly suporta operadores aritméticos, de comparação, lógicos e de atribuição. Os operadores de comparação utilizam nomes em estilo de texto.

---

## 1. Operadores Aritméticos

| Operador | Descrição | Exemplo |
|----------|-----------|---------|
| `+` | Adição | `3 + 5` → `8` |
| `-` | Subtracção | `10 - 4` → `6` |
| `*` | Multiplicação | `3 * 4` → `12` |
| `/` | Divisão | `10 / 3` → `3.333` |
| `%` | Módulo (resto) | `10 % 3` → `1` |
| `**` | Potência | `2 ** 3` → `8` |
| `++` | Incremento (sufixo) | `i++` → `i = i + 1` |
| `--` | Decremento (sufixo) | `i--` → `i = i - 1` |

```aly
let x = 10
x = x + 5       # 15
x++             # 16 (desugared to x = x + 1)
```

---

## 2. Operadores de Comparação (Relacionais)

Aly utiliza operadores de comparação baseados em texto:

| Operador | Significado | Exemplo |
|----------|-------------|---------|
| `eq` | Igual a | `5 eq 5` → `true` |
| `ne` | Diferente de | `5 ne 3` → `true` |
| `gt` | Maior que | `5 gt 3` → `true` |
| `lt` | Menor que | `5 lt 3` → `false` |
| `gte` | Maior ou igual a | `5 gte 5` → `true` |
| `lte` | Menor ou igual a | `5 lte 3` → `false` |

```aly
if age gte 18 {
    print("Adult")
}
```

---

## 3. Operadores Lógicos

| Operador | Descrição | Exemplo |
|----------|-------------|---------|
| `and` | E Lógico | `true and false` → `false` |
| `or` | Ou Lógico | `true or false` → `true` |
| `not` | Não Lógico | `not true` → `false` |
| `!` | Não Lógico (alias) | `!true` → `false` |

```aly
if age gte 18 and has_id eq true {
    print("Allowed")
}
```

---

## 4. Operadores de Atribuição

| Operador | Descrição | Exemplo |
|----------|-------------|---------|
| `=` | Atribuição | `x = 5` |
| `+=` | Adicionar e atribuir | `x += 3` |
| `-=` | Subtract and assign | `x -= 2` |
| `*=` | Multiplicar e atribuir | `x *= 4` |
| `/=` | Dividir e atribuir | `x /= 2` |

```aly
let score = 100
score += 50    # score = 150
score -= 20    # score = 130
```

---

## 5. Operadores de Cadeia de Caracteres

| Operador | Descrição | Exemplo |
|----------|-------------|---------|
| `+` | Concatenação | `"Hello " + "World"` |
| `*` | Repetição | `"Ha" * 3` → `"HaHaHa"` |

---

## 6. Prioridade dos Operadores (mais alta a mais baixa)

1. `**` (potência)
2. `++` `--` `not` `!` (unário)
3. `*` `/` `%`
4. `+` `-`
5. `gt` `lt` `gte` `lte` `eq` `ne` (comparações)
6. `and`
7. `or`
8. `=` `+=` `-=` `*=` `/=` (atribuição)
