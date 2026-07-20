# Aly-lang

Uma linguagem de programação expressiva, tipada dinamicamente, construída em Rust.

## Sobre
Aly é uma linguagem de programação moderna com sintaxe limpa, projetada para
scripting, automação e prototipagem rápida.

### Características
- **Sintaxe simples e expressiva**: `let`, `const`, `fun`, `loop`, `if`/`elif`/`else`
- **Tipagem dinâmica**: int, float, string, bool, vector, object, function
- **Módulos nativos**: fs, http, json, crypto, regex, datetime, uuid, gzip, csv, net, math, gui
- **Async/Await**: Concorrência integrada
- **Package Manager**: `apg` para gerenciamento de pacotes
- **Compiler**: Geração de código C compilável com GCC
- **GUI nativa**: Widgets via FLTK
- **REPL interativo**: Multi-linha, histórico, syntax highlight

## Exemplo Rápido
```aly
# Hello World
let message = "Olá, mundo!"
print(message)

# Loop
let i = 0
loop i lt 5 {
    print(i)
    i = i + 1
}

# Função
fun add(a, b) {
    return a + b
}
```

## Instalação
```bash
git clone https://github.com/jefferson-it/aly
cd aly
cargo build --release
./target/release/aly
```

## Uso
```bash
# Executar um arquivo
aly run script.aly

# REPL interativo
aly

# Compilar para binário
aly comp script.aly

# Gerenciar pacotes
apg install <pacote>
```

## Documentação
Veja a documentação completa em `doc/` e exemplos em `examples/`.

## Licença
MIT
