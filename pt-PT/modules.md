# Módulos em Aly

Módulos permitem organizar código em ficheiros separados e controlar a visibilidade de símbolos com `import` e `export`.

---

## 1. Importação de Módulos

```aly
import math
import fs
import http
```

Módulos importados registam os seus símbolos públicos noâmbito global.

---

## 2. Estrutura do Módulo

Aly utiliza um sistema de módulos baseado em ficheiros. Cada ficheiro `.aly` é um módulo. Símbolos declarados com `export` tornam-se públicos:

```aly
# math_utils.aly
export fun square(n) {
    return n * n
}

export const PI = 3.14159

# Privado — invisível para importadores
let internal_counter = 0
```

---

## 3. Importação de Módulos de Utilizador

```aly
import "math_utils"

print(square(5))    # Resultado: 25
print(PI)           # Resultado: 3.14159
```

Caminhos podem ser relativos ou absolutos:

```aly
import "lib/utils/helpers"
import "../shared/common"
```

---

## 4. Internals do Sistema de Módulos

O sistema de módulos de tempo de execução (`Module` struct) regista:

- **`name`**: Identificador do módulo
- **`public_vars`**: Símbolos exportados para consumidores
- **`private_vars`**: Símbolos internos (inacessíveis fora do módulo)

Símbolos que não são explicitamente `export`ed permanecem privados ao módulo que os define.
