# Funções no Aly

As funções são valores de primeira classe definidos com a palavra-chave `fun`. Elas podem ser nomeadas, anônimas, armazenadas em variáveis, passadas como argumentos e retornadas de outras funções.

---

## 1. Definição de Função Nomeada

```aly
fun add(a, b) {
    return a + b
}

print(add(3, 4))  # Outputs: 7
```

Os parâmetros são posicionais. Parênteses ao redor dos parâmetros são obrigatórios. Chaves delimitam o corpo da função.

---

## 2. Valores de Retorno

Uma função retorna o valor da última expressão avaliada ou explicitamente com `return`:

```aly
fun square(n) {
    return n * n
}

fun double(n) {
    n * 2      # Implicit return (last expression)
}

print(square(4))   # Outputs: 16
print(double(5))   # Outputs: 10
```

`return` sem uma expressão retorna `None`.

---

## 3. Funções Anônimas (Lambdas/Closures)

Funções sem nome podem ser atribuídas a variáveis ou passadas inline:

```aly
let multiply = fun(a, b) {
    return a * b
}

print(multiply(3, 4))  # Outputs: 12
```

Funções anônimas capturam variáveis do seu escopo envolvente (closures).

```aly
let factor = 2
let doubler = fun(n) {
    return n * factor   # Captures `factor` from outer scope
}

print(doubler(5))  # Outputs: 10
```

---

## 4. Parâmetros Padrão

Os parâmetros podem ter valores padrão:

```aly
fun greet(name, greeting = "Hello") {
    print(greeting + ", " + name)
}

greet("Alice")              # Outputs: Hello, Alice
greet("Bob", "Hi")          # Outputs: Hi, Bob
```

---

## 5. Funções Variádicas

Um parâmetro variádico coleta os argumentos excedentes em uma lista usando `...`:

```aly
fun sum_all(...nums) {
    let total = 0
    loop let i = 0; i lt nums.len; i = i + 1 {
        total = total + nums[i]
    }
    return total
}

print(sum_all(1, 2, 3, 4))  # Outputs: 10
```

É permitido apenas um parâmetro variádico, e ele deve ser o último parâmetro.

---

## 6. Funções de Primeira Classe

As funções podem ser armazenadas em estruturas de dados e passadas:

```aly
let operations = {
    add: fun(a, b) { return a + b },
    sub: fun(a, b) { return a - b },
}

print(operations.add(10, 5))  # Outputs: 15
```