# Metaprogramación en Aly

Aly admite la generación y transformación de código en tiempo de compilación a través de macros y evaluación perezosa.

---

## 1. Definición de Macros

```aly
macrodef assert(condition) {
    if not (condition) {
        print("Assertion failed")
    }
}

let x = 10
assert(x gt 5)  # Se expande en tiempo de compilación
```

Las macros reciben árboles de tokens y producen nodos AST que se insertan en el código de llamada.

---

## 2. Evaluación Perezosa

```aly
lazy let expensive = compute_large_value()
# La expresión no se evalúa hasta el primer acceso

print(expensive)  # Evaluado aquí
```

Las variables perezosas se inicializan solo en su primer acceso, útiles para el cálculo diferido y las referencias circulares.

---

## 3. Evaluación de Constantes en Tiempo de Compilación

El módulo `const_eval` evalúa expresiones en tiempo de compilación cuando todas las entradas son constantes conocidas:

```aly
const SIZE = 100
const AREA = SIZE * SIZE  # Evaluado en tiempo de compilación
```

---

## 4. Ganchos de AST

El módulo `metaprogramming` del compilador proporciona ganchos para transformar el AST durante la compilación, permitiendo extensiones de sintaxis personalizadas e incrustación de DSL.