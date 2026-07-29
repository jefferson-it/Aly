# System Utilities in Aly

Aly provides system-level utilities for desktop notifications, system tray, and clipboard access.

---

## 1. Desktop Notifications

```aly
import system.notification

system.notification.show("Aly", "Hello from Aly!", "path/to/icon.png")
system.notification.schedule("Reminder", "Meeting in 5 minutes", 300)  # 300 seconds
```

---

## 2. System Tray

```aly
import system.tray

let tray = system.tray.create("My App", "icon.png")
system.tray.add_menu(tray, "Show", fun() { print("Show clicked") })
system.tray.add_menu(tray, "Quit", fun() { system.tray.destroy(tray) })
system.tray.run(tray)
```

---

## 3. Clipboard

```aly
import system.clipboard

system.clipboard.copy("Text to copy")
let text = system.clipboard.paste()
print(text)  # Outputs: Text to copy

system.clipboard.clear()
```