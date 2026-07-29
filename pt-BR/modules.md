# Módulos no Aly

Os módulos permitem organizar o código em arquivos separados e controlar a visibilidade de símbolos com `import` e `export`.

---

## 1. Importando Módulos

```aly
import math
import fs
import http
```

Módulos importados registram seus símbolos públicos no escopo global.

---

## 2. Estrutura do Módulo

O Aly usa um sistema de módulos baseado em arquivos. Cada arquivo `.aly` é um módulo. Símbolos declarados com `export` tornam-se públicos:

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

## 3. Importando Módulos do Usuário

```aly
import "math_utils"

print(square(5))    # Outputs: 25
print(PI)           # Outputs: 3.14159
```

Caminhos podem ser relativos ou absolutos:

```aly
import "lib/utils/helpers"
import "../shared/common"
```

---

## 4. Internos do Sistema de Módulos

O sistema de módulos de tempo de execução (runtime) (struct `Module`) rastreia:

- **`name`**: Identificador do módulo
- **`public_vars`**: Símbolos exportados para consumidores
- **`private_vars`**: Símbolos internos (inacessíveis fora do módulo)

Símbolos que não são explicitamente exportados com `export` permanecem privados ao módulo definidor.
