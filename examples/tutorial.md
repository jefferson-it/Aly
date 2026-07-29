# Aly-lang Tutorial for Beginners

## Introduction
Welcome to Aly-lang! This guide will walk you through the basics of the language, from installation to writing your first program.

## Installation
1. Install Rust: `curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh`
2. Install Aly interpreter: `apg install aly-lang`

## Hello World
Create a file `hello.aly`:
```aly
let message = "Olá, mundo!"
print(message)
```

Run it:
```bash
aly run hello.aly
```

## Variables and Types
- Declare mutable variables with `let`: `let x = 10`
- Declare constants with `const`: `const PI = 3.14`
- Basic types: `number`, `string`, `boolean`, `array`, `object`

## Control Flow
- Conditional statements: `if`, `elif`, `else`
- Loops: `loop`, `while`, `for` (in progress)
- Match expressions: `match value { pattern => expr }`

## Functions
Define a function:
```aly
fun add(a, b) {
    return a + b
}
```

## Structs
Define a struct:
```aly
struct Point {
    x: number,
    y: number,
}

let p = Point(10, 20)
print(p.x)
```

## Error Handling
- Use `try` / `catch` blocks:
```aly
try {
    // code that may fail
} catch err {
    print(err)
}
```

## Packages
Use APG to manage packages:
```bash
apg init myproject
apg add http-client
apg install http-client
```

## Next Steps
- Explore the `examples/` directory for more code samples.
- Read the documentation in `doc/` for detailed explanations of each feature.
- Contribute to the project by submitting issues or pull requests.

Happy coding! 🚀