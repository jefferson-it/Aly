# Soporte para Android en Aly

Aly puede compilar y empaquetar aplicaciones para dispositivos Android.

---

## 1. Herramienta de Compilación para Android

```bash
aly android-build --package com.example.myapp --name "My App" input.aly
```

Esto genera una estructura de proyecto Android con:
- Archivos de compilación Gradle
- AndroidManifest.xml
- Puente JNI para el runtime de Aly
- Empaquetado de APK

---

## 2. Runtime de Android

```aly
import android

android.toast("Hello from Aly!")
let battery = android.get_battery_level()
print("Battery: $battery%")

android.vibrate(500)  # Vibrar durante 500ms
```

---

## 3. Estructura del Proyecto

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