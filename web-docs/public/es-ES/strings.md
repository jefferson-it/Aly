# Cadenas de Texto en Aly

Las cadenas de texto son secuencias de caracteres encerradas entre comillas dobles. Soportan interpolación de plantillas, concatenación y un conjunto rico de operaciones incorporadas.

---

## 1. Literales de Cadena de Texto

```aly
let greeting = "Hello, World!"
let empty = ""
```

Secuencias de escape: `\n` (nueva línea), `\t` (tabulación), `\\` (barra invertida), `\"` (comilla doble), `\'` (comilla simple).

---

## 2. Interpolación de Cadenas de Texto

Aly soporta dos sintaxis de interpolación dentro de cadenas de texto con comillas dobles:

### `$variable` — Referencia directa a variable

```aly
let name = "Aly"
let msg = "Welcome to $name!"
print(msg)   # Outputs: Welcome to Aly!
```

### `&expression` — Evaluación de expresión

```aly
let a = 10
let b = 5
let msg = "Sum is &(a + b)"
print(msg)   # Outputs: Sum is 15
```

La sintaxis `$` funciona para nombres de variables simples. La sintaxis `&` evalúa expresiones arbitrarias.

---

## 3. Concatenación y Repetición de Cadenas de Texto

```aly
let hello = "Hello" + " " + "World"   # "Hello World"
let repeated = "Ha" * 3               # "HaHaHa"
```

---

## 4. Operaciones Incorporadas de Cadenas de Texto

Las cadenas de texto tienen métodos accesibles a través del módulo `str`:

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

### Indexación de cadenas de texto

```aly
let text = "Aly"
print(text[0])   # Outputs: "A"
print(text[1])   # Outputs: "l"
```

---

## 5. Cadenas de Texto Multilínea

La continuación de línea con la barra invertida `\` permite dividir cadenas de texto largas en varias líneas:

```aly
let msg = "This is a very long string " \
          "that continues on the next line"
```