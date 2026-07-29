# Otimização do Compilador no Aly

O compilador do Aly inclui passagens de otimização tanto para o compilador AOT como para a JIT VM.

---

## 1. Passagens de Otimização

O compilador executa uma série de passagens de otimização na MIR (Mid-level Intermediate Representation):

- **Constant Folding**: Avalia expressões constantes em tempo de compilação (`const_eval`)
- **Dead Code Elimination**: Remove código inalcançável
- **Inlining**: Substitui chamadas de função pelo corpo da função quando benéfico
- **Strength Reduction**: Substitui operações dispendiosas por outras mais baratas

---

## 2. Compilação Incremental

```bash
alyc -o output input.aly --incremental
```

Armazena em cache módulos previamente compilados e apenas recompila os ficheiros alterados:

```aly
# Cache armazenada no diretório .aly_cache/
# Deteta automaticamente alterações através das datas de modificação e hashes dos ficheiros
```

---

## 3. Compilação Paralela

```bash
alyc -o output input.aly --parallel
```

Compila módulos independentes em paralelo utilizando múltiplas threads, reduzindo o tempo total de compilação para projetos com vários módulos.

---

## 4. Otimizações JIT

O JIT da VM (`jit.rs`) otimiza caminhos de código frequentes:

- **Inline Caching**: Armazena em cache os resultados da procura de métodos (`inline_cache.rs`)
- **Fast Call Convention**: Passagem de argumentos otimizada (`fastcall.rs`, `call_conventions.rs`)
- **Optimized VM**: Execução de bytecode otimizada (`optimized.rs`)
