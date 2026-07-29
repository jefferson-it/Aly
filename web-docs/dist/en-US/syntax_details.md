# Semicolons, Line Continuation, and Comments in Aly

This page documents specific lexical rules regarding statement termination, multi-line statements, and comment delimiters in Aly.

---

## 1. Comments

Aly supports single-line and multi-line (block) comments.

* **Single-line Comments**: Start with `#` and extend to the end of the line.
* **Multi-line / Block Comments**: Enclosed within `## ... ##` tags.

```aly
# This is a single-line comment

##
This is a multi-line block comment
spanning multiple lines of text
##
let x = 10
```

---

## 2. Semicolon Semantics (`;`)

Semicolons `;` are **not** general-purpose statement terminators in Aly. Statements are implicitly terminated by line breaks (newlines) or block delimiters `{}`.

### Semicolons in Loop Headers (Only Valid Use-Case)
Semicolons are exclusively permitted inside loop headers to separate initialization, conditional check, and iteration update expressions.

```aly
# Valid: Semicolons separating expressions inside a loop header
loop let i = 0; i lt 5; i = i + 1 {
    print(i)
}
```

### Invalid Semicolons
Using semicolons anywhere else in Aly script will result in a syntax parser error:
```aly
let x = 10; # Error: Semicolon used outside of a loop header
print(x);   # Error: Semicolon used outside of a loop header
```

---

## 3. Line Continuation (`\`)

If you need to break a long statement across multiple lines, append a backslash `\` at the end of the line. This instructs the parser to treat the following line as a continuation of the current statement.

```aly
let msg = "Hello " + "World" \
          " from Aly!"
print(msg) # Outputs: Hello World from Aly!
```

Line continuation ceases as soon as a block `{}` or keyword-based block (like `do`/`od`) begins.
