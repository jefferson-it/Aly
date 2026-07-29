# GUI — Sistema de Interface Gráfica

O módulo `gui` fornece uma API unificada e independente de backend para criação de interfaces gráficas no Aly. O mesmo código funciona com FLTK, GTK4, Qt ou DOM, bastando selecionar o backend desejado.

---

## Arquitetura

```
src/gui/
  mod.rs       - Módulo principal, re-exports
  backend.rs   - Trait GuiBackend + registro de backends
  widget.rs    - WidgetId, geração de IDs, constantes de propriedades
  event.rs     - Sistema de callbacks para eventos
  layout.rs    - LayoutDirection, Alignment
```

A camada de abstração define um trait `GuiBackend` que cada backend (FLTK, GTK4, Qt, DOM) implementa. As funções nativas expostas ao Aly (`gui.window`, `gui.button`, etc.) delegam ao backend registrado.

### Backends

| Backend | Módulo Rust | Status |
|---------|-------------|--------|
| FLTK | `src/native/gui_fltk.rs` | ✅ Implementado (padrão) |
| GTK4 | `src/native/gtk_backend.rs` | ✅ Implementado |
| Qt | — | ❌ Não implementado |
| DOM | — | ❌ Não implementado |

---

## Uso

### Selecionar backend

```aly
gui.useBackend("fltk")  // FLTK é o padrão
```

### Criar widgets

```aly
let win   = gui.window("Título", 800, 600)
let btn   = gui.button("Clique")
let lbl   = gui.label("Texto")
let div   = gui.div("vertical")     // "vertical" ou "horizontal"
let inp   = gui.input("Placeholder")
let ta    = gui.textarea()
let pwd   = gui.password()
let cb    = gui.checkbox("Opção")
let radio = gui.radio("Escolha A")
let slider = gui.slider(0, 100, 50)
let pb    = gui.progressbar(0.5)
let dd    = gui.dropdown("item1,item2,item3")
let spin  = gui.spinner(0, 100, 1, 50)
let container = gui.container(400, 300, "vertical")
```

### Árvore de widgets

```aly
gui.insert(parent, child)
```

### Propriedades

```aly
gui.set(widget, "label", "Novo texto")
gui.set(widget, "value", "42")
gui.set(widget, "checked", "true")

let texto = gui.get(widget, "label")
```

### Eventos

```aly
gui.on(btn, "onClick", meuHandler)
gui.on(inp, "onChange", meuHandler)
gui.on(btn, "onMouseOver", meuHandler)
gui.on(btn, "onMouseOut", meuHandler)
gui.on(inp, "onFocus", meuHandler)
gui.on(inp, "onBlur", meuHandler)
gui.on(btn, "onKeyPress", meuHandler)
```

### Loop principal

```aly
gui.run()
```

---

## Exemplo completo

```aly
let win = gui.window("Exemplo", 400, 300)
let div = gui.div("vertical")
let lbl = gui.label("Olá, mundo!")
let btn = gui.button("OK")

gui.insert(win, div)
gui.insert(div, lbl)
gui.insert(div, btn)

gui.set(lbl, "label", "Clique no botão")

gui.on(btn, "onClick", fn() {
    console.log("Clicou!")
})

gui.run()
```

---

## Propriedades suportadas

| Propriedade | Tipo | Descrição |
|-------------|------|-----------|
| `label` | string | Texto do widget |
| `value` | string | Valor texto (inputs) |
| `innerText` | string | Alias para `value` |
| `checked` | bool | Checkbox/Radio marcado |
| `enabled` | bool | Widget habilitado |
| `visible` | bool | Widget visível |

---

## Eventos suportados

| Evento | Descrição |
|--------|-----------|
| `onClick` | Clique do mouse |
| `onChange` | Valor alterado |
| `onMouseOver` | Mouse entrou no widget |
| `onMouseOut` | Mouse saiu do widget |
| `onFocus` | Widget recebeu foco |
| `onBlur` | Widget perdeu foco |
| `onKeyPress` | Tecla pressionada |

---

## Adicionar um novo backend

Para implementar um novo backend, crie um struct que implemente o trait `GuiBackend`:

```rust
use crate::gui::{GuiBackend, BackendType, register_backend};

pub struct MeuBackend;

impl GuiBackend for MeuBackend {
    fn backend_type(&self) -> BackendType { BackendType::Fltk }

    fn create_window(&mut self, title: &str, w: i32, h: i32) -> String { /* ... */ }
    fn create_button(&mut self, label: &str) -> String { /* ... */ }
    // ... demais métodos
    fn run(&mut self) { /* ... */ }
}

// Registrar
register_backend(Box::new(MeuBackend));
```

O backend deve ser registrado antes de chamar qualquer função `gui.*` do Aly.
