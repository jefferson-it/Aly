# Concorrência em Aly

Aly fornece primitivas básicas de concorrência através do tipo `AtomicVar` para estado partilhado seguro entre threads.

---

## 1. Variáveis Atómicas

`AtomicVar` envolve um valor em `Arc<RwLock<ValueData>>`, permitindo acesso concorrente seguro:

```aly
import concurrency

let counter = concurrency.AtomicVar.new(0)
```

---

## 2. Criação de Threads

Threads podem ser criadas via `shell.thread_spawn` e `shell.thread_pool`:

```aly
import shell

fun worker() {
    print("A executar numa thread")
}

shell.thread_spawn("worker")

shell.thread_pool(["worker", "worker"])
```

`shell.thread_pool` cria múltiplas threads em paralelo e aguarda a conclusão de todas.

---

## 3. Execução Paralela de Comandos

```aly
import shell

let results = shell.multi_exec(["echo hello", "echo world"])
```

Executa múltiplos comandos de consola simultaneamente e recolhe as saídas como um vetor.
