# Standard Libraries in Aly

Aly includes built-in standard libraries that can be imported to handle basic operational tasks such as timestamps, sleeps, JSON/CSV formatting, and file structures.

---

## 1. Time & Sleep Operations (`timer`, `sys`)

* **`import timer`**: Focuses on scheduling and delays.
  * `timer.sleep(ms)`: Halts program execution for the specified milliseconds.
* **`import sys`**: Interfaces with system metrics.
  * `sys.time()`: Returns Unix time in seconds.

```aly
import timer
import sys

print("Start: " + sys.time())
timer.sleep(1000)
print("End: " + sys.time())
```

---

## 2. File Paths (`path`)

The `path` library provides cross-platform file path resolution utility functions.

* `path.exists(filepath)`: Returns `true` if file exists.
* `path.join(part1, part2)`: Concatenates path directory paths safely.

```aly
import path

if path.exists("config.json") {
    print("Found configuration file.")
}
```
