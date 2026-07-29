# Functions in Aly

Functions are first-class values defined with the `fun` keyword. They can be named, anonymous, stored in variables, passed as arguments, and returned from other functions.

---

## 1. Named Function Definition

```aly
fun add(a, b) {
    return a + b
}

print(add(3, 4))  # Outputs: 7
```

Parameters are positional. Parentheses around parameters are required. Curly braces delimit the function body.

---

## 2. Return Values

A function returns the value of the last expression evaluated, or explicitly with `return`:

```aly
fun square(n) {
    return n * n
}

fun double(n) {
    n * 2      # Implicit return (last expression)
}

print(square(4))   # Outputs: 16
print(double(5))   # Outputs: 10
```

`return` without an expression returns `None`.

---

## 3. Anonymous Functions (Lambdas/Closures)

Functions without a name can be assigned to variables or passed inline:

```aly
let multiply = fun(a, b) {
    return a * b
}

print(multiply(3, 4))  # Outputs: 12
```

Anonymous functions capture variables from their enclosing scope (closures).

```aly
let factor = 2
let doubler = fun(n) {
    return n * factor   # Captures `factor` from outer scope
}

print(doubler(5))  # Outputs: 10
```

---

## 4. Default Parameters

Parameters can have default values:

```aly
fun greet(name, greeting = "Hello") {
    print(greeting + ", " + name)
}

greet("Alice")              # Outputs: Hello, Alice
greet("Bob", "Hi")          # Outputs: Hi, Bob
```

---

## 5. Variadic Functions

A variadic parameter collects excess arguments into a list using `...`:

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

Only one variadic parameter is allowed, and it must be the last parameter.

---

## 6. First-Class Functions

Functions can be stored in data structures and passed around:

```aly
let operations = {
    add: fun(a, b) { return a + b },
    sub: fun(a, b) { return a - b },
}

print(operations.add(10, 5))  # Outputs: 15
```