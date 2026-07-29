# Web Platform Module in Aly

Aly provides web runtime capabilities including DOM manipulation, CSS styling, Web Components, and WebAssembly backend.

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

## 2. CSS Styling

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

## 4. DOM Backend

```aly
import dom_backend

let backend = dom_backend.create()
backend.render("<app></app>")
backend.mount("#root")
```

The DOM backend enables server-side rendering and virtual DOM diffing.

---

## 5. WebAssembly Backend

```aly
import wasm_backend

let wasm = wasm_backend.compile("module.wasm")
let result = wasm_backend.invoke(wasm, "add", [1, 2])
print(result)  # Outputs: 3
```

The WASM backend allows running WebAssembly modules directly from Aly.