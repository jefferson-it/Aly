# Suporte Android em Aly

Aly pode compilar e empacotar aplicações para dispositivos Android.

---

## 1. Ferramenta de Compilação Android

```bash
aly android-build --package com.example.myapp --name "My App" input.aly
```

Isto gera uma estrutura de projeto Android com:
- Ficheiros de compilação Gradle
- AndroidManifest.xml
- Ponte JNI para o tempo de execução Aly
- Empacotamento APK

---

## 2. Tempo de Execução Android

```aly
import android

android.toast("Hello from Aly!")
let battery = android.get_battery_level()
print("Bateria: %")

android.vibrate(500)  # Vibrar durante 500ms
```

---

## 3. Estrutura do Projeto

```
myapp/
├── build.gradle
├── src/
│   └── main/
│       ├── AndroidManifest.xml
│       ├── java/com/example/myapp/MainActivity.java
│       └── jniLibs/
│           ├── arm64-v8a/libaly.so
│           └── armeabi-v7a/libaly.so
└── app.aly
```
