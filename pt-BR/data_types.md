# Tipos de Dados no Aly

O Aly é dinamicamente tipado, mas suporta internamente um rico conjunto de tipos de dados primitivos e compostos robustos.

---

## 1. Primitivos

* **Integer (`int`)**: Inteiros com sinal de 64 bits (ex. `42`, `-10`).
* **Floating-Point (`float`)**: Números de ponto flutuante IEEE 754 de 64 bits (ex. `3.14159`, `-0.001`).
* **Boolean (`bool`)**: Valores lógicos, `true` ou `false`.
* **Character (`char`)**: Literais de caractere de um byte entre aspas simples (ex. `'A'`, `'\n'`, `'\t'`).
* **String (`string`)**: Sequência de caracteres entre aspas duplas (ex. `"Hello, World"`). Suporta interpolação de string usando a sintaxe ``.
* **None (`null`)**: Um tipo especial que representa a ausência de valor, acessado através da palavra-chave `None`.
* **Void**: Significa uma função ou expressão que não retorna nenhum valor.

---

## 2. Lista / Vetor (`list`)

Vetores são arrays dinâmicos ordenados que podem conter elementos de qualquer tipo (incluindo vetores ou objetos aninhados).

```aly
let numbers = [1, 2, 3, 4]
let mixed = ["Aly", 10.5, true, None]

# Index access
print(numbers[0]) # Outputs: 1
```

---

## 3. Objetos / Mapas (`object`)

Objetos são mapeamentos de chave-valor que representam dicionários ou arrays associativos. As chaves são identificadores ou strings.

```aly
let user = {
    name: "Jefferson",
    role: "Developer",
    active: true
}

# Member access
print(user.name) # Outputs: Jefferson
print(user["role"]) # Outputs: Developer
```
