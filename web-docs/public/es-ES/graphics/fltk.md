# Backend de GUI FLTK en Aly

FLTK (Fast Light Toolkit) es el backend de GUI multiplataforma predeterminado en Aly debido a su velocidad y bajo consumo de recursos.

---

## 1. Activación

No se requiere ninguna configuración adicional para usar FLTK, pero puede especificarlo explícitamente:

```aly
gui.useBackend("fltk")
```

---

## 2. Características

* **Ligero**: Se compila estáticamente en el binario de salida sin requerir bibliotecas del sistema dinámicas pesadas.
* **Velocidad**: Inicialización de ventana instantánea y renderizado mínimo con baja latencia.
* **Portabilidad**: Funciona consistentemente en Linux, Windows y macOS.