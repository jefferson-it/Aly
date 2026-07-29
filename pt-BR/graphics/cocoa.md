# Backend GUI Cocoa no Aly

O backend Cocoa mapeia os widgets do Aly diretamente para classes AppKit (`NSWindow`, `NSButton`, etc.) em sistemas macOS.

---

## 1. Ativação

Para forçar a renderização nativa Apple AppKit:

```aly
gui.useBackend("cocoa")
```

---

## 2. Conformidade com a Plataforma

Ao utilizar seletores padrão do runtime AppKit, as janelas Cocoa automaticamente obedecem às convenções de layout do macOS, barras de menu Apple e renderizações de fonte do sistema.
