# Interface Gráfica de Utilizador (GUI) em Aly

Aly disponibiliza um toolkit GUI nativo abstracto que compila e executa em múltiplos backends de plataforma.

---

## 1. Abstração de Interface Unificada

Em vez de escrever código UI específico da plataforma, os programadores Aly constroem disposições utilizando classes GUI OOP padrão (`Window`, `Button`, `Label`, `Input`). Por baixo dos panos, estas disposições mapeiam para widgets gerados pelo backend de plataforma activo.

```aly
const win = new Window("Title", 400, 300)
const btn = new Button("Press")
win.insert(btn)
win.loop()
```

---

## 2. Backends Suportados

* **FLTK**: O backend predefinido. Muito leve e portátil.
* **GTK4**: Experiência de ambiente de trabalho moderna focada em Linux.
* **Cocoa**: Componentes UI nativos macOS.
* **Qt**: Placeholder de backend personalizado.
* **DOM**: Representação virtual para compilação web.
