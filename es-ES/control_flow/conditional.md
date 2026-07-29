# Condicionales en Aly

Los condicionales te permiten dirigir el flujo de ejecución según expresiones booleanas.

---

## 1. Sentencias `if` / `elif` / `else`

La sentencia `if` evalúa una condición lógica. Se requieren llaves `{}` para encerrar los bloques de alcance.

```aly
let age = 18

if age gt 60 {
    print("Senior")
} elif age gte 18 {
    print("Adult")
} else {
    print("Minor")
}
```

---

## 2. Operadores de Comparación

Aly utiliza nombres de texto con estilo prefijo para los operadores de comparación estándar:
* **`eq`**: Igual (`==`)
* **`ne`**: No Igual (`!=`)
* **`gt`**: Mayor Que (`>`)
* **`lt`**: Menor Que (`<`)
* **`gte`**: Mayor Que o Igual (`>=`)
* **`lte`**: Menor Que o Igual (`<=`)

---

## 3. Operadores Lógicos

* **`and`**: AND Lógico. Devuelve `true` solo si ambas expresiones son verdaderas.
* **`or`**: OR Lógico. Devuelve `true` si al menos una expresión es verdadera.
* **`not` / `!`**: NOT Lógico. Invierte el valor booleano de la expresión.