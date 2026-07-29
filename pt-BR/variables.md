# Variáveis e Mutabilidade no Aly

O Aly oferece suporte tanto para variáveis mutáveis quanto para constantes imutáveis. Ele usa escopo léxico baseado em blocos e inferência dinâmica de tipos.

---

## 1. Declarações de Variável (`let`)

A palavra-chave `let` é usada para declarar variáveis mutáveis. Uma variável mutável pode ser reatribuída a um valor diferente em qualquer ponto após a inicialização.

```aly
let score = 100
score = score + 50
print(score) # Outputs: 150
```

---

## 2. Declarações de Constante (`const`)

A palavra-chave `const` é usada para declarar constantes imutáveis. Uma vez inicializado, o valor de uma constante não pode ser alterado. Tentar atribuir um novo valor a uma constante resultará em um erro do compilador/interpretador.

```aly
const PI = 3.14159
# PI = 3.2 # Error: Constant reassignment
```

---

## 3. Escopo Léxico

As variáveis e constantes no Aly têm escopo de bloco. Um bloco é definido por chaves `{}`.

* **Escopo Global**: Variáveis declaradas fora de qualquer bloco são acessíveis em qualquer lugar no arquivo.
* **Escopo Local**: Variáveis declaradas dentro de um bloco são visíveis apenas dentro daquele bloco e em quaisquer blocos aninhados. Assim que a execução sai do bloco, a variável local é descartada.

```aly
let x = 10

if true {
    let y = 20
    print(x + y) # Outputs: 30
}

# print(y) # Error: y is not defined in this scope
```

---

## 4. Inferência Dinâmica de Tipos

O Aly não exige assinaturas de tipo explícitas (como `int x = 10`). O compilador/interpretador determina automaticamente o tipo com base no valor atribuído em tempo de execução ou tempo de compilação.
