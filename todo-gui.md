# TODO — Sistema de GUI do Aly

## Objetivos

Tornar a biblioteca gráfica do Aly multiplataforma, modular e independente do backend utilizado (FLTK, GTK4, Qt, DOM), permitindo que o mesmo código escrito em Aly funcione em qualquer backend sem modificações.

---

## Etapas

### ✅ Etapa 1 — Abstraction Layer (`src/gui/`)

Criar o módulo `src/gui/` com interfaces e tipos independentes de backend.

- [x] `src/gui/widget.rs` — `WidgetId`, gerador de IDs, constantes de props/eventos
- [x] `src/gui/backend.rs` — Trait `GuiBackend` (17 métodos) + `register_backend()` / `with_backend()` + `BackendType`
- [x] `src/gui/event.rs` — Sistema de callbacks (`set_callback`, `fire_callback`)
- [x] `src/gui/layout.rs` — `LayoutDirection`, `Alignment`
- [x] `src/gui/mod.rs` — Re-exports públicos
- [x] `src/lib.rs` — `pub mod gui;`
- [x] `doc/29-gui.md` — Documentação oficial da API
- [x] `__tests__/gui/test_abstraction.aly` — Exemplo de teste

**Funções nativas registradas no Aly:** `gui.useBackend`, `gui.window`, `gui.button`, `gui.label`, `gui.div`, `gui.input`, `gui.textarea`, `gui.password`, `gui.checkbox`, `gui.radio`, `gui.slider`, `gui.progressbar`, `gui.dropdown`, `gui.spinner`, `gui.container`, `gui.insert`, `gui.set`, `gui.get`, `gui.on`, `gui.run`

---

### ✅ Etapa 2 — Backend FLTK (`src/native/gui_fltk.rs`)

Implementar o trait `GuiBackend` usando FLTK como backend de renderização.

- [x] 15 tipos de widget (Window, Button, Label, Div, Input, TextArea, Password, Checkbox, Radio, Slider, ProgressBar, Dropdown, Spinner, Container, Separator)
- [x] Inserção na árvore de widgets (Window, Div, Radius, Container)
- [x] Propriedades (label, value, checked, enabled, visible)
- [x] Eventos (onClick, onChange, onMouseOver, onMouseOut, onFocus, onBlur, onKeyPress, onMouseWheel, onResize)
- [x] Loop principal FLTK

---

### ✅ Etapa 3 — Backend GTK4 (`src/native/gtk_backend.rs`)

Implementar o trait `GuiBackend` usando GTK4 como backend de renderização.

- [x] 14 tipos de widget (Window, Button, Label, Div, Input, TextArea, Password, Checkbox, Radio, Slider, ProgressBar, Dropdown, Tabs, Separator)
- [x] Inserção na árvore (Window, Div, Tabs)
- [x] Propriedades (label, value, checked, enabled, visible, fraction, class)
- [x] Eventos (onClick, onChange, onMouseOver, onMouseOut, onClose)
- [x] Loop principal GTK4
- [x] Seleção via `gui.useBackend("gtk")`

---

### ✅ Etapa 4 — Backend Qt (`src/native/qt_backend.rs`)

Implementar o trait `GuiBackend` usando Qt6 como backend de renderização.

- [x] Criar `src/native/qt_backend.rs`
- [x] Adicionar dependência Qt6 ao `Cargo.toml`
- [x] Implementar todos os métodos do `GuiBackend`
- [x] Registrar via `gui.useBackend("qt")`
- [x] Garantir paridade de widgets com FLTK/GTK4
- [x] Testar compatibilidade

---

### ✅ Etapa 5 — Backend DOM (`src/native/dom_backend.rs`)

Implementar o trait `GuiBackend` usando HTML/DOM como backend de renderização.

- [x] Criar `src/native/dom_backend.rs`
- [x] Virtual DOM tree — `DomNode` struct com id, tag, atributos, children, texto, eventos
- [x] Gerador de HTML estático — `generate_html()` e `generate_html_fragment()`
- [x] 15 tipos de widget mapeados para elementos HTML (div, button, input, select, progress, etc.)
- [x] Manipulação de árvore (inserção, propriedades)
- [x] Eventos DOM (onClick, onChange com atributos onclick/onchange)
- [x] Registro via `gui.useBackend("dom")`
- [x] Exemplo: `__tests__/gui/test_dom.aly`

**Modos de execução:**
- [x] Navegador — servir HTML gerado via HTTP
- [ ] Desktop — WebView/Electron como mecanismo de renderização
- [ ] Embedded — runtime DOM embarcado sem navegador externo

**Integração:**
- [ ] HTML com `<script type="text/aly">`
- [ ] Interoperabilidade com JavaScript
- [ ] Runtime JS embarcado (V8 / Rust V8)

---

### 🔲 Etapa 6 — Revisão e Melhorias

- [ ] Revisar API do FLTK para suportar todos os widgets do roadmap
- [ ] CSS/Stylesheet integrado à abstração (atualmente só FLTK tem)
- [ ] Sistema de layouts avançado (Grid, Stack, etc.)
- [ ] Temas e estilos unificados entre backends
- [ ] Testes automatizados para cada backend
- [ ] Benchmark de desempenho entre backends
- [ ] Suporte a WebView/Electron como backend desktop

---

## Como usar

```aly
// Seleciona backend (FLTK é padrão)
gui.useBackend("fltk")  // ou "gtk", "qt", "dom"

// Cria widgets
let win = gui.window("App", 800, 600)
let btn = gui.button("OK")

// Conecta eventos
gui.on(btn, "onClick", fn() {
    console.log("Clicou!")
})

// Inicia o loop
gui.run()
```

## Estrutura do projeto

```
src/
  gui/
    mod.rs          — Módulo principal, re-exports
    backend.rs      — Trait GuiBackend + registry
    widget.rs       — WidgetId, constantes
    event.rs        — Sistema de callbacks
    layout.rs       — LayoutDirection, Alignment
  native/
    gui_fltk.rs          — Implementação FLTK do GuiBackend
    gtk_backend.rs       — Implementação GTK4 do GuiBackend
    gui_abstraction.rs   — Funções nativas expostas ao Aly
doc/
    29-gui.md       — Documentação da API
todo-gui.md         — Este arquivo
```
