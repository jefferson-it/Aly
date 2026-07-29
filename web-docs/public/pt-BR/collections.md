# Coleções no Aly

Além de listas/vetores e objetos, o Aly fornece tuples, sets, stacks, queues, linked lists e enumerations.

---

## 1. Tuples

Uma coleção ordenada e imutável de valores entre parênteses:

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

## 2. Sets

Uma coleção não ordenada de valores únicos:

```aly
let unique = {1, 2, 3, 3, 2}
print(unique)  # Outputs: Set containing {1, 2, 3}
```

Duplicatas são removidas automaticamente na criação.

---

## 3. Stacks

Estrutura de dados Last-In-First-Out (LIFO):

```aly
let stack = Stack.new()
stack.push(10)
stack.push(20)
let top = stack.pop()  # Returns 20
let len = stack.len()  # Returns 1
```

- `push(value)`: Adiciona item no topo
- `pop()`: Remove e retorna o item do topo
- `len()`: Número de itens

---

## 4. Queues

Estrutura de dados First-In-First-Out (FIFO):

```aly
let queue = Queue.new()
queue.enqueue("first")
queue.enqueue("second")
let item = queue.dequeue()  # Returns "first"
```

- `enqueue(value)`: Adiciona item no fim
- `dequeue()`: Remove e retorna o item da frente
- `len()`: Número de itens

---

## 5. Linked Lists

Lista simplesmente encadeada com inserção na cabeça (head):

```aly
let list = LinkedList.new()
list.push(10)
list.push(20)
list.push(30)
print(list.len)  # Outputs: 3
```

- `push(value)`: Insere na cabeça
- `len()`: Número de nós

---

## 6. Enumerations

Conjuntos nomeados de variantes com dados associados opcionais:

```aly
let status = Enum("Status", {
    "active": true,
    "inactive": false,
    "pending": null
})
```