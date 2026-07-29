# Optimização do Compilador em Aly

O compilador Aly inclui passes de optimização tanto para o compilador AOT como para a VM JIT.

---

## 1. Passes de Optimização

O compilador executa uma série de passes de optimização no MIR (Mid-level Intermediate Representation):

- **Constant Folding**: Avalia expressões constantes em tempo de compilação (`const_eval`)
- **Dead Code Elimination**: Remove código inalcançável
- **Inlining**: Substitui chamadas de função pelo corpo da função quando benéfico
- **Strength Reduction**: Substitui operações dispendiosas por operações mais baratas

---

## 2. Compilação Incremental

```bash
alyc -o output input.aly --incremental
```

Guarda em cache módulos previamente compilados e apenas recompila ficheiros alterados:

```aly
# Cache guardado no directório .aly_cache/
# Detecta automaticamente alterações através de carimbos de data e hashes de ficheiros
```

---

## 3. Compilação Paralela

```bash
alyc -o output input.aly --parallel
```

Compila módulos independentes em paralelo usando múltiplos threads, reduzindo o tempo total de compilação para projetos com múltiplos módulos.

---

## 4. Optimizações JIT

A VM JIT (`jit.rs`) optimiza caminhos de código quentes:

- **Inline Caching**: Guarda resultados de pesquisa de métodos (`inline_cache.rs`)
- **Fast Call Convention**: Passagem de argumentos optimizada (`fastcall.rs`, `call_conventions.rs`)
- **VM Optimizado**: Execução de bytecode afinada (`optimized.rs`)
