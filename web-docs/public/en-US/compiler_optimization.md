# Compiler Optimization in Aly

The Aly compiler includes optimization passes for both the AOT compiler and the JIT VM.

---

## 1. Optimization Passes

The compiler runs a series of optimization passes on the MIR (Mid-level Intermediate Representation):

- **Constant Folding**: Evaluates constant expressions at compile time (`const_eval`)
- **Dead Code Elimination**: Removes unreachable code
- **Inlining**: Replaces function calls with the function body when beneficial
- **Strength Reduction**: Replaces expensive operations with cheaper ones

---

## 2. Incremental Compilation

```bash
alyc -o output input.aly --incremental
```

Caches previously compiled modules and only recompiles changed files:

```aly
# Cache stored in .aly_cache/ directory
# Automatically detects changes via file timestamps and hashes
```

---

## 3. Parallel Compilation

```bash
alyc -o output input.aly --parallel
```

Compiles independent modules in parallel using multiple threads, reducing total build time for multi-module projects.

---

## 4. JIT Optimizations

The VM's JIT (`jit.rs`) optimizes hot code paths:

- **Inline Caching**: Caches method lookup results (`inline_cache.rs`)
- **Fast Call Convention**: Optimized argument passing (`fastcall.rs`, `call_conventions.rs`)
- **Optimized VM**: Tuned bytecode execution (`optimized.rs`)
