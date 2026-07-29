# Backend GUI GTK4 no Aly

O backend GTK4 conecta os widgets visuais do Aly aos widgets nativos do GNOME Desktop em sistemas Linux.

---

## 1. Ativação

Para forçar a renderização de layout GTK4:

```aly
gui.useBackend("gtk")
```

---

## 2. Integração com Estilos Linux

Janelas GTK4 automaticamente respeitam temas GTK ativos do sistema, modos escuros e folhas de estilo CSS carregadas via:

```aly
win.importStyle("theme.css")
```
