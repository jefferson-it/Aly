# Ponto e Vírgula, Continuação de Linha e Comentários no Aly

Esta página documenta regras lexicais específicas referentes à terminação de declarações, declarações de múltiplas linhas e delimitadores de comentários no Aly.

---

## 1. Comentários

O Aly suporta comentários de uma única linha e de múltiplas linhas (bloco).

* **Comentários de uma única linha**: Começam com `#` e estendem-se até ao final da linha.
* **Comentários de múltiplas linhas / Bloco**: Delimitados pelas etiquetas `## ... ##`.

```aly
# This is a single-line comment

##
This is a multi-line block comment
spanning multiple lines of text
##
let x = 10
```

---

## 2. Semântica do Ponto e Vírgula (`;`)

Os pontos e vírgulas `;` **não** são terminadores de declaração de uso geral no Aly. As declarações são terminadas implicitamente por quebras de linha (novas linhas) ou delimitadores de bloco `{}`.

### Ponto e Vírgula em Cabeçalhos de Laços (Único Caso de Uso Válido)
Os pontos e vírgulas são permitidos exclusivamente dentro dos cabeçalhos dos laços para separar a inicialização, a verificação condicional e as expressões de atualização de iteração.

```aly
# Valid: Semicolons separating expressions inside a loop header
loop let i = 0; i lt 5; i = i + 1 {
    print(i)
}
```

### Pontos e Vírgulas Inválidos
A utilização de pontos e vírgulas em qualquer outro lugar no script Aly resultará num erro de sintaxe do analisador:
```aly
let x = 10; # Error: Semicolon used outside of a loop header
print(x);   # Error: Semicolon used outside of a loop header
```

---

## 3. Continuação de Linha (`\`)

Se precisar de quebrar uma declaração longa em múltiplas linhas, adicione uma barra invertida `\` no final da linha. Isto instrui o analisador a tratar a linha seguinte como uma continuação da declaração atual.

```aly
let msg = "Hello " + "World" \
          " from Aly!"
print(msg) # Outputs: Hello World from Aly!
```

A continuação de linha cessa assim que se inicia um bloco `{}` ou um bloco baseado em palavras-chave (como `do`/`od`).
