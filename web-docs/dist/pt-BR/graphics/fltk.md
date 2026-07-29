# FLTK GUI Backend in Aly

FLTK (Fast Light Toolkit) is the default cross-platform GUI backend in Aly due to its speed and low footprint.

---

## 1. Activation

No additional configuration is required to use FLTK, but you can explicitly specify it:

```aly
gui.useBackend("fltk")
```

---

## 2. Features

* **Lightweight**: Compiles statically into the output binary without requiring heavy dynamically linked system libraries.
* **Speed**: Instantaneous window initialization and minimal rendering lag.
* **Portability**: Operates consistently across Linux, Windows, and macOS.
