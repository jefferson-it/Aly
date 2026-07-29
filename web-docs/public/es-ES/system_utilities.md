# Utilidades del Sistema en Aly

Aly proporciona utilidades de nivel de sistema para notificaciones de escritorio, bandeja del sistema y acceso al portapapeles.

---

## 1. Notificaciones de Escritorio

```aly
import system.notification

system.notification.show("Aly", "Hello from Aly!", "path/to/icon.png")
system.notification.schedule("Reminder", "Meeting in 5 minutes", 300)  # 300 seconds
```

---

## 2. Bandeja del Sistema

```aly
import system.tray

let tray = system.tray.create("My App", "icon.png")
system.tray.add_menu(tray, "Show", fun() { print("Show clicked") })
system.tray.add_menu(tray, "Quit", fun() { system.tray.destroy(tray) })
system.tray.run(tray)
```

---

## 3. Portapapeles

```aly
import system.clipboard

system.clipboard.copy("Text to copy")
let text = system.clipboard.paste()
print(text)  # Outputs: Text to copy

system.clipboard.clear()
```