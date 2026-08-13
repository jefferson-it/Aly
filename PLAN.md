# PLAN.md — Planejamento de Features do Aly

> Documento de planejamento. Nada aqui é "feito" até ser marcado como tal.

---

## 1. Ponteiros Reais + Tipos Fixos (Runtime/Interpreter)

**Status: implementação em andamento** (types.rs, memory.rs, vars.rs, native/mod.rs, aly.rs, interpreter.rs já alterados; build ainda não validado)

### Modelo
Heap de células em Rust seguro (sem `unsafe`): `HashMap<usize, Rc<RefCell<ValueData>>>` thread-local — análogo ao projeto Python (`Memory.alocacoes` + `Cell`), com endereço `usize` opaco (0 = nulo). Acesso a endereço inexistente (pós-`free`) → `RuntimeError` "ponteiro pendurado".

### Sintaxe
- `<&x>` → endereço real de `x` (equivalente ao `&x` de C)
- `<*p>` → desreferência (equivalente ao `*p` de C)
- `<*p> = v` → atribuição através do ponteiro (como `*p = v` em C)
- Builtins: `addr`, `deref`, `store`, `alloc`, `alloc_typed`, `free`, `is_null`, `ptr_type`, `tomb` (migrado)

### Tipos fixos
- `let x : i8 = 5` — anotação de tipo com `:`
- `i8, i16, i32, i64, u8, u16, u32, u64, f32, f64` via variante única `ValueData::FixedInt`
- Coação com checagem de overflow (`TypeError` em vez de wrap)
- `type` keyword expandido: `i8..f64`, `ptr`, `ptr<int>`

### Etapas
1. Tipos novos em `native/types.rs` (FixedIntTy, FixedRepr, Type::Fixed/Pointer, ValueData::FixedInt/Pointer, coerce) — **feito**
2. Heap em `runtime/memory.rs` (novo, thread_local) — **feito**
3. Sintaxe `<&x>`/`<*p>`/`<*p>=v` (normalize_pointer_blocks + process_value + deref_assign no exec) — **feito**
4. Write-through no Var (`vars.rs`) + boxing via `addr_of` — **feito**
5. Builtins NativeData + dispatch no `function_run` — **feito**
6. `let x : i8` + `type` expandido — **feito**
7. Testes: `__tests__/pointer.*` e `typed.*` — **pendente**

### Limitações v1
- Sem aritmética de ponteiro (`<*p> + 1`) — caminho math passa por `eval_math` (f64)
- Compilador AOT + VM fora de escopo
- Alvo de `<*p>`: token único

---

## 2. Sistema de Módulos — Novos Requisitos

**Status: planejado (não iniciado)**

### 2.1 Cabeçalho de módulo estilo Go (`package`)

- Declaração obrigatória no topo do arquivo, estilo Go:

```aly
package meu_modulo

import "outro_modulo"
```

- Sintaxe: `package <nome>` na primeira linha do arquivo.
- O nome do módulo pode ser simples (`package fs`) ou com namespace (`package aly.io` — validar se usaremos `.` ou `:`).

### 2.2 Multiplicação implícita com parênteses

- `1abc` vira **`(1 * abc)`** — com parênteses, não `1 * abc` solto:

```aly
2x          # (2 * x)
3pi + 1     # (3 * pi) + 1
5(2 + 3)    # decidir: suportar número antes de parêntese? (5 * (2 + 3))
```

- Os parênteses garantem precedência correta: `2x^2` → `(2 * x)^2` e não `2 * x^2`.
- Regra de tokenização: dígito diretamente seguido de identificador (sem espaço) → implícito.
- Local provável: `letter_per_letter` (runtime/parser.rs) e/ou `compiler/parser.rs`.

### 2.3 Import por NOME do módulo (não pelo nome do arquivo)

- `import "fs"` procura pelo **nome declarado no `package`** do módulo, não pelo nome do arquivo físico.
- Resolução:
  1. Procurar o nome do módulo no diretório atual e subdiretórios (manifest.json ajuda)
  2. Procurar em `modules/` ou equivalente
  3. Erro claro se não encontrar: `ModuleError: módulo 'fs' não encontrado`
- Precisa de um índice de módulos (nome → caminho do arquivo).

### 2.4 Regras de criação de módulos: `internal:` reservado

- O usuário pode criar um módulo chamado `fs`, `io`, `math`, etc. — **sem restrição**.
- O namespace `internal:` é **reservado para builtins** do Aly:
  - `internal:fs` → proibido criar; só o runtime registra
  - Tentar `package internal:fs` → `ModuleError: namespace 'internal:' é reservado`
- Comparação por prefixo: nomes que começam com `internal:` são bloqueados para o usuário.

### 2.5 `aly init <nome?>` — gerador de projeto (estilo npm)

- Novo comando CLI: `aly init` ou `aly init meu_projeto`
- Gera `manifest.json` (análogo ao `package.json`):

```json
{
  "name": "meu_projeto",
  "version": "0.1.0",
  "main_module": "main",
  "dependencies": {}
}
```

- `main_module` → nome do módulo principal; **padrão `"main"`**; o usuário pode renomear (`aly init meu_app --main mod_principal` ou editando o manifest).
- Estrutura gerada sugerida:
  - `manifest.json`
  - `main.aly` (com `package main`)
  - `modules/` (diretório para módulos locais)
- Se o diretório não existe, `aly init meu_projeto` cria a pasta; `aly init` usa o diretório atual.

---

## 3. JOT como Dependência e Binário da Linguagem

**Status: planejado (não iniciado)** — pedido do usuário, apenas anotado

### Requisito
Usar [https://github.com/jefferson-it/JOT](https://github.com/jefferson-it/JOT) como:
1. **Dependência da linguagem** (jot-core, jot-parser, jot-serializer — o Cargo.toml já tem essas deps comentadas, apontando para `/home/jefferson/Desktop/projects/JOT/jot/{core,parser,serializer}`)
2. **Binário pertencente à linguagem**: `cargo build --bin jot` deve funcionar nesta raiz (assim como `cargo build --bin aly`)

### Estrutura do JOT (investigada)
- Repositório: workspace Rust em `jot/` com crates:
  - `core/` — tipos `JotValue`, `JotError`
  - `parser/` — lexer, parser, tree builder
  - `serializer/` — conversores JSON/YAML/TOML etc.
  - `cli/` — **binário** `jot` (comandos: `read`, `pick`, `conv`, `run`/REPL)
  - `native/` — binding napi-rs Node.js
  - `python/` — binding PyO3
- JOT **não está clonado** nesta máquina ainda (`~/Desktop/projects/JOT` não existe)
- Build do próprio repo JOT: `cd jot && cargo build --bin jot`

### Caminhos possíveis (decidir depois)
- Clonar JOT para dentro desta raiz (ex.: `jot/` no workspace do Aly) e ajustar as path deps para `./jot/core` etc. → `cargo build --bin jot` funciona daqui
- Ou git dependency no Cargo.toml (mas bin de git dep **não** vira `cargo build --bin jot` nesta raiz — requisito pede binário local, então clonar/vendorizar parece o caminho)
- Restaurar as deps comentadas no Cargo.toml e o recurso `jot-embed`

### Pendências de integração
- Como o Aly usa JOT: embed (jot-embed) e/ou external (jot_external.rs)
- `src/runtime/jot.rs`, `jot_embed.rs`, `jot_external.rs` já existem no repo — falta ligar com as crates reais
- Renomear/ajustar binário `jot` para não conflitar com o crate `jot` interno
