# Metaprogramação em Aly

Aly suporta geração e transformação de código em tempo de compilação através de macros e avaliação preguiçosa.

---

## 1. Definições de Macros

```aly
macrodef assert(condition) {
    if not (condition) {
        print("Assertion failed")
    }
}

let x = 10
assert(x gt 5)  # Expandido em tempo de compilação
```

Macros recebem árvores de tokens e produzem nós AST que são spliced no código chamador.

---

## 2. Avaliação Preguiçosa

```aly
lazy let expensive = compute_large_value()
# A expressão não é avaliada até ao primeiro acesso

print(expensive)  # Avaliado aqui
```

Variáveis preguiçosas são inicializadas apenas no seu primeiro acesso, úteis para computação diferida e referências circulares.

---

## 3. Avaliação de Constantes em Tempo de Compilação

O módulo `const_eval` avalia expressões em tempo de compilação quando todas as entradas são constantes conhecidas:

```aly
const SIZE = 100
const AREA = SIZE * SIZE  # Avaliado em tempo de compilação
```

---

## 4. Ganchos AST

O módulo `metaprogramming` do compilador fornece ganchos para transformar o AST durante a compilação, permitindo extensões de sintaxe personalizadas e incorporação de DSL.
