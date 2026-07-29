# Ligações de Linguagem em Aly

Aly fornece interoperabilidade com outras linguagens de programação através de módulos de ligação.

---

## 1. Ligações Java / JNI

```aly
import bindings.java

let jvm = bindings.java.create_vm()
let obj = bindings.java.call_static("java.lang.Math", "max", [10, 20])
print(obj)  # Resultado: 20
```

O gerador JNI (`jni_gen` tool) gera automaticamente código de ligação a partir de schemas Aly.

---

## 2. Ligações .NET

```aly
import bindings.dotnet

let result = bindings.dotnet.call("System.Math", "Sin", [3.14159])
print(result)
```

---

## 3. Ligações Node.js

```aly
import bindings.nodejs

let npm = bindings.nodejs.require("lodash")
let result = bindings.nodejs.call(npm, "add", [1, 2, 3])
print(result)  # Resultado: 6
```

---

## 4. Ligações Rust

Funções nativas Rust são registadas como módulos nativos Aly através do sistema de tipos `ValueData`. Consulte [Ligações Nativas](native.md) para mais detalhes.
