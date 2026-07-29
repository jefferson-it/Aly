# Constantes e o Mecanismo de Congelamento `tomb` no Aly

O Aly suporta constantes tradicionais em tempo de compilação, bem como congelamento de mutabilidade dinâmico em tempo de execução.

---

## 1. Constantes (`const`)

Constantes são declaradas em tempo de compilação e não podem ser reatribuídas ou mutadas.

```aly
const MAX_LIMIT = 100
```

---

## 2. Congelamento Dinâmico (`tomb`)

A função nativa `tomb(&variable)` bloqueia uma variável mutável, convertendo-a em uma constante em tempo de execução. Quaisquer tentativas subsequentes de mutar ou reatribuir a variável falharão.

```aly
let name = "Pedro"

# At this stage, name can be mutated
name = "Peter" 

# Freeze the variable 'name'
tomb(&name)

# Attempting to mutate 'name' now will trigger an error
# name = "John" # Error: Reassignment to frozen variable
```

Esse recurso é útil para garantir a segurança dos dados, definir configurações de tempo de execução que não devem mudar após a configuração e bloquear estados.
