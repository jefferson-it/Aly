# Syntax & Comments

## Statements

A statement is normally one line of code. The interpreter groups tokens per line
and executes them once a line (or a complete brace block) is read.

```aly
let a = 1
let b = 2
print(a + b)
```

A semicolon `;` may be used as an explicit separator, e.g. inside loop headers:

```aly
loop i = 0 ; i lt 3 ; i++ {
    print(i)
}
```

## Comments

### Line comments

Everything after `#` on a line is ignored:

```aly
print("hi")   # this is ignored
```

### Multi-line comments

Text between two `##` markers is ignored:

```aly
##
Everything here is a comment,
across as many lines as needed.
##
```

## Blocks

Braces `{ ... }` delimit the bodies of `if`, `loop`, `match`, functions, etc.

```aly
if x eq 1 {
    print("one")
}
```

## Identifiers

Identifiers name variables, constants and functions. They are made of letters,
digits and underscores, and are referenced directly by name.

The `&` prefix takes a **reference/address** of a variable (used, for example,
by the `tomb` keyword and pointer-style arguments).

## Reserved words

```
let    const   fun     return   if     elif   else
loop   do      match   tomb     import export new
struct model   try     catch    async  await
and    or      xor     not
eq     neq     lt      lte      gt     gte
true   false   None    _
```
