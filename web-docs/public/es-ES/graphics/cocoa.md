# Backend GUI Cocoa en Aly

El backend Cocoa mapea los widgets de Aly directamente a clases de AppKit (`NSWindow`, `NSButton`, etc.) en sistemas macOS.

---

## 1. Activación

Para forzar el renderizado nativo de Apple AppKit:

```aly
gui.useBackend("cocoa")
```

---

## 2. Cumplimiento de la Plataforma

Al utilizar los selectores de runtime estándar de AppKit, las ventanas de Cocoa cumplen automáticamente con las convenciones de diseño de macOS, las barras de menú de Apple y los renderizados de fuentes del sistema.