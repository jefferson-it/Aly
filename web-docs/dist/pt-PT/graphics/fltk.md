# Backend GUI FLTK em Aly

FLTK (Fast Light Toolkit) é o backend GUI transversal predefinido em Aly devido à sua velocidade e baixo consumo de recursos.

---

## 1. Activação

Não é necessária configuração adicional para usar FLTK, mas pode especificá-lo explicitamente:

```aly
gui.useBackend("fltk")
```

---

## 2. Funcionalidades

* **Leve**: Compila-se staticamente no binário de saída sem necessitar de bibliotecas de sistema dinamicamente ligadas pesadas.
* **Velocidade**: Inicialização de janela instantânea e atraso mínimo de renderização.
* **Portabilidade**: Opera consistentemente em Linux, Windows e macOS.
