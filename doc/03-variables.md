# Variables & Constants

## Declaring variables

Use `let` to declare a mutable variable:

```aly
let name = "Ana"
let age = 30
```

A variable can be declared without a value (it starts as `None`):

```aly
let result
result = 42
```

## Assignment

Assign to an existing variable with `=`:

```aly
let count = 0
count = count + 1
```

Variables are **type-stable**: once a variable holds a given type, assigning a
value of a different type raises an error. Reassign a value of the same type.

```aly
let n = 10
n = 20        # ok
# n = "text"  # error: n is an int
```

## Constants

Use `const` for values that must never change:

```aly
const PI = 3.14159
# PI = 3        # error: PI is constant
```

## Immutability with `tomb`

The `tomb` keyword freezes a previously-mutable variable, turning it immutable
from that point on. This is useful when a value is computed conditionally and
should not change afterwards:

```aly
let user = None

if id.len gt 0 {
    user = findUser(id)
}

tomb user       # user is now immutable
# user = "x"    # error
```

`tomb` accepts one or more variable names:

```aly
tomb a b c
```

## Increment / decrement

`++` and `--` are shorthand for `x = x + 1` and `x = x - 1`:

```aly
let i = 0
i++
i++
print(i)   # 2
```
