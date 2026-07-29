# Concurrencia en Aly

Aly proporciona primitivas de concurrencia básicas a través del tipo `AtomicVar` para el estado compartido seguro entre hilos.

---

## 1. Variables Atómicas

`AtomicVar` envuelve un valor en `Arc<RwLock<ValueData>>`, permitiendo el acceso concurrente seguro:

```aly
import concurrency

let counter = concurrency.AtomicVar.new(0)
```

---

## 2. Creación de Hilos

Los hilos pueden crearse mediante `shell.thread_spawn` y `shell.thread_pool`:

```aly
import shell

fun worker() {
    print("Running in thread")
}

shell.thread_spawn("worker")

shell.thread_pool(["worker", "worker"])
```

`shell.thread_pool` crea múltiples hilos en paralelo y espera a que todos terminen.

---

## 3. Ejecución Paralela de Comandos

```aly
import shell

let results = shell.multi_exec(["echo hello", "echo world"])
```

Ejecuta múltiples comandos de shell concurrentemente y recopila las salidas como un vector.