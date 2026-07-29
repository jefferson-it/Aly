# Schemas (POO baseada em classes) no Aly

Schemas fornecem um sistema de blueprint semelhante a classes para instanciar objetos estruturados com valores de campo padrão e métodos membros.

---

## 1. Definição de Schema

Um schema é declarado usando a palavra-chave `schema`.

```aly
schema Person {
    name: "John",
    age: 25,
    fun greet() {
        print("Hello, my name is " + @name + " and I am " + @age)
    }
}
```

* **Prefixo `@`**: Usado dentro de métodos do schema para referenciar campos pertencentes à instância atual (equivalente a `this` ou `self`).

---

## 2. Instanciando Schemas (`new`)

Instâncias são criadas usando a palavra-chave `new`.

### Construtor Padrão
Se nenhum argumento for passado, os campos são inicializados com seus valores padrão:
```aly
let p = new Person()
p.greet() # Exibe: Hello, my name is John and I am 25
```

### Argumentos do Construtor
Passar argumentos para `new Schema(...)` sobrescreve os valores padrão dos campos na ordem em que foram declarados:
```aly
let custom = new Person("Jefferson", 30)
custom.greet() # Exibe: Hello, my name is Jefferson and I am 30
```
