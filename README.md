# ⚡ Aly Language (`aly`)

[![Rust](https://img.shields.io/badge/language-Rust-orange.svg)](https://www.rust-lang.org/)
[![License](https://img.shields.io/badge/license-MIT-blue.svg)](LICENSE)
[![Status](https://img.shields.io/badge/status-active--development-green.svg)]()

> **Aly** é uma linguagem de programação moderna, expressiva e tipada dinamicamente, construída em Rust. Projetada para scripting, automação de sistemas, prototipagem rápida e desenvolvimento de interfaces gráficas (GUI).

---

## ✨ Características Principais

* **Sintaxe Limpa e Expressiva:** Declaração intuitiva com `let`, `const`, `fun`, `loop`, `if` / `elif` / `else`.
* **Tipagem Dinâmica Flexível:** Suporte nativo para `int`, `float`, `string`, `bool`, `vector`, `object` e `function`.
* **Módulos Nativos Integrados:** Biblioteca padrão completa incluindo `fs`, `http`, `json`, `crypto`, `regex`, `datetime`, `uuid`, `gzip`, `csv`, `net`, `math` e `gui`.
* **Gerenciador de Pacotes APG (`apg`):** Gerenciamento de dependências, resolução de registros e ecossistema de pacotes integrado.
* **REPL Interativo:** Console interativo com suporte multi-linha, histórico e destaque de sintaxe.
* **Interface Gráfica Nativa (GUI):** Criação de janelas e widgets nativos via integrações de interface.
* **Compilação de Scripts:** Capacidade de empacotar scripts `.aly` em binários executáveis.

---

## 💻 Exemplo Rápido

```aly
# Exemplo em Aly: Declaração, Funções e Loops

let message = "Bem-vindo à Aly Language!"
print(message)

# Função personalizada
fun calcular_soma(a, b) {
    return a + b
}

let resultado = calcular_soma(10, 20)
print("Resultado da soma: " + resultado)

# Estrutura de Loop
let i = 0
loop i lt 3 {
    print("Iteração " + i)
    i = i + 1
}
```

---

## 🚀 Instalação e Compilação

Para compilar a linguagem Aly a partir do código fonte:

```bash
# Clone o repositório
git clone https://github.com/jefferson-it/aly.git
cd aly

# Compilar em modo release
cargo build --release

# O binário estará disponível em target/release/aly
```

---

## 🛠️ Modos de Uso

```bash
# Executar um script Aly
aly run script.aly

# Iniciar o REPL interativo
aly

# Compilar um script para binário
aly comp script.aly

# Gerenciar pacotes com APG
apg install <nome_do_pacote>
```

---

## 📚 Documentação e Exemplos

* **Documentação dos Módulos:** Consulte o diretório [`doc/`](doc/) para guias detalhados da biblioteca padrão.
* **Exemplos Práticos:** Explore o diretório [`examples/`](examples/) para scripts demonstrativos.

---

## 📄 Licença

Distribuído sob a licença **MIT**. Veja [`LICENSE`](LICENSE) para mais detalhes.
