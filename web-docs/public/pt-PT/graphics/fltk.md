# Backend GUI FLTK no Aly

O FLTK (Fast Light Toolkit) é o backend GUI multiplataforma padrão no Aly devido à sua velocidade e baixo consumo de recursos.

---

## 1. Ativação

Nenhuma configuração adicional é necessária para utilizar o FLTK, mas pode especificá-lo explicitamente:

```aly
gui.useBackend("fltk")
```

---

## 2. Funcionalidades

* **Leve**: Compila estaticamente no binário de saída sem necessitar de bibliotecas de sistema pesadas ligadas dinamicamente.
* **Velocidade**: Inicialização de janelas instantânea e atraso de renderização mínimo.
* **Portabilidade**: Funciona de forma consistente em Linux, Windows e macOS.
