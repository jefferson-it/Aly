# Pontos e Vírgulas, Continuação de Linha e Comentários em Aly

Esta página documenta regras lexicais específicas sobre terminadores de instruções, instruções de múltiplas linhas e delimitadores de comentários em Aly.

---

## 1. Comentários

Aly suporta comentários de linha única e de múltiplas linhas (bloco).

* **Comentários de Linha Única**: Começam com `#` e estendem-se até ao fim da linha.
* **Comentários de Múltiplas Linhas / Bloco**: Envolvidos entre tags `## ... ##`.

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

Os pontos e vírgulas `;` **não** são terminadores gerais de instruções em Aly. As instruções são implicitamente terminadas por quebras de linha (novas linhas) ou delimitadores de bloco `{}`.

### Pontos e Vírgulas em Cabeçalhos de Loop (Único Caso Válido)

Os pontos e vírgulas são exclusivamente permitidos dentro de cabeçalhos de loop para separar inicialização, verificação de condição e expressões de actualização de iteração.

```aly
# Válido: Pontos e vírgulas a separar expressões dentro de um cabeçalho de loop
loop let i = 0; i lt 5; i = i + 1 {
    print(i)
}
```

### Pontos e Vírgulas Inválidos

Utilizar pontos e vírgulas em qualquer outro lugar num script Aly resultará num erro do analisador sintáctico:
```aly
let x = 10; # Erro: Ponto e vírgula utilizado fora de um cabeçalho de loop
print(x);   # Erro: Ponto e vírgula utilizado fora de um cabeçalho de loop
```

---

## 3. Continuação de Linha (`\`)

Se precisar de dividir uma instrução longa em múltiplas linhas, adicione uma barra invertida `\` no fim da linha. Isto instrui o analisador a tratar a linha seguinte como continuação da instrução actual.

```aly
let msg = "Hello " + "World" \
          " from Aly!"
print(msg) # Resultado: Hello World from Aly!
```

A continuação de linha cessa assim que um bloco `{}` ou bloco baseado em palavras-chave (como `do`/`od`) começa.
