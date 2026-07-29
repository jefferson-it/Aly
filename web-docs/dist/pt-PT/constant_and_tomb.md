# Constantes e o Mecanismo de Congelamento `tomb` em Aly

Aly suporta constantes tradicionais de tempo de compilação, bem como congelamento de mutabilidade dinâmica em tempo de execução.

---

## 1. Constantes (`const`)

As constantes são declaradas em tempo de compilação e não podem ser reassociadas nem mutadas.

```aly
const MAX_LIMIT = 100
```

---

## 2. Congelamento Dinâmico (`tomb`)

A função nativa `tomb(&variable)` bloqueia uma variável mutável, convertendo-a numa constante em tempo de execução. Quaisquer tentativas subsequentes de mutar ou reassociar a variável falharão.

```aly
let name = "Pedro"

# Neste estágio, name pode ser mutado
name = "Peter" 

# Congelar a variável 'name'
tomb(&name)

# Tentar mutar 'name' agora vai provocar um erro
# name = "John" # Erro: Reatribuição a variável congelada
```

Este recurso é útil para garantir a segurança dos dados, definir configurações de tempo de execução que não devem mudar após a configuração, e bloquear estados.
