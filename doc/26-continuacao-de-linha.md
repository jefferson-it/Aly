# Continuação de linha

Um statement pode ser estendido usando um `\"` no final da linha.
Todo o conteúdo subsequente (incluindo quebras de linha) faz parte do
statement, exceto quando um bloco `{ … }` ou a palavra‑chave `do`/`od`
inicia.

**Exemplo**
```aly
print("Hello " + "World" \
    " Extra")
```

A continuação termina ao encontrar um bloco `{ … }` ou `do`/`od`.