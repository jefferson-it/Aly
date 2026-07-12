# Control Flow

## Conditionals: `if` / `elif` / `else`

```aly
if x eq 1 {
    print("one")
} elif x eq 2 {
    print("two")
} else {
    print("other")
}
```

- Conditions use the [relational and logical operators](05-operators.md).
- Blocks are delimited by braces.
- `elif` and `else` are optional; you may chain several `elif` blocks.

## Loops: `loop`

Aly unifies `for` and `while` into a single `loop` keyword.

### Conditional loop (while-style)

```aly
let i = 0
loop i lt 5 {
    print(i)
    i++
}
```

### Three-part loop (for-style)

`loop init ; condition ; step { ... }`:

```aly
loop i = 0 ; i lt 5 ; i++ {
    print(i)
}
```

### `do loop` (run-at-least-once)

The body runs once before the condition is checked:

```aly
let i = 0
do {
    print(i)
    i++
} loop i lt 3
```

## Pattern matching: `match`

`match` compares a subject value against a series of arms. The first matching
arm runs, then matching stops.

```aly
match x {
    1 => print("one"),
    2 => print("two"),
    _ => print("something else")
}
```

### Arm bodies

An arm body is either a single expression terminated by a comma, or a brace
block:

```aly
match status {
    "ok" => print("all good"),
    "err" => {
        let msg = "failure"
        print(msg)
    }
    _ => print("unknown")
}
```

### Pattern kinds

| Pattern             | Matches                                   |
| ------------------- | ----------------------------------------- |
| literal (`1`, `"a"`)| exact equality                            |
| `_`                 | anything (wildcard / default)             |
| `a or b or c`       | any of the listed values                  |
| `lo..hi`            | inclusive numeric range (e.g. `10..14`)   |

```aly
match n {
    0 or 1 => print("small"),
    2..9   => print("single digit"),
    _      => print("big")
}
```
