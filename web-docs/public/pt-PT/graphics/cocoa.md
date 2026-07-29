# Backend GUI Cocoa em Aly

O backend Cocoa mapeia widgets de Aly directamente para classes AppKit (`NSWindow`, `NSButton`, etc.) em sistemas macOS.

---

## 1. Activação

Para forçar a renderização nativa Apple AppKit:

```aly
gui.useBackend("cocoa")
```

---

## 2. Conformidade da Plataforma

Ao utilizar selectores de runtime AppKit padrão, as janelas Cocoa cumprem automaticamente as convenções de disposição macOS, barras de menu Apple e renderizações de fontes do sistema.
