# Bucles en Aly

Los bucles te permiten repetir instrucciones de bloque. Aly unifica los bucles en la palabra clave `loop`.

---

## 1. Bucle Condicional (Estilo While)

Evalúa la expresión antes de entrar en el bucle. Se repite mientras la condición sea `true`.

```aly
let i = 1
loop i lte 5 {
    print(i)
    i = i + 1
}
```

---

## 2. Bucle de Iteración (Estilo For)

Puedes especificar inicializaciones, condiciones e incrementos del bucle usando puntos y coma:

```aly
loop let i = 0; i lt 5; i = i + 1 {
    print(i)
}
```

---

## 3. Sentencias de Control del Bucle

* **`break` / `pare`**: Sale del bucle inmediatamente.
* **`continue`**: Salta directamente a la siguiente iteración del bucle, omitiendo las instrucciones siguientes.