# Aly Language — Documentation

Aly is a small, dynamically-typed interpreted language written in Rust. Programs
are stored in `.aly` files and executed with:

```sh
Aly run path/to/file.aly
```

This guide documents the language **as implemented** in the interpreter.

## Table of Contents

1. [Getting Started](01-getting-started.md)
2. [Syntax & Comments](02-syntax.md)
3. [Variables & Constants](03-variables.md)
4. [Types](04-types.md)
5. [Operators](05-operators.md)
6. [Strings & Template Interpolation](06-strings.md)
7. [Control Flow (if / loop / match)](07-control-flow.md)
8. [Functions](08-functions.md)
9. [Objects & Vectors](09-objects-and-vectors.md)
10. [Modules (import / export)](10-modules.md)
11. [Standard Library](11-stdlib.md)

---

## Quick example

```aly
# A tiny program

let name = input("What is your name?")

if str.trim(name).len gt 0 {
    print("Hello, &name!")
} else {
    print("Hello, stranger!")
}

fun square(n) {
    return n * n
}

print("5 squared is:")
print(square(5))
```
