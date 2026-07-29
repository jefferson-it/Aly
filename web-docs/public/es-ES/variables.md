# Variables y Mutabilidad en Aly

Aly admite tanto variables mutables como constantes inmutables. Utiliza alcance léxico basado en bloques e inferencia de tipo dinámica.

---

## 1. Declaración de Variables (`let`)

La palabra clave `let` se usa para declarar variables mutables. Una variable mutable se puede reasignar a un valor diferente en cualquier momento después de la inicialización.

```aly
let score = 100
score = score + 50
print(score) # Outputs: 150
```

---

## 2. Declaración de Constantes (`const`)

La palabra clave `const` se usa para declarar constantes inmutables. Una vez inicializado, el valor de una constante no puede cambiarse. Intentar asignar un nuevo valor a una constante resultará en un error del compilador/intérprete.

```aly
const PI = 3.14159
# PI = 3.2 # Error: Reasignación de constante
```

---

## 3. Alcance Léxico

Las variables y constantes en Aly tienen alcance a nivel de bloque. Un bloque se define mediante llaves `{}`.

* **Alcance Global**: Las variables declaradas fuera de cualquier bloque son accesibles en cualquier parte del archivo.
* **Alcance Local**: Las variables declaradas dentro de un bloque solo son visibles dentro de ese bloque y cualquier bloque anidado. Una vez que la ejecución sale del bloque, la variable local se descarta.

```aly
let x = 10

if true {
    let y = 20
    print(x + y) # Outputs: 30
}

# print(y) # Error: y no está definida en este alcance
```

---

## 4. Inferencia de Tipos Dinámica

Aly no requiere firmas de tipo explícitas (como `int x = 10`). El compilador/intérprete determina automáticamente el tipo en función del valor asignado en tiempo de ejecución o tiempo de compilación.