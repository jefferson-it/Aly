# REPL y Programador en Aly

Aly proporciona un indicador REPL (Read-Eval-Print Loop) interactivo y un programador de tareas cooperativo.

---

## 1. REPL Interactivo

Iniciar el REPL:

```bash
aly repl
```

O desde el runtime:
```bash
cargo run --bin aly -- repl
```

El indicador evalúa expresiones de Aly de forma interactiva:

```
aly> let x = 10
aly> x * 2
20
aly> fun square(n) { return n * n }
aly> square(5)
25
```

---

## 2. Programador

El programador cooperativo gestiona tareas concurrentes:

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

Las tareas ceden voluntariamente el control, permitiendo que el programador intercale la ejecución de forma cooperativa.

---

## 3. Módulo Prompt

El módulo `prompt` proporciona el backend del REPL:

```aly
import prompt

let input = prompt.read("aly> ")
let result = prompt.eval(input)
print(result)
```

Características compatibles:
- Historial de comandos (flechas arriba/abajo)
- Finalización con tabulación
- Entrada de varias líneas
- Resaltado de errores