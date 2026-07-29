# Bindings de Linguagens no Aly

O Aly fornece interoperabilidade com outras linguagens de programação através de módulos de bindings.

---

## 1. Bindings Java / JNI

```aly
import bindings.java

let jvm = bindings.java.create_vm()
let obj = bindings.java.call_static("java.lang.Math", "max", [10, 20])
print(obj)  # Outputs: 20
```

O gerador JNI (ferramenta `jni_gen`) gera automaticamente código de binding a partir de schemas do Aly.

---

## 2. Bindings .NET

```aly
import bindings.dotnet

let result = bindings.dotnet.call("System.Math", "Sin", [3.14159])
print(result)
```

---

## 3. Bindings Node.js

```aly
import bindings.nodejs

let npm = bindings.nodejs.require("lodash")
let result = bindings.nodejs.call(npm, "add", [1, 2, 3])
print(result)  # Outputs: 6
```

---

## 4. Bindings Rust

Funções nativas em Rust são registradas como módulos nativos do Aly através do sistema de tipos `ValueData`. Veja [Bindings Nativos](native.md) para detalhes.