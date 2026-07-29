# APG — Gerenciador de Pacotes do Aly

O APG (Aly Package Manager) é o gerenciador de pacotes oficial para instalar, gerenciar e publicar pacotes do Aly.

---

## 1. Comandos CLI

### Inicializar um novo projeto

```bash
apg init myproject
```

Cria um novo diretório com `manifest.toml`:

```toml
[package]
name = "myproject"
version = "0.1.0"
description = "My Aly project"
```

### Instalar dependências

```bash
apg install
```

Lê o `manifest.toml` e instala todas as dependências listadas.

### Adicionar uma dependência

```bash
apg add http-client
apg add http-client@1.2.0  # Versão específica
```

### Remover uma dependência

```bash
apg remove http-client
```

### Publicar um pacote

```bash
apg publish
```

### Buscar no registro

```bash
apg search json
apg search --tag web
```

---

## 2. Dependências

Os pacotes são resolvidos a partir do registro de pacotes do Aly. As dependências são bloqueadas no `apg.lock` para builds reproduzíveis.

```bash
apg install  # Instalar do arquivo de lock
apg update   # Atualizar todas as dependências para as versões compatíveis mais recentes
```

---

## 3. Desenvolvimento Local

```bash
apg add --path ../my-lib  # Adicionar pacote local
```

---

## 4. Registro

O registro padrão é `https://registry.apm.io`. Registros personalizados podem ser configurados:

```bash
apg config set registry https://my-registry.com
```