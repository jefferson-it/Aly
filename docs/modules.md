# Modules in Aly

Modules allow organizing code into separate files and controlling symbol visibility with `import` and `export`.

---

## 1. Importing Modules

```aly
import math
import fs
import http
```

Imported modules register their public symbols into the global scope.

---

## 2. Module Structure

Aly uses a file-based module system. Each `.aly` file is a module. Symbols declared with `export` become public:

```aly
# math_utils.aly
export fun square(n) {
    return n * n
}

export const PI = 3.14159

# Private — not visible to importers
let internal_counter = 0
```

---

## 3. Importing User Modules

```aly
import "math_utils"

print(square(5))    # Outputs: 25
print(PI)           # Outputs: 3.14159
```

Paths can be relative or absolute:

```aly
import "lib/utils/helpers"
import "../shared/common"
```

---

## 4. Module System Internals

The runtime module system (`Module` struct) tracks:

- **`name`**: Module identifier
- **`public_vars`**: Symbols exported to consumers
- **`private_vars`**: Internal symbols (inaccessible outside the module)

Symbols that are not explicitly `export`ed remain private to the defining module.