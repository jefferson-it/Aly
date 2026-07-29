# Formato de Datos JOT en Aly

JOT es un formato de datos similar a JSON y un lenguaje de consultas integrado en Aly para la manipulación de datos estructurados.

---

## 1. Valores JOT

JOT admite los siguientes tipos de valores:

- **Nulo**: Ausencia de valor
- **Booleano**: `true` o `false`
- **Entero**: Enteros con signo de 64 bits
- **Flotante**: Números de punto flotante de 64 bits
- **Cadena**: Cadenas de texto
- **Arreglo**: Lista ordenada de valores JOT
- **Objeto**: Asignación clave-valor de valores JOT

---

## 2. Sintaxis JOT

```aly
let data = {
    "name": "Aly",
    "version": 1.0,
    "tags": ["dynamic", "fun"],
    "details": {
        "author": "Jefferson",
        "year": 2025
    }
}
```

---

## 3. Consultas con `pick`

El método `pick` recorre objetos anidados usando una secuencia de claves:

```aly
let val = data.pick(["details", "author"])
print(val)  # Outputs: "Jefferson"
```

Devuelve `None` si alguna clave en la cadena no existe.

---

## 4. Conversión a Cadena

```aly
let text = data.to_string()
print(text)
```

Devuelve el valor JOT como una cadena formateada, indentada para mayor legibilidad.

---

## 5. Runtime de JOT

El runtime de JOT (`jot.rs`) proporciona utilidades de análisis, formato y conversión. Sirve como representación interna de datos estructurados en todo el runtime de Aly.