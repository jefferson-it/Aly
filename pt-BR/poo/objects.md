# Objetos no Aly

Objetos no Aly são estruturas dinâmicas de chave-valor (mapas). Eles podem conter variáveis, objetos aninhados e funções (atuando como métodos).

---

## 1. Instanciação de Objetos

Um objeto literal é declarado usando chaves `{}`.

```aly
let user = {
    name: "Jefferson",
    role: "Developer",
    greet: fun(self) {
        print("Hello, my name is " + self.name)
    }
}

user.greet(user) # Exibe: Hello, my name is Jefferson
```

---

## 2. Modificações Dinâmicas

Propriedades e métodos podem ser adicionados ou sobrescritos dinamicamente:

```aly
let player = {}
player.score = 0
player.increment = fun(self, points) {
    self.score = self.score + points
}

player.increment(player, 10)
print(player.score) # Exibe: 10
```
