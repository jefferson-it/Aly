# REPL e Planificador em Aly

Aly fornece um prompt REPL (Read-Eval-Print Loop) interativo e um planificador cooperativo de tarefas.

---

## 1. REPL Interactivo

Iniciar o REPL:

```bash
aly repl
```

Ou a partir do tempo de execução:
```bash
cargo run --bin aly -- repl
```

O prompt avalia expressões Aly interactivamente:

```
aly> let x = 10
aly> x * 2
20
aly> fun square(n) { return n * n }
aly> square(5)
25
```

---

## 2. Planificador

O planificador cooperativo gere tarefas concorrentes:

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

As tarefas cedem voluntariamente o controlo, permitindo que o planificador intercale a execução de forma cooperativa.

---

## 3. Modulo Prompt

O módulo `prompt` fornece o backend REPL:

```aly
import prompt

let input = prompt.read("aly> ")
let result = prompt.eval(input)
print(result)
```

Suporta:
- Histórico de comandos (setas cima/baixo)
- Conclusão com Tab
- Entrada de múltiplas linhas
- Destaque de erros
