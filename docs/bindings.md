# Language Bindings in Aly

Aly provides interoperability with other programming languages through binding modules.

---

## 1. Java / JNI Bindings

```aly
import bindings.java

let jvm = bindings.java.create_vm()
let obj = bindings.java.call_static("java.lang.Math", "max", [10, 20])
print(obj)  # Outputs: 20
```

The JNI generator (`jni_gen` tool) automatically generates binding code from Aly schemas.

---

## 2. .NET Bindings

```aly
import bindings.dotnet

let result = bindings.dotnet.call("System.Math", "Sin", [3.14159])
print(result)
```

---

## 3. Node.js Bindings

```aly
import bindings.nodejs

let npm = bindings.nodejs.require("lodash")
let result = bindings.nodejs.call(npm, "add", [1, 2, 3])
print(result)  # Outputs: 6
```

---

## 4. Rust Bindings

Native Rust functions are registered as Aly native modules through the `ValueData` type system. See [Native Bindings](native.md) for details.