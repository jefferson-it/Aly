# ═══════════════════════════════════════════════════════════════════════════════
# Aly Language - Makefile
# ═══════════════════════════════════════════════════════════════════════════════

# ── Configuração ──────────────────────────────────────────────────────────────
PKG_NAME    := Aly
VERSION     := 0.1.0
CARGO       := cargo
RUST_FLAGS  :=

# Binários
BIN_ALY     := target/release/aly
BIN_APG     := target/release/apg
BIN_DEBUG   := target/debug/aly

# Diretórios
SRC_DIR     := src
TEST_DIR    := tests
TEST_ALY    := __tests__
BENCH_DIR   := benchmarks
EDITOR_DIR  := editor
DOC_DIR     := doc
EXAMPLE_DIR := examples

# ── Cores ─────────────────────────────────────────────────────────────────────
GREEN  := \033[0;32m
CYAN   := \033[0;36m
YELLOW := \033[1;33m
RED    := \033[0;31m
BOLD   := \033[1m
DIM    := \033[2m
RESET  := \033[0m

# ═══════════════════════════════════════════════════════════════════════════════
# Targets principais
# ═══════════════════════════════════════════════════════════════════════════════

.PHONY: all build build-debug release clean install uninstall help

## Compila todos os binários (release)
all: build

## Build release (otimizado)
build:
	@echo -e "$(CYAN)$(BOLD)▸ Compilando $(PKG_NAME) (release)...$(RESET)"
	$(CARGO) build --release $(RUST_FLAGS)
	@echo -e "$(GREEN)✓ Build release concluído$(RESET)"
	@echo -e "$(DIM)  Binários: $(BIN_ALY), $(BIN_APG)$(RESET)"

## Build debug (para desenvolvimento)
build-debug:
	@echo -e "$(CYAN)$(BOLD)▸ Compilando $(PKG_NAME) (debug)...$(RESET)"
	$(CARGO) build
	@echo -e "$(GREEN)✓ Build debug concluído$(RESET)"

## Alias para release
release: build

## Limpa artefatos de build
clean:
	@echo -e "$(CYAN)$(BOLD)▸ Limpando artefatos...$(RESET)"
	$(CARGO) clean
	@rm -f $(BENCH_DIR)/bench_prime_cpp
	@rm -f $(BENCH_DIR)/bench_prime_rs
	@rm -f $(BENCH_DIR)/bench_prime_go
	@rm -f $(BENCH_DIR)/bench_prime_cs
	@rm -f $(BENCH_DIR)/bench_prime
	@rm -f $(BENCH_DIR)/bench_prime.s
	@rm -f $(BENCH_DIR)/bench_prime_aly_tmp.*
	@rm -f $(BENCH_DIR)/runtime_aly.h
	@rm -f /tmp/aly-lang-0.1.0.vsix
	@echo -e "$(GREEN)✓ Limpeza concluída$(RESET)"

## Instala os binários no sistema
install: build
	@echo -e "$(CYAN)$(BOLD)▸ Instalando $(PKG_NAME)...$(RESET)"
	install -Dm755 $(BIN_ALY)  /usr/local/bin/aly
	install -Dm755 $(BIN_APG)  /usr/local/bin/apg
	@echo -e "$(GREEN)✓ Instalado em /usr/local/bin/{aly, apg}$(RESET)"

## Remove os binários do sistema
uninstall:
	@echo -e "$(CYAN)$(BOLD)▸ Removendo $(PKG_NAME)...$(RESET)"
	rm -f /usr/local/bin/aly
	rm -f /usr/local/bin/apg
	@echo -e "$(GREEN)✓ Removido$(RESET)"

# ═══════════════════════════════════════════════════════════════════════════════
# Testes
# ═══════════════════════════════════════════════════════════════════════════════

.PHONY: test test-unit test-integration test-all test-aly test-quick check fmt fmt-check clippy

## Roda todos os testes Rust (1 thread p/ evitar travamento)
test:
	@echo -e "$(CYAN)$(BOLD)▸ Rodando testes Rust...$(RESET)"
	$(CARGO) test -- --test-threads=1
	@echo -e "$(GREEN)✓ Todos os testes passaram$(RESET)"

## Roda apenas testes unitários
test-unit:
	@echo -e "$(CYAN)$(BOLD)▸ Rodando testes unitários...$(RESET)"
	$(CARGO) test -- --test-threads=1 unit
	@echo -e "$(GREEN)✓ Testes unitários passaram$(RESET)"

## Roda apenas testes de integração
test-integration:
	@echo -e "$(CYAN)$(BOLD)▸ Rodando testes de integração...$(RESET)"
	$(CARGO) test -- --test-threads=1 integration
	@echo -e "$(GREEN)✓ Testes de integração passaram$(RESET)"

## Roda todos os testes Rust + Aly
test-all: test test-aly

## Roda os scripts de teste .aly
test-aly: build
	@echo -e "$(CYAN)$(BOLD)▸ Rodando scripts de teste .aly...$(RESET)"
	@FAIL=0; \
	for f in $(TEST_ALY)/*.aly; do \
		echo -e "$(DIM)  → $$f$(RESET)"; \
		$(BIN_ALY) run "$$f" 2>&1 || FAIL=1; \
	done; \
	for f in $(TEST_ALY)/**/*.aly; do \
		echo -e "$(DIM)  → $$f$(RESET)"; \
		$(BIN_ALY) run "$$f" 2>&1 || FAIL=1; \
	done; \
	if [ $$FAIL -eq 0 ]; then \
		echo -e "$(GREEN)✓ Todos os testes .aly passaram$(RESET)"; \
	else \
		echo -e "$(RED)✗ Alguns testes falharam$(RESET)"; \
		exit 1; \
	fi

## Teste rápido (compila + roda um arquivo .aly)
test-quick: build
	@echo -e "$(CYAN)$(BOLD)▸ Teste rápido$(RESET)"
	$(BIN_ALY) run test.aly

## Verifica código (lint + types)
check:
	@echo -e "$(CYAN)$(BOLD)▸ Verificando código...$(RESET)"
	$(CARGO) check
	@echo -e "$(GREEN)✓ Código verificado$(RESET)"

## Formata código
fmt:
	@echo -e "$(CYAN)$(BOLD)▸ Formatando código...$(RESET)"
	$(CARGO) fmt
	@echo -e "$(GREEN)✓ Código formatado$(RESET)"

## Verifica formatação (CI)
fmt-check:
	$(CARGO) fmt -- --check

## Roda clippy (lint do Rust)
clippy:
	@echo -e "$(CYAN)$(BOLD)▸ Rodando clippy...$(RESET)"
	$(CARGO) clippy -- -D warnings
	@echo -e "$(GREEN)✓ Clippy limpo$(RESET)"

# ═══════════════════════════════════════════════════════════════════════════════
# Benchmarks
# ═══════════════════════════════════════════════════════════════════════════════

.PHONY: bench bench-prime bench-array

## Roda benchmarks completos
bench: build
	@echo -e "$(CYAN)$(BOLD)▸ Rodando benchmarks...$(RESET)"
	bash $(BENCH_DIR)/run_benchmarks.sh

## Benchmark de primos
bench-prime: build
	@echo -e "$(CYAN)$(BOLD)▸ Benchmark de primos$(RESET)"
	bash $(BENCH_DIR)/run_benchmarks.sh 200000 5

## Benchmark de arrays
bench-array: build
	@echo -e "$(CYAN)$(BOLD)▸ Benchmark de arrays$(RESET)"
	bash $(BENCH_DIR)/quick_bench.sh

# ═══════════════════════════════════════════════════════════════════════════════
# Editor / Extensões
# ═══════════════════════════════════════════════════════════════════════════════

.PHONY: editor-install editor-build editor-uninstall

## Instala extensão no editor (menu interativo)
editor-install:
	@bash $(EDITOR_DIR)/install.sh

## Empacula e instala extensão no VS Code
editor-build:
	@bash $(EDITOR_DIR)/build_extension.sh

## Remove extensão do VS Code
editor-uninstall:
	@echo -e "$(CYAN)$(BOLD)▸ Removendo extensão do VS Code...$(RESET)"
	@code --uninstall-extension "jefferson-it.aly-lang" 2>/dev/null && \
		echo -e "$(GREEN)✓ Extensão removida$(RESET)" || \
		echo -e "$(YELLOW)⚠ Extensão não encontrada$(RESET)"

# ═══════════════════════════════════════════════════════════════════════════════
# REPL / Execução
# ═══════════════════════════════════════════════════════════════════════════════

.PHONY: repl run run-vm comp help-aly

## Inicia o REPL do Aly
repl: build
	@$(BIN_ALY)

## Roda um arquivo .aly (USE: make run FILE=examples/hello.aly)
run: build
	@if [ -z "$(FILE)" ]; then \
		echo -e "$(RED)Uso: make run FILE=caminho/para/arquivo.aly$(RESET)"; \
		exit 1; \
	fi
	@$(BIN_ALY) run $(FILE)

## Roda via VM (bytecode)
run-vm: build
	@if [ -z "$(FILE)" ]; then \
		echo -e "$(RED)Uso: make run-vm FILE=caminho/para/arquivo.aly$(RESET)"; \
		exit 1; \
	fi
	@$(BIN_ALY) --vm run $(FILE)

## Compila um arquivo .aly para binário nativo (USE: make comp FILE=examples/hello.aly)
comp: build
	@if [ -z "$(FILE)" ]; then \
		echo -e "$(RED)Uso: make comp FILE=caminho/para/arquivo.aly$(RESET)"; \
		exit 1; \
	fi
	@$(BIN_ALY) comp $(FILE)

## Mostra ajuda do Aly
help-aly: build
	@$(BIN_ALY) --help

# ═══════════════════════════════════════════════════════════════════════════════
# APG (Gerenciador de pacotes)
# ═══════════════════════════════════════════════════════════════════════════════

.PHONY: apg apg-init apg-install apg-update

## Mostra ajuda do APG
apg: build
	@$(BIN_APG) --help

## Inicializa um projeto Aly
apg-init: build
	@$(BIN_APG) init

## Instala dependências
apg-install: build
	@$(BIN_APG) install

## Atualiza dependências
apg-update: build
	@$(BIN_APG) update

# ═══════════════════════════════════════════════════════════════════════════════
# Documentação
# ═══════════════════════════════════════════════════════════════════════════════

.PHONY: doc doc-open

## Lista documentos disponíveis
doc:
	@echo -e "$(CYAN)$(BOLD)Documentação disponível:$(RESET)"
	@ls -1 $(DOC_DIR)/*.md 2>/dev/null | while read f; do \
		echo -e "  $(GREEN)→$(RESET) $$f"; \
	done

## Abre a documentação (usa xdg-open)
doc-open:
	@xdg-open $(DOC_DIR)/index.md 2>/dev/null || \
		echo -e "$(YELLOW)⚠ Não foi possível abrir o navegador$(RESET)"

# ═══════════════════════════════════════════════════════════════════════════════
# Exemplos
# ═══════════════════════════════════════════════════════════════════════════════

.PHONY: examples example-hello example-loop

## Lista exemplos disponíveis
examples:
	@echo -e "$(CYAN)$(BOLD)Exemplos disponíveis:$(RESET)"
	@ls -1 $(EXAMPLE_DIR)/*.aly 2>/dev/null | while read f; do \
		echo -e "  $(GREEN)→$(RESET) $$f"; \
	done

## Roda o exemplo hello world
example-hello: build
	@$(BIN_ALY) run $(EXAMPLE_DIR)/hello.aly

## Roda o exemplo de loop
example-loop: build
	@$(BIN_ALY) run $(EXAMPLE_DIR)/loop.aly

# ═══════════════════════════════════════════════════════════════════════════════
# Docker
# ═══════════════════════════════════════════════════════════════════════════════

.PHONY: docker-build docker-run

## Build da imagem Docker
docker-build:
	@echo -e "$(CYAN)$(BOLD)▸ Buildando imagem Docker...$(RESET)"
	docker build -t aly:latest .
	@echo -e "$(GREEN)✓ Imagem aly:latest criada$(RESET)"

## Roda container Docker
docker-run:
	@docker run -it --rm -v $(PWD):/app aly:latest

# ═══════════════════════════════════════════════════════════════════════════════
# Profiling
# ═══════════════════════════════════════════════════════════════════════════════

.PHONY: profile flamegraph

## Gera perfil de performance
profile: build
	@echo -e "$(CYAN)$(BOLD)▸ Gerando perfil...$(RESET)"
	$(CARGO) build --release
	@echo -e "$(DIM)  Use: perf record -g $(BIN_ALY) run <file>.aly$(RESET)"
	@echo -e "$(DIM)  Depois: perf report$(RESET)"

## Gera flamegraph
flamegraph: build
	@echo -e "$(CYAN)$(BOLD)▸ Gerando flamegraph...$(RESET)"
	@if command -v cargo-flamegraph &>/dev/null; then \
		cargo flamegraph -- - run test.aly; \
	else \
		echo -e "$(RED)Instale cargo-flamegraph: cargo install flamegraph$(RESET)"; \
	fi

# ═══════════════════════════════════════════════════════════════════════════════
# Utilitários
# ═══════════════════════════════════════════════════════════════════════════════

.PHONY: version info status watch

## Mostra versão
version:
	@echo -e "$(CYAN)$(PKG_NAME)$(RESET) v$(VERSION)"

## Mostra informações do projeto
info:
	@echo -e "$(CYAN)$(BOLD)$(PKG_NAME) v$(VERSION)$(RESET)"
	@echo ""
	@echo -e "$(DIM)Binários:$(RESET)"
	@echo -e "  aly  — interpretador/compilador"
	@echo -e "  apg  — gerenciador de pacotes"
	@echo ""
	@echo -e "$(DIM)Targets:$(RESET)"
	@echo -e "  make build          — compila release"
	@echo -e "  make test           — roda testes Rust"
	@echo -e "  make test-aly       — roda testes .aly"
	@echo -e "  make bench          — roda benchmarks"
	@echo -e "  make run FILE=x.aly — roda arquivo"
	@echo -e "  make comp FILE=x.aly — compila para nativo"
	@echo -e "  make editor-build   — instala extensão VS Code"
	@echo -e "  make help           — mostra todos os targets"

## Verifica status do projeto
status: check test

## Watch: recompila a cada mudança (requer cargo-watch)
watch:
	@if command -v cargo-watch &>/dev/null; then \
		cargo watch -x check -x test; \
	else \
		echo -e "$(RED)Instale cargo-watch: cargo install cargo-watch$(RESET)"; \
	fi

# ═══════════════════════════════════════════════════════════════════════════════
# Help
# ═══════════════════════════════════════════════════════════════════════════════

.PHONY: help
.DEFAULT_GOAL := help

## Mostra ajuda completa
help:
	@echo -e "$(CYAN)$(BOLD)"
	@echo "  _                 _"
	@echo " / \   __ _  __ _(_) ___  ___"
	@echo "/ _ \ / _\` |/ _\` | |/ _ \/ __|"
	@echo "/ ___ \ (_| | (_| | | (_) \__ \\"
	@echo "/_/   \_\__, |\__, |_|\___/|___/"
	@echo "        |___/   Makefile"
	@echo -e "$(RESET)"
	@echo -e "$(BOLD)Uso:$(RESET) make <target>"
	@echo ""
	@echo -e "$(BOLD)Build:$(RESET)"
	@echo "  make build           Build release (otimizado)"
	@echo "  make build-debug     Build debug"
	@echo "  make clean           Remove artefatos de build"
	@echo "  make install         Instala em /usr/local/bin"
	@echo "  make uninstall       Remove do /usr/local/bin"
	@echo ""
	@echo -e "$(BOLD)Testes:$(RESET)"
	@echo "  make test            Testes Rust"
	@echo "  make test-unit       Testes unitários"
	@echo "  make test-integration Testes de integração"
	@echo "  make test-all        Rust + .aly tests"
	@echo "  make test-aly        Scripts .aly"
	@echo "  make test-quick      Teste rápido com test.aly"
	@echo "  make check           Verifica código"
	@echo "  make fmt             Formata código"
	@echo "  make clippy          Lint (clippy)"
	@echo ""
	@echo -e "$(BOLD)Benchmarks:$(RESET)"
	@echo "  make bench           Benchmarks completos"
	@echo "  make bench-prime     Benchmark de primos"
	@echo "  make bench-array     Benchmark de arrays"
	@echo ""
	@echo -e "$(BOLD)Execução:$(RESET)"
	@echo "  make run FILE=x.aly  Roda arquivo .aly"
	@echo "  make run-vm FILE=x.aly Roda via VM (bytecode)"
	@echo "  make comp FILE=x.aly Compila para binário nativo"
	@echo "  make repl            Inicia REPL"
	@echo ""
	@echo -e "$(BOLD)Editor:$(RESET)"
	@echo "  make editor-install  Menu interativo de instalação"
	@echo "  make editor-build    Empacula e instala no VS Code"
	@echo "  make editor-uninstall Remove extensão do VS Code"
	@echo ""
	@echo -e "$(BOLD)Pacotes (APG):$(RESET)"
	@echo "  make apg             Ajuda do APG"
	@echo "  make apg-init        Inicializa projeto"
	@echo "  make apg-install     Instala dependências"
	@echo ""
	@echo -e "$(BOLD)Outros:$(RESET)"
	@echo "  make examples        Lista exemplos"
	@echo "  make doc             Lista documentação"
	@echo "  make info            Informações do projeto"
	@echo "  make version         Mostra versão"
	@echo "  make watch           Watch mode (cargo-watch)"
	@echo "  make profile         Gera perfil de performance"
	@echo "  make flamegraph      Gera flamegraph"
