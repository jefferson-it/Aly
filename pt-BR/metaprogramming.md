# Metaprogramação no Aly

O Aly suporta geração e transformação de código em tempo de compilação através de macros e avaliação preguiçosa (lazy evaluation).

---

## 1. Definições de Macro

```aly
macrodef assert(condition) {
    if not (condition) {
        print("Assertion failed")
    }
}

let x = 10
assert(x gt 5)  # Expands at compile time
```

As macros recebem árvores de tokens e produzem nós AST que são inseridos no código de chamada.

---

## 2. Avaliação Preguiçosa (Lazy)

```aly
lazy let expensive = compute_large_value()
# The expression is not evaluated until first access

print(expensive)  # Evaluated here
```

As variáveis lazy são inicializadas apenas no seu primeiro acesso, útil para computação adiada e referências circulares.

---

## 3. Avaliação de Constante em Tempo de Compilação

O módulo `const_eval` avalia expressões em tempo de compilação quando todas as entradas são constantes conhecidas:

```aly
const SIZE = 100
const AREA = SIZE * SIZE  # Evaluated at compile time
```

---

## 4. Ganchos AST (Hooks)

O módulo `metaprogramming` do compilador fornece hooks para transformar a AST durante a compilação, permitindo extensões de sintaxe personalizadas e incorporação de DSL.
