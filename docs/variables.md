# Variables and Mutability in Aly

Aly provides support for both mutable variables and immutable constants. It uses block-based lexical scoping and dynamic type inference.

---

## 1. Variable Declarations (`let`)

The `let` keyword is used to declare mutable variables. A mutable variable can be rebound to a different value at any point after initialization.

```aly
let score = 100
score = score + 50
print(score) # Outputs: 150
```

---

## 2. Constant Declarations (`const`)

The `const` keyword is used to declare immutable constants. Once initialized, the value of a constant cannot be changed. Attempting to assign a new value to a constant will result in a compiler/interpreter error.

```aly
const PI = 3.14159
# PI = 3.2 # Error: Constant reassignment
```

---

## 3. Lexical Scoping

Variables and constants in Aly are block-scoped. A block is defined by curly braces `{}`.

* **Global Scope**: Variables declared outside of any block are accessible anywhere in the file.
* **Local Scope**: Variables declared inside a block are only visible within that block and any nested blocks. Once execution exits the block, the local variable is discarded.

```aly
let x = 10

if true {
    let y = 20
    print(x + y) # Outputs: 30
}

# print(y) # Error: y is not defined in this scope
```

---

## 4. Dynamic Type Inference

Aly does not require explicit type signatures (like `int x = 10`). The compiler/interpreter automatically determines the type based on the value assigned at runtime or compile-time.
