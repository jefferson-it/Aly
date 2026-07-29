# Cadeias de Caracteres em Aly

Cadeias de caracteres são sequências de caracteres entre aspas duplas. Suportam interpolação de modelos, concatenação e um conjunto rico de operações integradas.

---

## 1. Literais de Cadeia de Caracteres

```aly
let greeting = "Hello, World!"
let empty = ""
```

Sequências de escape: `\n` (nova linha), `\t` (tabulação), `\\` (barra invertida), `\"` (aspa dupla), `\'` (aspa simples).

---

## 2. Interpolação de Cadeia de Caracteres

Aly suporta duas sintaxes de interpolação dentro de cadeias de caracteres entre aspas duplas:

### `` — Referência direta de variável

```aly
let name = "Aly"
let msg = "Welcome to !"
print(msg)   # Resultado: Welcome to Aly!
```

### `&expression` — Avaliação de expressão

```aly
let a = 10
let b = 5
let msg = "Sum is &(a + b)"
print(msg)   # Resultado: Sum is 15
```

A sintaxe `$` funciona para nomes de variáveis simples. A sintaxe `&` avalia expressões arbitrárias.

---

## 3. Concatenação e Repetição de Cadeias de Caracteres

```aly
let hello = "Hello" + " " + "World"   # "Hello World"
let repeated = "Ha" * 3               # "HaHaHa"
```

---

## 4. Operações Integradas de Cadeia de Caracteres

As cadeias de caracteres têm métodos acessíveis através do módulo `str`:

```aly
import str

let text = "  Hello Aly  "
print(str.trim(text))        # Remove espaços em branco: "Hello Aly"
print(str.len(text))         # Comprimento: 12
print(str.lower(text))       # Minúsculas: "  hello aly  "
print(str.upper(text))       # Maiúsculas: "  HELLO ALY  "
print(str.contains(text, "Aly"))  # true
print(str.starts_with(text, "  He"))  # true
print(str.ends_with(text, "  "))  # true
```

### Indexação de Cadeias de Caracteres

```aly
let text = "Aly"
print(text[0])   # Resultado: "A"
print(text[1])   # Resultado: "l"
```

---

## 5. Cadeias de Caracteres de Várias Linhas

Continuação de linha com barra invertida `\` permite dividir cadeias longas em várias linhas:

```aly
let msg = "This is a very long string " \
          "that continues on the next line"
```
