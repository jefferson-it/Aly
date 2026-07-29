# Ponto e Vírgula, Continuação de Linha e Comentários no Aly

Esta página documenta regras léxicas específicas sobre terminação de instrução, instruções de várias linhas e delimitadores de comentários no Aly.

---

## 1. Comentários

O Aly suporta comentários de linha única e de múltiplas linhas (bloco).

* **Comentários de linha única**: Começam com `#` e se estendem até o fim da linha.
* **Comentários de bloco / múltiplas linhas**: Delimitados pelas tags `## ... ##`.

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

Pontos e vírgulas `;` **não** são terminadores de instrução de uso geral no Aly. As instruções são implicitamente terminadas por quebras de linha (newlines) ou delimitadores de bloco `{}`.

### Pontos e Vírgulas em Cabeçalhos de Loop (Único Caso de Uso Válido)
Pontos e vírgulas são permitidos exclusivamente dentro de cabeçalhos de loop para separar a inicialização, verificação condicional e expressões de atualização de iteração.

```aly
# Valid: Semicolons separating expressions inside a loop header
loop let i = 0; i lt 5; i = i + 1 {
    print(i)
}
```

### Pontos e Vírgulas Inválidos
O uso de pontos e vírgulas em qualquer outro lugar no script Aly resultará em um erro do parser de sintaxe:
```aly
let x = 10; # Error: Semicolon used outside of a loop header
print(x);   # Error: Semicolon used outside of a loop header
```

---

## 3. Continuação de Linha (`\`)

Se você precisar quebrar uma instrução longa em várias linhas, anexe uma barra invertida `\` no final da linha. Isso instrui o parser a tratar a próxima linha como uma continuação da instrução atual.

```aly
let msg = "Hello " + "World" \
          " from Aly!"
print(msg) # Outputs: Hello World from Aly!
```

A continuação de linha cessa assim que um bloco `{}` ou bloco baseado em palavra-chave (como `do`/`od`) começa.
