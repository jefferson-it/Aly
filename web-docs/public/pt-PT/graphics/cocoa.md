# Backend GUI Cocoa no Aly

O backend Cocoa mapeia os widgets visuais do Aly diretamente para as classes do AppKit (`NSWindow`, `NSButton`, etc.) nos sistemas macOS.

---

## 1. Ativação

Para forçar a renderização nativa do Apple AppKit:

```aly
gui.useBackend("cocoa")
```

---

## 2. Conformidade com a Plataforma

Ao utilizar os seletores padrão de tempo de execução do AppKit, as janelas Cocoa cumprem automaticamente com as convenções de layout do macOS, as barras de Menu da Apple e as renderizações de tipo de letra do sistema.
