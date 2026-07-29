# Strings in Aly

Strings are sequences of characters enclosed in double quotes. They support template interpolation, concatenation, and a rich set of built-in operations.

---

## 1. String Literals

```aly
let greeting = "Hello, World!"
let empty = ""
```

Escape sequences: `\n` (newline), `\t` (tab), `\\` (backslash), `\"` (double quote), `\'` (single quote).

---

## 2. String Interpolation

Aly supports two interpolation syntaxes inside double-quoted strings:

### `$variable` — Direct variable reference

```aly
let name = "Aly"
let msg = "Welcome to $name!"
print(msg)   # Outputs: Welcome to Aly!
```

### `&expression` — Expression evaluation

```aly
let a = 10
let b = 5
let msg = "Sum is &(a + b)"
print(msg)   # Outputs: Sum is 15
```

The `$` syntax works for simple variable names. The `&` syntax evaluates arbitrary expressions.

---

## 3. String Concatenation and Repetition

```aly
let hello = "Hello" + " " + "World"   # "Hello World"
let repeated = "Ha" * 3               # "HaHaHa"
```

---

## 4. Built-in String Operations

Strings have methods accessible through the `str` module:

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

### String indexing

```aly
let text = "Aly"
print(text[0])   # Outputs: "A"
print(text[1])   # Outputs: "l"
```

---

## 5. Multiline Strings

Line continuation with backslash `\` allows splitting long strings across lines:

```aly
let msg = "This is a very long string " \
          "that continues on the next line"
```