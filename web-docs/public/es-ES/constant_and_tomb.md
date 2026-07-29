# Constantes y el Mecanismo de Congelamiento `tomb` en Aly

Aly admite constantes tradicionales de tiempo de compilación, así como el congelamiento de mutabilidad dinámica en tiempo de ejecución.

---

## 1. Constantes (`const`)

Las constantes se declaran en tiempo de compilación y no se pueden reasignar ni mutar.

```aly
const MAX_LIMIT = 100
```

---

## 2. Congelamiento Dinámico (`tomb`)

La función nativa `tomb(&variable)` bloquea una variable mutable, convirtiéndola en una constante en tiempo de ejecución. Cualquier intento posterior de mutar o reasignar la variable fallará.

```aly
let name = "Pedro"

# En esta etapa, name puede ser mutado
name = "Peter" 

# Congelar la variable 'name'
tomb(&name)

# Intentar mutar 'name' ahora provocará un error
# name = "John" # Error: Reasignación a variable congelada
```

Esta función es útil para garantizar la seguridad de los datos, definir configuraciones de tiempo de ejecución que no deben cambiar después de la configuración y bloquear estados.