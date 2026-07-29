# Funções em Aly

Funções são valores de primeira classe definidos com a palavra-chave `fun`. Podem ser nomeadas, anónimas, guardadas em variáveis, passadas como argumentos e devolvidas por outras funções.

---

## 1. Definição de Função Nomeada

```aly
fun add(a, b) {
    return a + b
}

print(add(3, 4))  # Resultado: 7
```

Os parâmetros são posicionais. Os parêntesis entre parâmetros são obrigatórios. As chavetas delimitam o corpo da função.

---

## 2. Valores de Retorno

Uma função devolve o valor da última expressão avaliada, ou explicitamente com `return`:

```aly
fun square(n) {
    return n * n
}

fun double(n) {
    n * 2      # Retorno implícito (última expressão)
}

print(square(4))   # Resultado: 16
print(double(5))   # Resultado: 10
```

`return` sem uma expressão devolve `None`.

---

## 3. Funções Anónimas (Lambdas/Closures)

Funções sem nome podem ser atribuídas a variáveis ou passadas inline:

```aly
let multiply = fun(a, b) {
    return a * b
}

print(multiply(3, 4))  # Resultado: 12
```

As funções anónimas capturam variáveis do seu escopo envolvente (closures).

```aly
let factor = 2
let doubler = fun(n) {
    return n * factor   # Captura `factor` do escopo externo
}

print(doubler(5))  # Resultado: 10
```

---

## 4. Parâmetros Predefinidos

Os parâmetros podem ter valores predefinidos:

```aly
fun greet(name, greeting = "Hello") {
    print(greeting + ", " + name)
}

greet("Alice")              # Resultado: Hello, Alice
greet("Bob", "Hi")          # Resultado: Hi, Bob
```

---

## 5. Funções Variádicas

Um parâmetro variádico recolhe argumentos excessivos numa lista usando `...`:

```aly
fun sum_all(...nums) {
    let total = 0
    loop let i = 0; i lt nums.len; i = i + 1 {
        total = total + nums[i]
    }
    return total
}

print(sum_all(1, 2, 3, 4))  # Resultado: 10
```

Apenas um parâmetro variádico é permitido e deve ser o último parâmetro.

---

## 6. Funções de Primeira Classe

Funções podem ser guardadas em estruturas de dados e passadas:

```aly
let operations = {
    add: fun(a, b) { return a + b },
    sub: fun(a, b) { return a - b },
}

print(operations.add(10, 5))  # Resultado: 15
```
