# Backends del Compilador en Aly

El compilador AOT de Aly (`alyc`) puede transpilar código a múltiples lenguajes y plataformas de destino.

---

## 1. Backends Disponibles

| Backend | Comando | Salida |
|---------|---------|--------|
| ELF Nativo | `alyc -o output input.aly` | Ejecutable Linux |
| PE de Windows | `alyc -o output.exe input.aly` | Ejecutable Windows |
| Go | `alyc -o output.go input.aly` | Código fuente Go |
| Rust | `alyc -o output.rs input.aly` | Código fuente Rust |
| JavaScript | `alyc -o output.js input.aly` | Código fuente JavaScript |
| Python | `alyc -o output.py input.aly` | Código fuente Python |
| Bytecode JVM | `alyc -o output.class input.aly` | Bytecode Java |
| Kotlin | `alyc -o output.kt input.aly` | Código fuente Kotlin |
| C++ | `alyc -o output.cpp input.aly` | Código fuente C++ |
| Shell/Bash | `alyc -o output.sh input.aly` | Script de shell |

---

## 2. Backend LLVM

Usa LLVM para la generación de código nativo con optimizaciones:

```bash
alyc -o output input.aly --backend llvm
```

---

## 3. Backend C++

Transpila Aly a C++ con un header de runtime (`runtime_aly.h`):

```bash
alyc -o output.cpp input.aly
g++ -o output output.cpp runtime_aly.h
```

---

## 4. Backend JVM/Bytecode

Genera bytecode JVM directamente (archivos `.class`):

```bash
alyc -o MyApp.class input.aly
java MyApp
```

---

## 5. Detalles del Transpilador

Cada backend convierte el HIR (Representación Intermedia de Alto Nivel) de Aly al AST del lenguaje de destino y luego genera código fuente. El módulo `cpp_abi` maneja la compatibilidad con la ABI de C++ para las convenciones de llamada.