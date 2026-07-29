# Backend GUI GTK4 en Aly

El backend GTK4 conecta los widgets visuales de Aly con los widgets nativos de escritorio GNOME en sistemas Linux.

---

## 1. Activación

Para forzar el renderizado del diseño GTK4:

```aly
gui.useBackend("gtk")
```

---

## 2. Integración de Estilos de Linux

Las ventanas de GTK4 respetan automáticamente los temas activos del sistema GTK, los modos oscuros y las hojas de estilo CSS cargadas mediante:

```aly
win.importStyle("theme.css")
```