# Utilitários de Sistema em Aly

Aly fornece utilitários de nível de sistema para notificações de ambiente de trabalho, bandeja do sistema e acesso à área de transferência.

---

## 1. Notificações de Ambiente de Trabalho

```aly
import system.notification

system.notification.show("Aly", "Hello from Aly!", "path/to/icon.png")
system.notification.schedule("Reminder", "Meeting in 5 minutes", 300)  # 300 segundos
```

---

## 2. Bandeja do Sistema

```aly
import system.tray

let tray = system.tray.create("My App", "icon.png")
system.tray.add_menu(tray, "Show", fun() { print("Show clicked") })
system.tray.add_menu(tray, "Quit", fun() { system.tray.destroy(tray) })
system.tray.run(tray)
```

---

## 3. Área de Transferência

```aly
import system.clipboard

system.clipboard.copy("Text to copy")
let text = system.clipboard.paste()
print(text)  # Resultado: Text to copy

system.clipboard.clear()
```
