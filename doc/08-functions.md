# Functions

## Declaration

Declare a function with `fun`:

```aly
fun add(a, b) {
    return a + b
}

let result = add(2, 3)   # 5
```

## Return values

`return` ends the function and yields a value. A function with no `return`
yields `None`.

```aly
fun greet(name) {
    return "Hello, $name"
}
```

## Multiple return values

A function can return several values separated by commas. The caller
destructures them into multiple variables:

```aly
fun stats(a, b) {
    return a + b, a - b
}

let sum, diff = stats(10, 4)      # sum = 14, diff = 6
```

Destructuring also works when assigning to existing variables:

```aly
sum, diff = stats(20, 5)          # sum = 25, diff = 15
```

## The discard variable `_`

Use `_` to ignore a value — as a parameter, an assignment target, or one of
several destructured returns:

```aly
let val, _ = stats(10, 4)         # keep sum, ignore diff

fun ignore_first(_, val) {
    return val
}
```

## Async / await

Prefix a function with `async` to make it return a **promise**. Use `await` to
resolve it:

```aly
async fun fetch_data(x) {
    return x * 2
}

let p = fetch_data(15)            # a promise
let value = await p               # 30
```

`await` on a non-promise value simply returns the value unchanged:

```aly
let n = await 42                  # 42
```

## Built-in functions

| Function            | Description                                   |
| ------------------- | --------------------------------------------- |
| `print(x)`          | Print a value followed by a newline.          |
| `input(prompt)`     | Print a prompt and read a line from stdin.    |
| `pow(base, exp)`    | Exponentiation.                               |
| `sqrt(v)`           | Square root.                                  |
| `random()`          | Pseudo-random float in `[0, 1)`.              |
| `round(v)`          | Round to nearest integer.                     |
| `roundUp(v)`        | Ceiling.                                       |
| `roundDown(v)`      | Floor.                                          |
| `to_fixed(v, p)`    | Format `v` with `p` decimal places (string).  |

```aly
print(pow(2, 3))          # 8
print(sqrt(16))           # 4
print(to_fixed(2.5, 2))   # "2.50"
```
