# Metaprogramming in Aly

Aly supports compile-time code generation and transformation through macros and lazy evaluation.

---

## 1. Macro Definitions

```aly
macrodef assert(condition) {
    if not (condition) {
        print("Assertion failed")
    }
}

let x = 10
assert(x gt 5)  # Expands at compile time
```

Macros receive token trees and produce AST nodes that are spliced into the calling code.

---

## 2. Lazy Evaluation

```aly
lazy let expensive = compute_large_value()
# The expression is not evaluated until first access

print(expensive)  # Evaluated here
```

Lazy variables are initialized only on their first access, useful for deferred computation and circular references.

---

## 3. Compile-Time Constant Evaluation

The `const_eval` module evaluates expressions at compile time when all inputs are known constants:

```aly
const SIZE = 100
const AREA = SIZE * SIZE  # Evaluated at compile time
```

---

## 4. AST Hooks

The compiler's `metaprogramming` module provides hooks for transforming the AST during compilation, enabling custom syntax extensions and DSL embedding.