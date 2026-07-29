# Variáveis e Mutabilidade no Aly

O Aly fornece suporte tanto para variáveis mutáveis como para constantes imutáveis. Utiliza escopo lexical baseado em blocos e inferência de tipos dinâmica.

---

## 1. Declarações de Variáveis (`let`)

A palavra-chave `let` é utilizada para declarar variáveis mutáveis. Uma variável mutável pode ser reatribuída a um valor diferente em qualquer ponto após a inicialização.

```aly
let score = 100
score = score + 50
print(score) # Outputs: 150
```

---

## 2. Declarações de Constantes (`const`)

A palavra-chave `const` é utilizada para declarar constantes imutáveis. Uma vez inicializado, o valor de uma constante não pode ser alterado. Tentar atribuir um novo valor a uma constante resultará num erro do compilador/interpretador.

```aly
const PI = 3.14159
# PI = 3.2 # Error: Constant reassignment
```

---

## 3. Escopo Lexical

As variáveis e constantes no Aly têm escopo de bloco. Um bloco é definido por chavetas `{}`.

* **Escopo Global**: As variáveis declaradas fora de qualquer bloco são acessíveis em qualquer lugar no ficheiro.
* **Escopo Local**: As variáveis declaradas dentro de um bloco são visíveis apenas dentro desse bloco e de quaisquer blocos aninhados. Assim que a execução sai do bloco, a variável local é descartada.

```aly
let x = 10

if true {
    let y = 20
    print(x + y) # Outputs: 30
}

# print(y) # Error: y is not defined in this scope
```

---

## 4. Inferência de Tipos Dinâmica

O Aly não exige assinaturas de tipos explícitas (como `int x = 10`). O compilador/interpretador determina automaticamente o tipo com base no valor atribuído em tempo de execução ou de compilação.
