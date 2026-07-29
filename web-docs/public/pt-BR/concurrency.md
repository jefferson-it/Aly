# Concorrência no Aly

O Aly fornece primitivas básicas de concorrência através do tipo `AtomicVar` para estado compartilhado thread-safe.

---

## 1. Variáveis Atômicas

`AtomicVar` envolve um valor em `Arc<RwLock<ValueData>>`, permitindo acesso simultâneo seguro:

```aly
import concurrency

let counter = concurrency.AtomicVar.new(0)
```

---

## 2. Spawning de Threads

Threads podem ser criadas via `shell.thread_spawn` e `shell.thread_pool`:

```aly
import shell

fun worker() {
    print("Running in thread")
}

shell.thread_spawn("worker")

shell.thread_pool(["worker", "worker"])
```

`shell.thread_pool` gera múltiplas threads em paralelo e aguarda a conclusão de todas.

---

## 3. Execução Paralela de Comandos

```aly
import shell

let results = shell.multi_exec(["echo hello", "echo world"])
```

Executa múltiplos comandos de shell concorrentemente e coleta as saídas como um vetor.