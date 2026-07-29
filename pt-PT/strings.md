# Strings em Aly

As strings são sequências de caracteres delimitadas por aspas duplas. Suportam interpolação de modelos, concatenação e um conjunto rico de operações integradas.

---

## 1. Literais de String

```aly
let greeting = "Hello, World!"
let empty = ""
```

Sequências de escape: `\n` (nova linha), `\t` (tabulação), `\\` (barra invertida), `\"` (aspa dupla), `\'` (aspa simples).

---

## 2. Interpolação de String

O Aly suporta duas sintaxes de interpolação dentro de strings com aspas duplas:

### `$variable` — Referência direta à variável

```aly
let name = "Aly"
let msg = "Welcome to $name!"
print(msg)   # Outputs: Welcome to Aly!
```

### `&expression` — Avaliação de expressão

```aly
let a = 10
let b = 5
let msg = "Sum is &(a + b)"
print(msg)   # Outputs: Sum is 15
```

A sintaxe `$` funciona para nomes de variáveis simples. A sintaxe `&` avalia expressões arbitrárias.

---

## 3. Concatenação e Repetição de Strings

```aly
let hello = "Hello" + " " + "World"   # "Hello World"
let repeated = "Ha" * 3               # "HaHaHa"
```

---

## 4. Operações Integradas de Strings

As strings possuem métodos acessíveis através do módulo `str`:

```aly
import str

let text = "  Hello Aly  "
print(str.trim(text))        # Removes whitespace: "Hello Aly"
print(str.len(text))         # Length: 12
print(str.lower(text))       # Lowercase: "  hello aly  "
print(str.upper(text))       # Uppercase: "  HELLO ALY  "
print(str.contains(text, "Aly"))  # true
print(str.starts_with(text, "  He"))  # true
print(str.ends_with(text, "  "))  # true
```

### Indexação de strings

```aly
let text = "Aly"
print(text[0])   # Outputs: "A"
print(text[1])   # Outputs: "l"
```

---

## 5. Strings de Várias Linhas

A continuação de linha com barra invertida `\` permite dividir strings longas em várias linhas:

```aly
let msg = "This is a very long string " \
          "that continues on the next line"
```
