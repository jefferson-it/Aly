# Compiler Backends in Aly

Aly's AOT compiler (`alyc`) can transpile code to multiple target languages and platforms.

---

## 1. Available Backends

| Backend | Command | Output |
|---------|---------|--------|
| Native ELF | `alyc -o output input.aly` | Linux executable |
| Windows PE | `alyc -o output.exe input.aly` | Windows executable |
| Go | `alyc -o output.go input.aly` | Go source code |
| Rust | `alyc -o output.rs input.aly` | Rust source code |
| JavaScript | `alyc -o output.js input.aly` | JavaScript source code |
| Python | `alyc -o output.py input.aly` | Python source code |
| JVM Bytecode | `alyc -o output.class input.aly` | Java bytecode |
| Kotlin | `alyc -o output.kt input.aly` | Kotlin source code |
| C++ | `alyc -o output.cpp input.aly` | C++ source code |
| Shell/Bash | `alyc -o output.sh input.aly` | Shell script |

---

## 2. LLVM Backend

Uses LLVM for native code generation with optimizations:

```bash
alyc -o output input.aly --backend llvm
```

---

## 3. C++ Backend

Transpiles Aly to C++ with a runtime header (`runtime_aly.h`):

```bash
alyc -o output.cpp input.aly
g++ -o output output.cpp runtime_aly.h
```

---

## 4. JVM/Bytecode Backend

Generates JVM bytecode directly (`.class` files):

```bash
alyc -o MyApp.class input.aly
java MyApp
```

---

## 5. Transpiler Details

Each backend converts Aly's HIR (High-level Intermediate Representation) into the target language's AST, then generates source code. The `cpp_abi` module handles C++ ABI compatibility for calling conventions.
