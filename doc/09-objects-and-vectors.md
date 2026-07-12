# Objects & Vectors

## Vectors

A vector is an ordered list, written with square brackets:

```aly
let nums = [1, 2, 3]
let mixed = ["a", 2, true]
```

### Length and indexing

```aly
let nums = [10, 20, 30]
print(nums.len)     # 3
print(nums.0)       # 10
print(nums.2)       # 30
```

Vectors may nest:

```aly
let grid = [[1, 2], [3, 4]]
```

## Objects

An object holds key/value pairs, written with braces:

```aly
let person = {
    name: "Ana",
    age: 30
}
```

### Accessing properties

```aly
print(person.name)   # Ana
print(person.age)    # 30
print(person.len)    # 2   (number of keys)
```

### Self-reference with `@`

Within an object literal, `@` refers to a previously-defined property of the
same object:

```aly
let box = {
    width: 4,
    height: 3,
    area: @width * @height
}

print(box.area)      # 12
```

### Nested objects

```aly
let config = {
    server: {
        host: "localhost",
        port: 8080
    }
}

print(config.server.host)   # localhost
```

## Serialization

Objects and vectors print as a readable structure. Convert an object to its
JSON-like string form with `.to_str`.

```aly
print(person.to_str)
```
