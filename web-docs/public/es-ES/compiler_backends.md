# Backends de Compilador en Aly

El compilador AOT de Aly (`alyc`) puede transpilar código a múltiples lenguajes de destino y plataformas.

---

## 1. Backends Disponibles

| Backend | Comando | Salida |
|---------|---------|--------|
| Native ELF | `alyc -o output input.aly` | Ejecutable Linux |
| Windows PE | `alyc -o output.exe input.aly` | Ejecutable Windows |
| Go | `alyc -o output.go input.aly` | Código fuente de Go |
| Rust | `alyc -o output.rs input.aly` | Código fuente de Rust |
| JavaScript | `alyc -o output.js input.aly` | Código fuente de JavaScript |
| Python | `alyc -o output.py input.aly` | Código fuente de Python |
| JVM Bytecode | `alyc -o output.class input.aly` | Java bytecode |
| Kotlin | `alyc -o output.kt input.aly` | Código fuente de Kotlin |
| C++ | `alyc -o output.cpp input.aly` | Código fuente de C++ |
| Shell/Bash | `alyc -o output.sh input.aly` | Script de shell |

---

## 2. Backend LLVM

Utiliza LLVM para la generación de código nativo con optimizaciones:

```bash
alyc -o output input.aly --backend llvm
```

---

## 3. Backend C++

Transpila Aly a C++ con un encabezado de runtime (`runtime_aly.h`):

```bash
alyc -o output.cpp input.aly
g++ -o output output.cpp runtime_aly.h
```

---

## 4. Backend JVM/Bytecode

Genera JVM bytecode directamente (archivos `.class`):

```bash
alyc -o MyApp.class input.aly
java MyApp
```

---

## 5. Detalles del Transpilador

Cada backend convierte la representación intermedia de alto nivel (HIR) de Aly en el AST del lenguaje de destino, luego genera el código fuente. El módulo `cpp_abi` maneja la compatibilidad ABI de C++ para las convenciones de llamada.