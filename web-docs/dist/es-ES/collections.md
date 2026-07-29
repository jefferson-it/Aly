# Colecciones en Aly

Más allá de listas/vectores y objetos, Aly proporciona tuplas, conjuntos, pilas, colas, listas enlazadas y enumeraciones.

---

## 1. Tuplas

Una colección ordenada e inmutable de valores encerrados entre paréntesis:

```aly
let point = (10, 20)
print(point[0])  # Outputs: 10
print(point[1])  # Outputs: 20
```

```aly
let mixed = ("Aly", 42, true)
print(mixed.len)  # Outputs: 3
```

---

## 2. Conjuntos

Una colección no ordenada de valores únicos:

```aly
let unique = {1, 2, 3, 3, 2}
print(unique)  # Outputs: Set containing {1, 2, 3}
```

Los duplicados se eliminan automáticamente al crear el conjunto.

---

## 3. Pilas

Estructura de datos de Último en Entrar, Primero en Salir (LIFO):

```aly
let stack = Stack.new()
stack.push(10)
stack.push(20)
let top = stack.pop()  # Returns 20
let len = stack.len()  # Returns 1
```

- `push(value)`: Añadir elemento en la parte superior
- `pop()`: Eliminar y devolver el elemento superior
- `len()`: Número de elementos

---

## 4. Colas

Estructura de datos de Primero en Entrar, Primero en Salir (FIFO):

```aly
let queue = Queue.new()
queue.enqueue("first")
queue.enqueue("second")
let item = queue.dequeue()  # Returns "first"
```

- `enqueue(value)`: Añadir elemento al final
- `dequeue()`: Eliminar y devolver el elemento del frente
- `len()`: Número de elementos

---

## 5. Listas Enlazadas

Lista simplemente enlazada con inserción en la cabeza:

```aly
let list = LinkedList.new()
list.push(10)
list.push(20)
list.push(30)
print(list.len)  # Outputs: 3
```

- `push(value)`: Insertar en la cabeza
- `len()`: Número de nodos

---

## 6. Enumeraciones

Conjuntos con nombre de variantes con datos asociados opcionales:

```aly
let status = Enum("Status", {
    "active": true,
    "inactive": false,
    "pending": null
})
```