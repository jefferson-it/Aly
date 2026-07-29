# Android Support in Aly

Aly can build and package applications for Android devices.

---

## 1. Android Build Tool

```bash
aly android-build --package com.example.myapp --name "My App" input.aly
```

This generates an Android project structure with:
- Gradle build files
- AndroidManifest.xml
- JNI bridge for Aly runtime
- APK packaging

---

## 2. Android Runtime

```aly
import android

android.toast("Hello from Aly!")
let battery = android.get_battery_level()
print("Battery: $battery%")

android.vibrate(500)  # Vibrate for 500ms
```

---

## 3. Project Structure

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