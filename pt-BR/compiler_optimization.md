# Otimização do Compilador no Aly

O compilador do Aly inclui passes de otimização tanto para o compilador AOT quanto para a VM JIT.

---

## 1. Passes de Otimização

O compilador executa uma série de passes de otimização na MIR (Mid-level Intermediate Representation):

- **Constant Folding**: Avalia expressões constantes em tempo de compilação (`const_eval`)
- **Dead Code Elimination**: Remove código inacessível
- **Inlining**: Substitui chamadas de função pelo corpo da função quando vantajoso
- **Strength Reduction**: Substitui operações custosas por opções mais baratas

---

## 2. Compilação Incremental

```bash
alyc -o output input.aly --incremental
```

Armazena em cache módulos previamente compilados e recompila apenas os arquivos alterados:

```aly
# Cache armazenado no diretório .aly_cache/
# Detecta automaticamente alterações por meio de timestamps e hashes de arquivo
```

---

## 3. Compilação Paralela

```bash
alyc -o output input.aly --parallel
```

Compila módulos independentes em paralelo usando múltiplas threads, reduzindo o tempo total de build para projetos multi-módulo.

---

## 4. Otimizações JIT

O JIT da VM (`jit.rs`) otimiza caminhos de código frequentes (hot code paths):

- **Inline Caching**: Armazena em cache os resultados de pesquisa de método (`inline_cache.rs`)
- **Fast Call Convention**: Passagem de argumentos otimizada (`fastcall.rs`, `call_conventions.rs`)
- **Optimized VM**: Execução de bytecode ajustada (`optimized.rs`)