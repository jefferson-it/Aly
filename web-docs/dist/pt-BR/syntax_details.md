# Ponto e vírgula, continuação de linha e comentários em Aly

Esta página documenta regras léxicas específicas sobre terminação de declarações, declarações multi-linha e delimitadores de comentários em Aly.

---

## 1. Comentários

Aly suporta comentários de linha única e multi-linha (bloco).

* **Single-line Comentários**: Começam com `#` e se estendem até o final da linha.
* **Multi-line / Block Comentários**: Delimitados por `## ... ##` tags.

```aly
# This is a single-line comment

##
This is a multi-line block comment
spanning multiple lines of text
##
let x = 10
```

---

## 2. Semântica do ponto e vírgula (`;`)

Ponto e vírgula `;` are **not** general-purpose statement terminators in Aly. Statements are implicitly terminated by line breaks (newlines) or block delimiters `{}`.

### Ponto e vírgula in Loop Headers (Only Valid Use-Case)
Ponto e vírgula are exclusively permitted inside loop headers to separate initialization, conditional check, and iteration update expressions.

```aly
# Valid: Ponto e vírgula separating expressions inside a loop header
loop let i = 0; i lt 5; i = i + 1 {
    print(i)
}
```

### Invalid Ponto e vírgula
Using semicolons anywhere else in Aly script will result in a syntax parser error:
```aly
let x = 10; # Error: Semicolon used outside of a loop header
print(x);   # Error: Semicolon used outside of a loop header
```

---

## 3. Continuação de linha (`\`)

If you need to break a long statement across multiple lines, append a backslash `\` at the end of the line. This instructs the parser to treat the following line as a continuation of the current statement.

```aly
let msg = "Hello " + "World" \
          " from Aly!"
print(msg) # Outputs: Hello World from Aly!
```

Line continuation ceases as soon as a block `{}` or keyword-based block (like `do`/`od`) begins.
