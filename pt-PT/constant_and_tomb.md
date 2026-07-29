# Constantes e o Mecanismo de Congelamento `tomb` no Aly

O Aly suporta tanto constantes tradicionais em tempo de compilação como o congelamento dinâmico de mutabilidade em tempo de execução.

---

## 1. Constantes (`const`)

As constantes são declaradas em tempo de compilação e não podem ser reatribuídas ou modificadas.

```aly
const MAX_LIMIT = 100
```

---

## 2. Congelamento Dinâmico (`tomb`)

A função nativa `tomb(&variable)` bloqueia uma variável mutável, convertendo-a numa constante em tempo de execução. Quaisquer tentativas subsequentes de modificar ou reatribuir a variável irão falhar.

```aly
let name = "Pedro"

# Nesta fase, name pode ser modificado
name = "Peter" 

# Congelar a variável 'name'
tomb(&name)

# Tentar modificar 'name' agora irá desencadear um erro
# name = "John" # Error: Reassignment to frozen variable
```

Esta funcionalidade é útil para garantir a segurança dos dados, definir configurações de tempo de execução que não devem ser alteradas após a configuração e bloquear estados.
