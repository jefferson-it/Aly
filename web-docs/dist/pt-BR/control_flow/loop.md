# Loops in Aly

Laços de repetição (Loops) let you repeat block instructions. Aly unifies loops in the `loop` .

---

## 1. Conditional Loop (While style)

Evaluates the expression before entering the loop. It repeats as long as the condition is `true`.

```aly
let i = 1
loop i lte 5 {
    print(i)
    i = i + 1
}
```

---

## 2. Iteration Loop (For style)

You can specify loop initializations, conditions, and increments using semi-colons:

```aly
loop let i = 0; i lt 5; i = i + 1 {
    print(i)
}
```

---

## 3. Loop Control Statements

* **`break` / `pare`**: Exits the loop immediately.
* **`continue`**: Jumps directly to the next loop iteration, bypassing instructions below.
