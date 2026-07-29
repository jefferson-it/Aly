# Módulo de Plataforma Web em Aly

Aly fornece capacidades de tempo de execução web, incluindo manipulação DOM, estilização CSS, Componentes Web e backend WebAssembly.

---

## 1. Web / DOM

```aly
import web

let doc = web.dom.create()
let div = web.dom.createElement("div")
div.setAttribute("class", "container")
div.setTextContent("Hello Aly")
web.dom.appendChild(document.body, div)
```

---

## 2. Estilização CSS

```aly
import css

let style = css.parse("body { color: red; font-size: 16px; }")
css.inject(style)

let btnStyle = css.create(".btn", {
    "background-color": "blue",
    "color": "white",
    "padding": "10px"
})
```

---

## 3. Componentes Web

```aly
import web_components

web_components.define("my-component", {
    template: "<div>Olá do componente</div>",
    style: ":host { display: block; }",
    fun connected() {
        print("Componente montado")
    }
})
```

---

## 4. Backend DOM

```aly
import dom_backend

let backend = dom_backend.create()
backend.render("<app></app>")
backend.mount("#root")
```

O backend DOM permite renderização no servidor e diferenciação de DOM virtual.

---

## 5. Backend WebAssembly

```aly
import wasm_backend

let wasm = wasm_backend.compile("module.wasm")
let result = wasm_backend.invoke(wasm, "add", [1, 2])
print(result)  # Resultado: 3
```

O backend WASM permite executar módulos WebAssembly directamente a partir de Aly.
