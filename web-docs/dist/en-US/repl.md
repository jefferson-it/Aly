# REPL and Scheduler in Aly

Aly provides an interactive REPL (Read-Eval-Print Loop) prompt and a cooperative task scheduler.

---

## 1. Interactive REPL

Launch the REPL:

```bash
aly repl
```

Or from the runtime:
```bash
cargo run --bin aly -- repl
```

The prompt evaluates Aly expressions interactively:

```
aly> let x = 10
aly> x * 2
20
aly> fun square(n) { return n * n }
aly> square(5)
25
```

---

## 2. Scheduler

The cooperative scheduler manages concurrent tasks:

```aly
import scheduler

scheduler.task(fun() {
    loop {
        print("Task 1 running")
        scheduler.yield()
    }
})

scheduler.task(fun() {
    loop {
        print("Task 2 running")
        scheduler.yield()
    }
})

scheduler.run()
```

Tasks voluntarily yield control, allowing the scheduler to interleave execution cooperatively.

---

## 3. Prompt Module

The `prompt` module provides the REPL backend:

```aly
import prompt

let input = prompt.read("aly> ")
let result = prompt.eval(input)
print(result)
```

Supports:
- Command history (up/down arrows)
- Tab completion
- Multi-line input
- Error highlighting
