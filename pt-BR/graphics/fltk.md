# Backend GUI FLTK no Aly

FLTK (Fast Light Toolkit) é o backend GUI multiplataforma padrão no Aly devido à sua velocidade e baixo consumo de recursos.

---

## 1. Ativação

Nenhuma configuração adicional é necessária para usar o FLTK, mas você pode especificá-lo explicitamente:

```aly
gui.useBackend("fltk")
```

---

## 2. Recursos

* **Leve**: Compila estaticamente no binário de saída sem exigir bibliotecas de sistema dinâmicas pesadas.
* **Velocidade**: Inicialização instantânea de janelas e atraso mínimo de renderização.
* **Portabilidade**: Opera de forma consistente em Linux, Windows e macOS.
