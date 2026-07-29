# Objectos em Aly

Objectos em Aly são estruturas dinâmicas de mapa chave-valor. Podem conter variáveis, objectos aninhados e funções (actuando como métodos).

---

## 1. Instanciação de Objectos

Um literal de objecto é declarado utilizando chavetas `{}`.

```aly
let user = {
    name: "Jefferson",
    role: "Developer",
    greet: fun(self) {
        print("Hello, my name is " + self.name)
    }
}

user.greet(user) # Resultado: Hello, my name is Jefferson
```

---

## 2. Modificações Dinâmicas

Propriedades e métodos podem ser adicionados ou substituídos dinamicamente:

```aly
let player = {}
player.score = 0
player.increment = fun(self, points) {
    self.score = self.score + points
}

player.increment(player, 10)
print(player.score) # Resultado: 10
```
