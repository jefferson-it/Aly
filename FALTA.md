# Aly-lang — Lista de Pendências

Análise do que está faltando ou incompleto na linguagem Aly v0.1.0.

---

## 🔴 Crítico

### 1. Zero testes automatizados ❌
**STATUS:** PARCIALMENTE IMPLEMENTADO
- Queda: Não existe diretório `tests/` com Rust tests
- Contudo: Existem 48 arquivos `.aly` de teste em `__tests__/`
- Problema: Sem estrutura formal de testes Rust para validar funcionalidades do interpretador

### 2. `try`/`catch` quebrado ❌
**STATUS:** IMPLEMENTADO INCORRETAMENTE
- Problema: Usa `panic::catch_unwind` que só captura panics do Rust, **não erros da linguagem Aly**
- Sintoma: `catch` recebe mensagem de panic como string sem estrutura
- Localização: `src/native/mod.rs:73-75` (`catch_error` é vazio)

### 3. Sem `throw`/`panic` para o usuário ❌
**STATUS:** COMPLETAMENTE AUSENTE
- Problema: Token `throw` não implementado, usuários não podem lançar erros programaticamente
- Localização: Token em `src/tokens.rs:63` mas não implementado no interpretador

### 4. 100+ `panic!()` no código ❌
**STATUS:** IMPLEMENTADO INCORRETAMENTE
- Problema: Todo erro de sintaxe/runtime derruba interpretador, sem recuperação
- Sintoma: Sem programabilidade, sem tratamento gracioso de erros

### 5. Estado global mutável com `unsafe` ⚠️
**STATUS:** IMPLEMENTADO
- Problema: Singleton global via `unsafe { &mut RUNTIME }`
- Impacto: Thread-unsafe, impede execução paralela
- Localização: `src/aly.rs:17-23`

### 6. Matemática via crate `eval` (JavaScript) ⚠️
**STATUS:** IMPLEMENTADO
- Problema: Toda aritmética passa por crate `eval`, avaliação como JavaScript
- Impacto: Números IEEE 754, inteiros grandes perdem precisão, overflow silencioso
- Localização: `src/native/mod.rs:26-31`

### 7. Comparação de tipos via string ✅
**STATUS:** IMPLEMENTADO
- Funcionalidade: Tipos comparados via `Display` trait
- Localização: `src/native/vars.rs:53`

---

## 🟠 Grave

### 8. Sem `break`/`continue` em loops ❌
**STATUS:** COMPLETAMENTE AUSENTE
- Problema: Token `break`/`continue` definido mas não suportado no interpretador
- Localização: `src/runtime/interpreter.rs:180-357` (loop implementation sem break/continue)

### 9. `async`/`await` sintético (não é concorrente) ⚠️
**STATUS:** IMPLEMENTADO LIMITADO
- Funcionalidade: `async` wrappa em `ValueData::Promise`, `await` desembrulha
- Limitação: Sem event loop, I/O não-bloqueante, concorrência real

### 10. Tokens `struct`/`model` mortos ❌
**STATUS:** TOKENS DEFINIDOS MAS SEM IMPLEMENTAÇÃO
- Problema: Tokens definidos em `src/tokens.rs:35-36` mas sem lógica de parsing/execução

### 11. Compilação não compila de verdade ✅
**STATUS:** DOCUMENTADO/CORRETO
- Funcionalidade: `aly comp` embute script em binário do interpretador
- Limitação: Não há LLVM, transpilação ou geração de código nativo (documentado)

### 12. Módulos sem documentação ⚠️
**STATUS:** DOCUMENTADOS (mas não em doc/)
- Problema: Módulos implementados mas documentação não em `doc/`
- Documentação: Existe em fonte, mas não documentada em `doc/`

### 13. Sem `for..in` ou loops por iteração ❌
**STATUS:** COMPLETAMENTE AUSENTE
- Problema: Não é possível iterar sobre vetores, objetos ou ranges
- loop range x..y {}
- loop x in arr {}
- loop x of arr {}

### 14. Argumentos nativos via string separada por vírgula ⚠️
**STATUS:** IMPLEMENTADO COM PROBLEMAS
- Funcionalidade: Funções nativas recebem como string única separada por vírgula
- Problema: Quebra com vírgulas em strings/expressões aninhadas
- Localização: `src/native/std.rs:17-46` (split_args)

### 15. Sem AST (Abstract Syntax Tree) ❌
**STATUS:** COMPLETAMENTE AUSENTE
- Problema: Interpretador opera diretamente em streams de token, sem AST
- Impacto: Impede otimizações, análise estática, compilação multi-passo

---

## 🟡 Moderado

### 16. Operadores de atribuição compostos ❌
**STATUS:** COMPLETAMENTE AUSENTE
- Problema: Faltam `+=`, `-=`, `*=`, `/=`, `%=`. Só `++` e `--` existem

### 17. GUI limitada ⚠️
**STATUS:** IMPLEMENTADA (limitada)
- Funcionalidade: 8 widgets (Window, Button, Label, etc.)
- Limitação: Único evento `onClick`, CSS suporta só 4 propriedades

### 18. REPL limitado ✅
**STATUS:** FUNCIONAL COMPLETO
- Funcionalidade: Multi-linha, histórico, autocomplete, syntax highlight implementados

### 19. Math ✅
**STATUS:** COMPLETO
- Funcionalidade: Todas funções trigonométricas, logaritmos, abs, min, max, constantes implementadas

### 20. Faltam funções na stdlib ✅
**STATUS:** COMPLETO EXPONENCIAL
- Funcionalidades: JSON, regex, base64, cripto, UUID, gzip, CSV, timer, console, sockets implementadas

### 21. Mensagens de erro misturam português e inglês ⚠️
**STATUS:** PARCIALMENTE RESOLVIDO
- Problema: Algumas mensagens misturam português e inglês

---

## 🟄 Leve

### 22. Operador ternário ❌
**STATUS:** COMPLETAMENTE AUSENTE
- Problema: `x ? y : z` não implementado

### 23. Short-circuit `&&`/`||` ❌
**STATUS:** IMPLEMENTADO INCORRETAMENTE
- Problema: Usa `and`/`or` que sempre avalia ambos os lados (via eval JS)

### 24. Semântica de `;` ⚠️
**STATUS:** DOCUMENTADA
- Documentado: Semicolons são para cabeçalhos de loop, não terminadores de statement

### 25. Sem continuação de linha `\` ⚠️
**STATUS:** DOCUMENTADO
- Documentado: Todo statement precisa caber em uma linha (a menos que contenha blocos)

### 26. Código morto ✅
**STATUS:** IMPLEMENTADO
- `Object.entries()` retorna pares chave-valor como vetor de vetores

---

## 🛠 Ferramentas Ausentes

- **LSP** — sem suporte a IDE
- **Formatador** — `aly fmt`
- **Linter** — `aly lint`
- **Debugger** — breakpoints, step-through
- **Gerenciador de pacotes** — `aly install`, `aly publish`
- **Exemplos** — sem diretório `examples/`
- **Tutorial** — sem guia passo a passo

---

## 📊 Prioridades Sugeridas

1. Testes automatizados (base para tudo)
2. Consertar `try`/`catch` (error handling funcional)
3. Adicionar `throw` para o usuário
4. Adicionar `break`/`continue`
5. Substituir `eval` crate por avaliação matemática nativa em Rust
6. Documentar módulos órfãos
7. Implementar `struct`/`model` (ou remover tokens mortos)
8. Adicionar `for..in`
9. Implementar `json.parse` / `json.stringify`
10. Adicionar funções matemáticas faltantes