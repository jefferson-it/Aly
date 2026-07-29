# Interface Gráfica do Usuário (GUI) no Aly

O Aly possui um toolkit GUI abstrato nativo que compila e executa em múltiplos backends de plataforma.

---

## 1. Abstração de Interface Unificada

Em vez de escrever código de UI específico para cada plataforma, desenvolvedores Aly constroem layouts usando classes GUI OOP padrão (`Window`, `Button`, `Label`, `Input`). Internamente, esses layouts são mapeados para widgets gerados pelo backend da plataforma ativa.

```aly
const win = new Window("Title", 400, 300)
const btn = new Button("Press")
win.insert(btn)
win.loop()
```

---

## 2. Backends Suportados

* **FLTK**: O backend padrão. Altamente leve e portátil.
* **GTK4**: Experiência desktop moderna focada em Linux.
* **Cocoa**: Componentes de UI nativos do macOS.
* **Qt**: Espaço reservado para backend customizado.
* **DOM**: Representação virtual para compilação web.
