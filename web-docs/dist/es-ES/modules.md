# Módulos en Aly

Los módulos permiten organizar el código en archivos separados y controlar la visibilidad de los símbolos con `import` y `export`.

---

## 1. Importación de Módulos

```aly
import math
import fs
import http
```

Los módulos importados registran sus símbolos públicos en el alcance global.

---

## 2. Estructura del Módulo

Aly utiliza un sistema de módulos basado en archivos. Cada archivo `.aly` es un módulo. Los símbolos declarados con `export` se vuelven públicos:

```aly
# math_utils.aly
export fun square(n) {
    return n * n
}

export const PI = 3.14159

# Privado — no visible para los importadores
let internal_counter = 0
```

---

## 3. Importación de Módulos de Usuario

```aly
import "math_utils"

print(square(5))    # Outputs: 25
print(PI)           # Outputs: 3.14159
```

Las rutas pueden ser relativas o absolutas:

```aly
import "lib/utils/helpers"
import "../shared/common"
```

---

## 4. Internals del Sistema de Módulos

El sistema de módulos en tiempo de ejecución (`Module` struct) realiza un seguimiento de:

- **`name`**: Identificador del módulo
- **`public_vars`**: Símbolos exportados a los consumidores
- **`private_vars`**: Símbolos internos (inaccesibles fuera del módulo)

Los símbolos que no están explícitamente `export`ed permanecen privados en el módulo que los define.