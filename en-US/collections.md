# Collections in Aly

Beyond lists/vectors and objects, Aly provides tuples, sets, stacks, queues, linked lists, and enumerations.

---

## 1. Tuples

An ordered, immutable collection of values enclosed in parentheses:

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

An unordered collection of unique values:

```aly
let unique = {1, 2, 3, 3, 2}
print(unique)  # Outputs: Set containing {1, 2, 3}
```

Duplicates are automatically removed on creation.

---

## 3. Stacks

Last-In-First-Out (LIFO) data structure:

```aly
let stack = Stack.new()
stack.push(10)
stack.push(20)
let top = stack.pop()  # Returns 20
let len = stack.len()  # Returns 1
```

- `push(value)`: Add item to top
- `pop()`: Remove and return top item
- `len()`: Number of items

---

## 4. Queues

First-In-First-Out (FIFO) data structure:

```aly
let queue = Queue.new()
queue.enqueue("first")
queue.enqueue("second")
let item = queue.dequeue()  # Returns "first"
```

- `enqueue(value)`: Add item to back
- `dequeue()`: Remove and return front item
- `len()`: Number of items

---

## 5. Linked Lists

Singly-linked list with head insertion:

```aly
let list = LinkedList.new()
list.push(10)
list.push(20)
list.push(30)
print(list.len)  # Outputs: 3
```

- `push(value)`: Insert at head
- `len()`: Number of nodes

---

## 6. Enumerations

Named sets of variants with optional associated data:

```aly
let status = Enum("Status", {
    "active": true,
    "inactive": false,
    "pending": null
})
```
