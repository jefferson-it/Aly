# REPL e Scheduler no Aly

O Aly fornece um prompt interativo REPL (Read-Eval-Print Loop) e um agendador de tarefas cooperativo.

---

## 1. REPL Interativo

Inicie o REPL:

```bash
aly repl
```

Ou a partir do runtime:
```bash
cargo run --bin aly -- repl
```

O prompt avalia expressões Aly interativamente:

```
aly> let x = 10
aly> x * 2
20
aly> fun square(n) { return n * n }
aly> square(5)
25
```

---

## 2. Agendador

O scheduler cooperativo gerencia tarefas concorrentes:

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

As tarefas cedem controle voluntariamente (yield), permitindo que o scheduler intercale a execução cooperativamente.

---

## 3. Módulo Prompt

O módulo `prompt` fornece o backend do REPL:

```aly
import prompt

let input = prompt.read("aly> ")
let result = prompt.eval(input)
print(result)
```

Suporta:
- Histórico de comandos (setas para cima/baixo)
- Autocompletar com Tab
- Entrada de múltiplas linhas
- Destaque de erros (Error highlighting)