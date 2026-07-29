# Backend GUI FLTK en Aly

FLTK (Fast Light Toolkit) es el backend GUI multiplataforma predeterminado en Aly debido a su velocidad y baja huella.

---

## 1. Activación

No se requiere configuración adicional para utilizar FLTK, pero usted puede especificarlo explícitamente:

```aly
gui.useBackend("fltk")
```

---

## 2. Características

* **Ligero**: Se compila estáticamente en el binario de salida sin requerir pesadas librerías del sistema enlazadas dinámicamente.
* **Velocidad**: Inicialización instantánea de ventanas y retraso de renderizado mínimo.
* **Portabilidad**: Opera consistentemente a través de Linux, Windows y macOS.