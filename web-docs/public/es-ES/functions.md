# Funciones en Aly

Las funciones son valores de primera clase definidos con la palabra clave `fun`. Pueden ser nombradas, anónimas, almacenadas en variables, pasadas como argumentos y devueltas desde otras funciones.

---

## 1. Definición de Función Nombrada

```aly
fun add(a, b) {
    return a + b
}

print(add(3, 4))  # Outputs: 7
```

Los parámetros son posicionales. Se requieren paréntesis alrededor de los parámetros. Las llaves delimitan el cuerpo de la función.

---

## 2. Valores de Retorno

Una función devuelve el valor de la última expresión evaluada, o explícitamente con `return`:

```aly
fun square(n) {
    return n * n
}

fun double(n) {
    n * 2      # Retorno implícito (última expresión)
}

print(square(4))   # Outputs: 16
print(double(5))   # Outputs: 10
```

`return` sin una expresión devuelve `None`.

---

## 3. Funciones Anónimas (Lambdas/Cierres)

Las funciones sin nombre pueden asignarse a variables o pasarse en línea:

```aly
let multiply = fun(a, b) {
    return a * b
}

print(multiply(3, 4))  # Outputs: 12
```

Las funciones anónimas capturan variables de su ámbito exterior (cierres).

```aly
let factor = 2
let doubler = fun(n) {
    return n * factor   # Captura `factor` del ámbito exterior
}

print(doubler(5))  # Outputs: 10
```

---

## 4. Parámetros por Defecto

Los parámetros pueden tener valores predeterminados:

```aly
fun greet(name, greeting = "Hello") {
    print(greeting + ", " + name)
}

greet("Alice")              # Outputs: Hello, Alice
greet("Bob", "Hi")          # Outputs: Hi, Bob
```

---

## 5. Funciones Variádicas

Un parámetro variádico recoge los argumentos adicionales en una lista usando `...`:

```aly
fun sum_all(...nums) {
    let total = 0
    loop let i = 0; i lt nums.len; i = i + 1 {
        total = total + nums[i]
    }
    return total
}

print(sum_all(1, 2, 3, 4))  # Outputs: 10
```

Solo se permite un parámetro variádico y debe ser el último parámetro.

---

## 6. Funciones de Primera Clase

Las funciones pueden almacenarse en estructuras de datos y pasarse de una a otra:

```aly
let operations = {
    add: fun(a, b) { return a + b },
    sub: fun(a, b) { return a - b },
}

print(operations.add(10, 5))  # Outputs: 15
```