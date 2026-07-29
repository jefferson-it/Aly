# Colecções em Aly

Além de listas/vetores e objectos, Aly fornece tuplos, conjuntos, pilhas, filas, listas ligadas e enumerações.

---

## 1. Tuplos

Uma colecção ordenada e imutável de valores entre parêntesis:

```aly
let point = (10, 20)
print(point[0])  # Resultado: 10
print(point[1])  # Resultado: 20
```

```aly
let mixed = ("Aly", 42, true)
print(mixed.len)  # Resultado: 3
```

---

## 2. Conjuntos

Uma colecção não ordenada de valores únicos:

```aly
let unique = {1, 2, 3, 3, 2}
print(unique)  # Resultado: Conjunto contendo {1, 2, 3}
```

Duplicatas são automaticamente removidas na criação.

---

## 3. Pilhas

Estrutura de dados Last-In-First-Out (LIFO):

```aly
let stack = Stack.new()
stack.push(10)
stack.push(20)
let top = stack.pop()  # Devolve 20
let len = stack.len()  # Devolve 1
```

- `push(value)`: Adicionar item ao topo
- `pop()`: Remover e devolver o item do topo
- `len()`: Número de itens

---

## 4. Filas

Estrutura de dados First-In-First-Out (FIFO):

```aly
let queue = Queue.new()
queue.enqueue("first")
queue.enqueue("second")
let item = queue.dequeue()  # Devolve "first"
```

- `enqueue(value)`: Adicionar item ao fundo
- `dequeue()`: Remover e devolver o item da frente
- `len()`: Número de itens

---

## 5. Listas Ligadas

Lista ligada simples com inserção na cabeça:

```aly
let list = LinkedList.new()
list.push(10)
list.push(20)
list.push(30)
print(list.len)  # Resultado: 3
```

- `push(value)`: Inserir na cabeça
- `len()`: Número de nós

---

## 6. Enumerações

Conjuntos nomeados de variantes com dados associados opcionais:

```aly
let status = Enum("Status", {
    "active": true,
    "inactive": false,
    "pending": null
})
```
