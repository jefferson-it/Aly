# Aly-lang — Todo List Prioritizado

## 🔴 Crítico (Fundação da Linguagem)

1. [ ] **Testes automatizados Rust** — Criar diretório `tests/` com testes unitários/integração Rust para validar interpretador (base para tudo)
2. [ ] **Consertar `try`/`catch`** — Substituir `panic::catch_unwind` por error handling nativo da linguagem Aly (não panic do Rust) — `src/native/mod.rs:73-75`
3. [ ] **Implementar `throw`/`panic` para usuário** — Token `throw` existe em `src/tokens.rs:63` mas sem implementação no interpretador
4. [ ] **Eliminar 100+ `panic!()`** — Substituir panics por erro recuperável (Result/Error type) em todo interpretador
5. [ ] **Remover estado global `unsafe`** — Substituir `unsafe { &mut RUNTIME }` em `src/aly.rs:17-23` por estado thread-safe
6. [ ] **Substituir crate `eval` (JS) por math nativo Rust** — Aritmética nativa Rust para precisão IEEE 754 correta, inteiros grandes, overflow controlado — `src/native/mod.rs:26-31`

## 🟠 Grave (Funcionalidades Core Faltando)

7. [ ] **Implementar `break`/`continue` em loops** — Tokens existem, interpretador em `src/runtime/interpreter.rs:180-357` ignora
8. [ ] **Implementar `for..in` / `for..of` / range loops** — Iteração sobre vetores, objetos, ranges (`x..y`, `x in arr`, `x of arr`)
3. [ ] **Implementar `throw` para o usuário** — Token `throw` existe em `src/tokens.rs:63` mas sem implementação no interpretador
9. [ ] **Implementar `struct`/`model`** — Tokens definidos em `src/tokens.rs:35-36` mas sem parsing/execução
10. [ ] **Implementar AST (Abstract Syntax Tree)** — Interpretador opera direto em token stream; AST necessário para otimizações, análise estática, compilação multi-pass
11. [ ] **Corrigir argumentos nativos via string CSV** — `split_args` em `src/native/std.rs:17-46` quebra com vírgulas em strings/expressões aninhadas
12. [ ] **Documentar módulos órfãos** — Documentar módulos implementados em `doc/`

## 🟡 Moderado (Qualidade e Ergonomia)

13. [ ] **Operadores de atribuição compostos** — Adicionar `+=`, `-=`, `*=`, `/=`, `%=` (só `++`/`--` existem)
14. [ ] **Corrigir short-circuit `&&`/`||`** — Atualmente usa `and`/`or` que avalia ambos lados via eval JS
15. [ ] **Operador ternário `x ? y : z`** — Completamente ausente
14. [ ] **Melhorar GUI** — Expandir eventos além de `onClick`, expandir CSS além de 4 propriedades
15. [ ] **Implementar `json.parse` / `json.stringify`** — Listado como prioridade sugerida
15. [ ] **Corrigir mensagens de erro PT/EN misturados** — Padronizar idioma das mensagens de erro
16. [ ] **Melhorar `async`/`await`** — Adicionar event loop real, I/O não-bloqueante, concorrência real

## 🟢 Leve (Syntax Sugar / Qualidade de Vida)

17. [ ] **Operador ternário `x ? y : z`** — Completamente ausente
18. [ ] **Continuação de linha `\`** — Statements devem caber em uma linha (exceto blocos)
19. [ ] **Semântica de `;`** — Documentar/validar: ponto-e-vírgula só para cabeçalhos de loop, não terminadores

## 🛠 Ferramentas Ausentes (Ecossistema)

20. [ ] **LSP** — Language Server Protocol para suporte a IDE
21. [ ] **Formatador** — `aly fmt`
22. [ ] **Linter** — `aly lint`
23. [ ] **Debugger** — Breakpoints, step-through
24. [ ] **Gerenciador de pacotes** — `aly install`, `aly publish`
25. [ ] **Diretório `examples/`** — Exemplos de uso da linguagem
26. [ ] **Tutorial** — Guia passo a passo para iniciantes

---

## 📋 Ordem de Execução Sugerida (Top 10)

1. **Testes Rust** → Base para validar tudo
2. **`try`/`catch` correto** → Error handling funcional
3. **`throw` do usuário** → Error handling programável
4. **Remover `panic!()`** → Estabilidade do runtime
5. **Remover `unsafe` global** → Thread-safety
6. **Math nativo Rust** → Precisão numérica correta
7. **`break`/`continue`** → Controle de fluxo básico
8. **`for..in` / ranges** → Iteração essencial
9. **AST** → Base para compiler/otimizações futuras
10. **`struct`/`model`** → Tipos de dados estruturados