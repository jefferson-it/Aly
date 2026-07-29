# Punto y Coma, Continuación de Líneas y Comentarios en Aly

Esta página documenta las reglas léxicas específicas sobre la terminación de sentencias, sentencias multilínea y delimitadores de comentarios en Aly.

---

## 1. Comentarios

Aly admite comentarios de una sola línea y comentarios multilínea (bloque).

* **Comentarios de una sola línea**: Comienzan con `#` y se extienden hasta el final de la línea.
* **Comentarios multilínea / de bloque**: Encerrados entre etiquetas `## ... ##`.

```aly
# This is a single-line comment

##
This is a multi-line block comment
spanning multiple lines of text
##
let x = 10
```

---

## 2. Semántica del Punto y Coma (`;`)

Los puntos y coma `;` **no** son terminadores de sentencias de propósito general en Aly. Las sentencias se terminan implícitamente con saltos de línea (nuevas líneas) o delimitadores de bloque `{}`.

### Puntos y Coma en Encabezados de Bucle (Único Caso Válido)
Los puntos y coma están únicamente permitidos dentro de los encabezados de bucle para separar las expresiones de inicialización, verificación de condición y actualización de iteración.

```aly
# Válido: Puntos y coma separando expresiones dentro de un encabezado de bucle
loop let i = 0; i lt 5; i = i + 1 {
    print(i)
}
```

### Puntos y Coma No Válidos
Usar puntos y coma en cualquier otro lugar del script de Aly resultará en un error del analizador sintáctico:
```aly
let x = 10; # Error: Punto y coma usado fuera de un encabezado de bucle
print(x);   # Error: Punto y coma usado fuera de un encabezado de bucle
```

---

## 3. Continuación de Línea (`\`)

Si necesita dividir una sentencia larga en varias líneas, agregue una barra invertida `\` al final de la línea. Esto instruye al analizador para que trate la línea siguiente como una continuación de la sentencia actual.

```aly
let msg = "Hello " + "World" \
          " from Aly!"
print(msg) # Outputs: Hello World from Aly!
```

La continuación de línea se detiene tan pronto como comienza un bloque `{}` o un bloque basado en palabras clave (como `do`/`od`).