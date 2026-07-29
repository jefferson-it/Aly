# Enlaces de Lenguaje en Aly

Aly proporciona interoperabilidad con otros lenguajes de programación a través de módulos de enlace.

---

## 1. Enlaces Java / JNI

```aly
import bindings.java

let jvm = bindings.java.create_vm()
let obj = bindings.java.call_static("java.lang.Math", "max", [10, 20])
print(obj)  # Outputs: 20
```

El generador JNI (`jni_gen` tool) genera automáticamente código de enlace a partir de esquemas de Aly.

---

## 2. Enlaces .NET

```aly
import bindings.dotnet

let result = bindings.dotnet.call("System.Math", "Sin", [3.14159])
print(result)
```

---

## 3. Enlaces Node.js

```aly
import bindings.nodejs

let npm = bindings.nodejs.require("lodash")
let result = bindings.nodejs.call(npm, "add", [1, 2, 3])
print(result)  # Outputs: 6
```

---

## 4. Enlaces Rust

Las funciones nativas de Rust se registran como módulos nativos de Aly a través del sistema de tipos `ValueData`. Consulte [Enlaces Nativos](native.md) para más detalles.