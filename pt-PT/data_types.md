# Tipos de Dados em Aly

Aly é tipada dinamicamente, mas suporta internamente um conjunto rico de tipos primitivos e compostos robustos.

---

## 1. Tipos Primitivos

* **Inteiro (`int`)**: Inteiros assinados de 64 bits (por exemplo, `42`, `-10`).
* **Ponto Flutuante (`float`)**: Números de ponto flutuante IEEE 754 de 64 bits (por exemplo, `3.14159`, `-0.001`).
* **Booleano (`bool`)**: Valores lógicos, `true` ou `false`.
* **Carácter (`char`)**: Literais de caracteres de um byte entre aspas simples (por exemplo, `'A'`, `'\n'`, `'\t'`).
* **Cadeia de Caracteres (`string`)**: Sequência de caracteres entre aspas duplas (por exemplo, `"Hello, World"`). Suporta interpolação de modelos de cadeia usando a sintaxe ``.
* **Nenhum (`null`)**: Um tipo especial que representa a ausência de valor, acedido através da palavra-chave `None`.
* **Vazio**: Indica uma função ou expressão que não devolve valor.

---

## 2. Lista / Vetor (`list`)

Vetores são arrays dinâmicos ordenados que podem conter elementos de qualquer tipo (incluindo vetores ou objectos aninhados).

```aly
let numbers = [1, 2, 3, 4]
let mixed = ["Aly", 10.5, true, None]

# Acesso por índice
print(numbers[0]) # Resultado: 1
```

---

## 3. Objectos / Mapas (`object`)

Objectos são mapeamentos de chave-valor que representam estruturas de dicionário ou array associativo. As chaves são identificadores ou cadeias de caracteres.

```aly
let user = {
    name: "Jefferson",
    role: "Developer",
    active: true
}

# Acesso a membros
print(user.name) # Resultado: Jefferson
print(user["role"]) # Resultado: Developer
```
