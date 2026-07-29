# Cadenas de Texto en Aly

Las cadenas son secuencias de caracteres encerradas entre comillas dobles. Admiten interpolación de plantillas, concatenación y un rico conjunto de operaciones integradas.

---

## 1. Literales de Cadena

```aly
let greeting = "Hello, World!"
let empty = ""
```

Secuencias de escape: `\n` (nueva línea), `\t` (tabulación), `\\` (barra invertida), `\"` (comilla doble), `\'` (comilla simple).

---

## 2. Interpolación de Cadenas

Aly admite dos sintaxis de interpolación dentro de cadenas entre comillas dobles:

### `$variable` — Referencia directa a variable

```aly
let name = "Aly"
let msg = "Welcome to $name!"
print(msg)   # Outputs: Welcome to Aly!
```

### `&expression` — Evaluación de expresiones

```aly
let a = 10
let b = 5
let msg = "Sum is &(a + b)"
print(msg)   # Outputs: Sum is 15
```

La sintaxis `$` funciona para nombres de variables simples. La sintaxis `&` evalúa expresiones arbitrarias.

---

## 3. Concatenación y Repetición de Cadenas

```aly
let hello = "Hello" + " " + "World"   # "Hello World"
let repeated = "Ha" * 3               # "HaHaHa"
```

---

## 4. Operaciones Integradas de Cadenas

Las cadenas tienen métodos accesibles a través del módulo `str`:

```aly
import str

let text = "  Hello Aly  "
print(str.trim(text))        # Elimina espacios en blanco: "Hello Aly"
print(str.len(text))         # Longitud: 12
print(str.lower(text))       # Minúsculas: "  hello aly  "
print(str.upper(text))       # Mayúsculas: "  HELLO ALY  "
print(str.contains(text, "Aly"))  # true
print(str.starts_with(text, "  He"))  # true
print(str.ends_with(text, "  "))  # true
```

### Indexación de Cadenas

```aly
let text = "Aly"
print(text[0])   # Outputs: "A"
print(text[1])   # Outputs: "l"
```

---

## 5. Cadenas Multilinea

La continuación de líneas con barra invertida `\` permite dividir cadenas largas en varias líneas:

```aly
let msg = "This is a very long string " \
          "that continues on the next line"
```