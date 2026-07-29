# Módulo de Plataforma Web en Aly

Aly proporciona capacidades de runtime web que incluyen manipulación de DOM, estilos CSS, Web Components y backend WebAssembly.

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

## 2. Estilos CSS

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

## 3. Web Components

```aly
import web_components

web_components.define("my-component", {
    template: "<div>Hello from component</div>",
    style: ":host { display: block; }",
    fun connected() {
        print("Component mounted")
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

El backend DOM permite el renderizado en el servidor y la comparación diferencial del DOM virtual.

---

## 5. Backend WebAssembly

```aly
import wasm_backend

let wasm = wasm_backend.compile("module.wasm")
let result = wasm_backend.invoke(wasm, "add", [1, 2])
print(result)  # Outputs: 3
```

El backend WASM permite ejecutar módulos WebAssembly directamente desde Aly.