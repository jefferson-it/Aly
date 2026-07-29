# Tools in Aly

Aly ships with a suite of development tools for formatting, linting, language server protocol, debugging, testing, and hot reloading.

---

## 1. Code Formatter (`fmt`)

```bash
aly fmt input.aly          # Format a single file
aly fmt input.aly -o output.aly  # Format and write to file
```

The formatter standardizes indentation, spacing, and brace placement.

---

## 2. Linter (`linter`)

```bash
aly lint input.aly
```

Checks for common issues:
- Unused variables
- Unreachable code
- Type mismatches
- Missing return values

---

## 3. Language Server Protocol (`lsp`)

```bash
aly lsp
```

Provides IDE features through LSP:
- Auto-completion
- Go-to-definition
- Hover type information
- Diagnostic reporting
- Code actions

---

## 4. Debugger (`debugger`)

```bash
aly debug input.aly
```

Interactive debugging features:
- Step through execution
- Breakpoints
- Variable inspection
- Call stack traces

---

## 5. Test Runner (`test_runner`)

```bash
aly test
```

Discovers and runs test functions:
```aly
fun test_addition() {
    assert(2 + 2 eq 4)
}

fun test_subtraction() {
    assert(5 - 3 eq 2)
}
```

---

## 6. Hot Reload (`hotreload`)

```bash
aly run --watch input.aly
```

Automatically detects file changes and reloads the script without restarting, preserving application state where possible.

---

## 7. Documentation Generator (`doc`)

```bash
aly doc input.aly -o docs/
```

Generates Markdown documentation from source code comments and annotations.

---

## 8. JNI Generator (`jni_gen`)

```bash
aly jni-gen input.aly
```

Generates JNI bindings for calling Aly from Java code.