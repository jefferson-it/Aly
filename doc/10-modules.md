# Modules

Aly programs can be split across multiple `.aly` files and combined with
`import`.

## Importing

`import "path"` reads and executes another file, making its definitions
(variables, constants and functions) available to the importing file:

```aly
# main.aly
import "greetings.aly"

print(greeting)
say_hello("Mundo")
```

```aly
# greetings.aly
const greeting = "Hello from imported file!"

fun say_hello(name) {
    print("Hello, " + name + "!")
}
```

### Path resolution

Relative import paths are resolved **against the directory of the importing
file**, so nested modules work naturally:

```aly
# module/main.aly
import "lib/math.aly"

print("PI = " + PI)
print("square(5) = " + square(5))
```

### Circular imports

A file that has already been imported is skipped on subsequent imports, so
cycles between modules are safe and do not cause infinite loops.

## Exporting

`export name` marks a variable, constant or function as part of a module's
public API:

```aly
# lib.aly
const version = "1.0.0"
export version

fun add(a, b) {
    return a + b
}
export add
```

Use `export` to document and declare the intended public surface of a module.
