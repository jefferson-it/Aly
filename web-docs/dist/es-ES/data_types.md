# Tipos de Datos en Aly

Aly es de tipado dinámico pero internamente admite un rico conjunto de tipos de datos primitivos y compuestos.

---

## 1. Primitivos

* **Entero (`int`)**: Enteros con signo de 64 bits (por ejemplo, `42`, `-10`).
* **Punto Flotante (`float`)**: Números de punto flotante IEEE 754 de 64 bits (por ejemplo, `3.14159`, `-0.001`).
* **Booleano (`bool`)**: Valores lógicos, ya sea `true` o `false`.
* **Carácter (`char`)**: Literales de carácter de un byte encerrados entre comillas simples (por ejemplo, `'A'`, `'\n'`, `'\t'`).
* **Cadena (`string`)**: Secuencia de caracteres encerrada entre comillas dobles (por ejemplo, `"Hello, World"`). Admite interpolación de plantillas de cadena usando la sintaxis `$variable`.
* **Nulo (`null`)**: Un tipo especial que representa la ausencia de valor, accedido mediante la palabra clave `None`.
* **Vacío**: Indica una función o expresión que no devuelve ningún valor.

---

## 2. Lista / Vector (`list`)

Los vectores son matrices dinámicas ordenadas que pueden contener elementos de cualquier tipo (incluyendo vectores u objetos anidados).

```aly
let numbers = [1, 2, 3, 4]
let mixed = ["Aly", 10.5, true, None]

# Acceso por índice
print(numbers[0]) # Outputs: 1
```

---

## 3. Objetos / Mapas (`object`)

Los objetos son asignaciones clave-valor que representan estructuras de diccionario o array asociativo. Las claves son identificadores o cadenas de texto.

```aly
let user = {
    name: "Jefferson",
    role: "Developer",
    active: true
}

# Acceso a miembros
print(user.name) # Outputs: Jefferson
print(user["role"]) # Outputs: Developer
```