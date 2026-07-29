# Strings & Template Interpolation

## String literals

Strings can use double or single quotes:

```aly
let a = "double quoted"
let b = 'single quoted'
```

## Escape sequences

The following escapes are recognized when printing:

| Escape | Meaning         |
| ------ | --------------- |
| `\n`   | newline         |
| `\t`   | tab             |
| `\r`   | carriage return |
| `\"`   | double quote    |
| `\'`   | single quote    |

```aly
print("line 1\nline 2")
```

## Interpolation

Double-quoted strings support interpolation. A variable or object property is
inserted using either the `$` or the `&` sigil:

```aly
let name = "Ana"
print("Hello, $name")        # Hello, Ana
print("Hello, &name")        # Hello, Ana
```

Object properties can be interpolated too:

```aly
let people = { name: "João" }
print("Hello, my name is &people.name")   # Hello, my name is João
```

Both sigils behave the same way; use whichever reads better.

## Length and characters

`.len` returns the number of characters; indexing returns a single character:

```aly
let word = "Hello"
print(word.len)    # 5
print(word.0)      # H
print(word.-1)     # o  (last character)
```

## String helpers

The [`str` standard-library module](11-stdlib.md) provides `upper`, `lower`,
`trim`, `contains`, `replace`, `split`, `starts_with`, `ends_with`, `index_of`
and `repeat`.

```aly
print(str.upper("hello"))            # HELLO
print(str.replace("a-b", "-", "_"))  # a_b
```
