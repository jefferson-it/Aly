# GTK4 GUI Backend in Aly

The GTK4 backend connects Aly visual widgets to native GNOME desktop widgets on Linux systems.

---

## 1. Activation

To force GTK4 layout rendering:

```aly
gui.useBackend("gtk")
```

---

## 2. Linux Styling Integration

GTK4 windows automatically respect active system GTK themes, dark modes, and CSS stylesheets loaded via:

```aly
win.importStyle("theme.css")
```
