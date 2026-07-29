# Schema em Aly

Schemas em Aly definem estruturas de dados com campos e métodos.

---

## 1. Definição de Schema

Um schema é definido utilizando a palavra-chave `schema`:

```aly
schema Person {
    name: "unknown",
    age: 0,
    
    fun greet() {
        print("Hello, " + @name)
    }
}

let p = new Person()
p.name = "Alice"
p.age = 30
p.greet()
```

---

## 2. Campos com Valores Predefinidos

Os campos podem ter valores predefinidos que são utilizados quando nenhum valor é fornecido:

```aly
schema Settings {
    theme: "dark",
    fontSize: 14
}

let settings = new Settings()
print(settings.theme)  # Resultado: dark
```

---

## 3. Campos Obrigatórios

Campos sem valores predefinidos devem ser fornecidos na criação:

```aly
schema Point {
    x: 0,
    y: 0
}

let p = new Point()
p.x = 10
p.y = 20
```
