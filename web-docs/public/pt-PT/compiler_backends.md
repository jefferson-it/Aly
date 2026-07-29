# Backends do Compilador no Aly

O compilador AOT do Aly (`alyc`) pode transpilar código para múltiplas linguagens e plataformas de destino.

---

## 1. Backends Disponíveis

| Backend | Comando | Saída |
|---------|---------|-------|
| Native ELF | `alyc -o output input.aly` | Executável Linux |
| Windows PE | `alyc -o output.exe input.aly` | Executável Windows |
| Go | `alyc -o output.go input.aly` | Código-fonte Go |
| Rust | `alyc -o output.rs input.aly` | Código-fonte Rust |
| JavaScript | `alyc -o output.js input.aly` | Código-fonte JavaScript |
| Python | `alyc -o output.py input.aly` | Código-fonte Python |
| JVM Bytecode | `alyc -o output.class input.aly` | Bytecode Java |
| Kotlin | `alyc -o output.kt input.aly` | Código-fonte Kotlin |
| C++ | `alyc -o output.cpp input.aly` | Código-fonte C++ |
| Shell/Bash | `alyc -o output.sh input.aly` | Script Shell |

---

## 2. Backend LLVM

Utiliza LLVM para geração de código nativo com otimizações:

```bash
alyc -o output input.aly --backend llvm
```

---

## 3. Backend C++

Transpila Aly para C++ com um cabeçalho de runtime (`runtime_aly.h`):

```bash
alyc -o output.cpp input.aly
g++ -o output output.cpp runtime_aly.h
```

---

## 4. Backend JVM/Bytecode

Gera bytecode JVM diretamente (ficheiros `.class`):

```bash
alyc -o MyApp.class input.aly
java MyApp
```

---

## 5. Detalhes do Transpilador

Cada backend converte a HIR (High-level Intermediate Representation) do Aly para a AST da linguagem de destino, gerando em seguida o código-fonte. O módulo `cpp_abi` lida com a compatibilidade ABI do C++ para convenções de chamada.
