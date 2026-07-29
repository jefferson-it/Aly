# Interfaz Gráfica de Usuario (GUI) en Aly

Aly cuenta con un toolkit de GUI nativo abstracto que se compila y ejecuta en múltiples backends de plataforma.

---

## 1. Abstracción de Interfaz Unificada

En lugar de escribir código de UI específico de la plataforma, los desarrolladores de Aly construyen diseños usando clases de GUI OOP estándar (`Window`, `Button`, `Label`, `Input`). Debajo del capó, estos diseños se mapan a widgets generados por el backend de plataforma activo.

```aly
const win = new Window("Title", 400, 300)
const btn = new Button("Press")
win.insert(btn)
win.loop()
```

---

## 2. Backends Compatibles

* **FLTK**: El backend predeterminado. Muy ligero y portable.
* **GTK4**: Experiencia de escritorio moderna enfocada en Linux.
* **Cocoa**: Componentes de UI nativos de macOS.
* **Qt**: Marcador de posición de backend personalizado.
* **DOM**: Representación virtual para compilación web.