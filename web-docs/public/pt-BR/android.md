# Suporte Android no Aly

O Aly pode compilar e empacotar aplicações para dispositivos Android.

---

## 1. Ferramenta de Build Android

```bash
aly android-build --package com.example.myapp --name "My App" input.aly
```

Isso gera uma estrutura de projeto Android com:
- Arquivos de build do Gradle
- AndroidManifest.xml
- Ponte JNI para o runtime do Aly
- Empacotamento APK

---

## 2. Runtime do Android

```aly
import android

android.toast("Hello from Aly!")
let battery = android.get_battery_level()
print("Battery: %")

android.vibrate(500)  # Vibrate for 500ms
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