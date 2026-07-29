# Backend GUI GTK4 no Aly

O backend GTK4 liga os widgets visuais do Aly aos widgets nativos do ambiente de trabalho GNOME nos sistemas Linux.

---

## 1. Ativação

Para forçar a renderização de layout GTK4:

```aly
gui.useBackend("gtk")
```

---

## 2. Integração de Estilos Linux

As janelas GTK4 respeitam automaticamente os temas GTK ativos do sistema, modos escuros e folhas de estilo CSS carregadas através de:

```aly
win.importStyle("theme.css")
```
