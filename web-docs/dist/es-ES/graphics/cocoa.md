# Backend de GUI Cocoa en Aly

El backend Cocoa asigna los widgets de Aly directamente a clases AppKit (`NSWindow`, `NSButton`, etc.) en sistemas macOS.

---

## 1. Activación

Para forzar el renderizado nativo de Apple AppKit:

```aly
gui.useBackend("cocoa")
```

---

## 2. Cumplimiento de Plataforma

Al utilizar selectores de tiempo de ejecución estándar de AppKit, las ventanas Cocoa cumplen automáticamente con las convenciones de diseño de macOS, las barras de menú de Apple y los renderizados de fuentes del sistema.