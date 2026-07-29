# Constants and the `tomb` Freezing Mechanism in Aly

Aly supports traditional compile-time constants as well as dynamic mutability freezing at runtime.

---

## 1. Constants (`const`)

Constants are declared at compile-time and cannot be rebound or mutated.

```aly
const MAX_LIMIT = 100
```

---

## 2. Dynamic Freezing (`tomb`)

The native function `tomb(&variable)` locks a mutable variable, converting it into a constant at runtime. Any subsequent attempts to mutate or rebind the variable will fail.

```aly
let name = "Pedro"

# At this stage, name can be mutated
name = "Peter" 

# Freeze the variable 'name'
tomb(&name)

# Attempting to mutate 'name' now will trigger an error
# name = "John" # Error: Reassignment to frozen variable
```

This feature is useful for ensuring data safety, defining runtime configurations that must not change after setup, and locking states.
