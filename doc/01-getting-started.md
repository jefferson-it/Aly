# Getting Started

## Running a program

Aly source files use the `.aly` extension. Run one with the `run` action:

```sh
Aly run main.aly
```

The interpreter reads the file, tokenizes it line by line, and executes each
statement in order.

## Actions

The CLI accepts an action as the first argument:

| Action | Description                                  |
| ------ | -------------------------------------------- |
| `run`  | Execute an `.aly` file.                      |
| `cli`  | Interactive mode (default when none given).  |
| `comp` | Compilation action (reserved / in progress). |

If no file is given, the interpreter defaults to `main.ba`.

## Hello world

```aly
print("Hello, world!")
```

```sh
$ Aly run hello.aly
Hello, world!
```

## Program structure

- One statement per line (a trailing `;` is also accepted as a separator).
- Blocks are delimited by braces `{ ... }`.
- Comments start with `#`, or span multiple lines between `##` markers.

```aly
# This is a line comment

##
This is a
multi-line comment
##

print("done")
```
