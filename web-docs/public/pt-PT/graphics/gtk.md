# Backend GUI GTK4 em Aly

O backend GTK4 liga widgets visuais de Aly a widgets nativos de ambiente de trabalho GNOME em sistemas Linux.

---

## 1. Activação

Para forçar a renderização de disposição GTK4:

```aly
gui.useBackend("gtk")
```

---

## 2. Integração de Estilos Linux

As janelas GTK4 respeitam automaticamente os temas GTK do sistema activo, modos escuros e folhas de estilo CSS carregadas via:

```aly
win.importStyle("theme.css")
```
