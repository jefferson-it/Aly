# Concurrency in Aly

Aly provides basic concurrency primitives through the `AtomicVar` type for thread-safe shared state.

---

## 1. Atomic Variables

`AtomicVar` wraps a value in `Arc<RwLock<ValueData>>`, enabling safe concurrent access:

```aly
import concurrency

let counter = concurrency.AtomicVar.new(0)
```

---

## 2. Thread Spawning

Threads can be spawned via `shell.thread_spawn` and `shell.thread_pool`:

```aly
import shell

fun worker() {
    print("Running in thread")
}

shell.thread_spawn("worker")

shell.thread_pool(["worker", "worker"])
```

`shell.thread_pool` spawns multiple threads in parallel and waits for all to finish.

---

## 3. Parallel Command Execution

```aly
import shell

let results = shell.multi_exec(["echo hello", "echo world"])
```

Executes multiple shell commands concurrently and collects outputs as a vector.