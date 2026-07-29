# Variáveis e Mutabilidade em Aly

Aly fornece suporte para variáveis mutáveis e constantes imutáveis. Utiliza escopo léxico baseado em blocos e inferência de tipos dinâmica.

---

## 1. Declaração de Variáveis (`let`)

A palavra-chave `let` é utilizada para declarar variáveis mutáveis. Uma variável mutável pode ser reassociada a um valor diferente em qualquer ponto após a inicialização.

```aly
let score = 100
score = score + 50
print(score) # Resultado: 150
```

---

## 2. Declaração de Constantes (`const`)

A palavra-chave `const` é utilizada para declarar constantes imutáveis. Uma vez inicializado, o valor de uma constante não pode ser alterado. Tentar atribuir um novo valor a uma constante resultará num erro do compilador/interpretador.

```aly
const PI = 3.14159
# PI = 3.2 # Erro: Reatribuição de constante
```

---

## 3. Escopo Léxico

Variáveis e constantes em Aly são de escopo de bloco. Um bloco é definido por chavetas `{}`.

* **Escopo Global**: Variáveis declaradas fora de qualquer bloco são acessíveis em qualquer parte do ficheiro.
* **Escopo Local**: Variáveis declaradas dentro de um bloco só são visíveis dentro desse bloco e de quaisquer blocos aninhados. Assim que a execução sai do bloco, a variável local é descartada.

```aly
let x = 10

if true {
    let y = 20
    print(x + y) # Resultado: 30
}

# print(y) # Erro: y não está definido neste escopo
```

---

## 4. Inferência Dinâmica de Tipos

Aly não requer assinaturas de tipo explícitas (como `int x = 10`). O compilador/interpretador determina automaticamente o tipo com base no valor atribuído em tempo de execução ou tempo de compilação.
