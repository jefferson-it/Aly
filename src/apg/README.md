# APG - Aly Package Gestor

## Visão Geral

APG (Aly Package Gestor) é o gerenciador de pacotes oficial da linguagem Aly. Ele segue uma filosofia simples, determinística, segura e baseada em Git, semelhante a linguagens como Go, Zig e Cargo, porém com identidade própria.

### Princípios Fundamentais

- **Git é a principal fonte de distribuição**
- **Sem servidor dedicado** (o registro oficial é apenas um índice)
- **Pacotes pertencem aos seus autores**
- **Instalação determinística**
- **Dependências reproduzíveis**
- **Simplicidade acima de recursos**

### Características Principais

- Gerência de pacotes baseada em Git (sem armazenamento intermediário)
- Suporte a múltiplos tipos de dependência:
  - Simples: `json = "1.0"`
  - Git: `parser = { git = "https://github.com/user/parser" }`
  - Local: `math = { path = "../math" }`
- Cache de pacotes (`~/.cache/apg/`)
- Arquivo de lock (`aly.lock`)
- Consultas ao registro (`aly apg-registry/packages/*.toml`)
- Comandos básicos: init, add, remove, update, publish, search, install, uninstall, doctor, clean, info, graph, lock, vendor
- Workspaces com múltiplos projetos
- Configuração de registro privado

### Fluxo de Trabalho

1. **Criar um pacote**
   ```bash
   apg init
   ```

2. **Adicionar dependências**
   ```bash
   apg add json
   apg add parser --git https://github.com/user/parser --branch main
   ```

3. **Publicar**
   ```bash
   apg publish
   ```
   (Cria um Pull Request para o registro oficial)

4. **Instalar**
   ```bash
   apg install json
   ```

### Exemplo de aly.toml

```toml
[package]
name = "meu-pacote"
version = "1.0.0"
authors = ["Jefferson"]
repository = "https://github.com/usuario/meu-pacote"
homepage = "https://github.com/usuario/meu-pacote"
keywords = ["exemplo", "demonstração"]

[dependencies]
json = "1.0"
crypto = "^2.0"
parser = { git = "https://github.com/user/parser" }
math = { path = "../math" }
```

### Exemplo de aly.lock

```toml
[packages]
json = "1.2.4"
crypto = "0.9.1"
parser = "2.1.3"
math = "local"
```

### Instalação Executável

O APG está instalado como um comando `apg`:

```bash
# Instalar APG em qualquer projeto Aly
# (geralmente como parte do sistema)

# Usar APG
apg init meu-projeto
cd meu-projeto
apg add json
apg install
cd ..
apg init outro-projeto
# Adicione o caminho para o outro projeto ao aly.lock
apg vendor
```

O executável é instalado em:
- `~/.local/bin` (Linux/macOS)
- Equivalente do sistema (Windows)

### Segurança

O APG valida sempre:
- Hash do pacote
- Integridade
- Versão
- Manifesto
- Nome do pacote

Evita ataques por nomes parecidos (typosquatting).

### Versionamento

Usa Semantic Versioning (semver).
Suporta operadores:
- `^` (careira)
- `~` (tilde)
- `>=`
- `<=`
- `>= ... <`

### Registro Oficial

O registro oficial é um repositório Git contendo apenas índices:

```
aly-registry/
  packages/
    json.toml
    crypto.toml
    parser.toml
```

Exemplo de registro:

```toml
[package]
name = "json"
repository = "https://github.com/aly/json"
latest = "1.2.4"
```

Todos os arquivos permanecem no repositório do autor; o registro apenas indexa.

### O que NÃO é implementado

O APG **não** cria:
- Servidor próprio
- Upload de pacotes
- Conta de usuário
- Login
- Autenticação centralizada
- Banco de dados
- Sistema de estrelas
- Comentários
- Avaliações
- Estatísticas online
- Dependências globais mágicas
- Scripts automáticos equivalentes a `preinstall`, `postinstall`, `prepare`
- Execução automática de código durante a instalação

### Segurança e Determinismo

APG nunca executa código durante a instalação (evita código malicioso).
Todas as instalações são determinísticas: o mesmo projeto compila idêntico hoje e daqui a dez anos.

O APG transmite a sensação de ser uma ferramenta do sistema operacional, não um serviço online.

O usuário pode confiar que:
- Instalar um pacote é previsível
- O mesmo projeto compila hoje e daqui a dez anos
- O Git é a fonte da verdade
- O registro é apenas um catálogo
- O APG permanece rápido, transparente e de baixa manutenção

### Comandos

```
apg init
apg add <pacote>
apg remove <pacote>
apg update
apg publish
apg search <termo>
apg install <pacote>
apg uninstall <pacote>
apg registry
apg doctor
apg clean
apg info <pacote>
apg graph
apg lock
apg vendor
```
