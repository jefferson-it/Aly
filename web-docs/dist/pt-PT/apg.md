# APG — Gestor de Pacotes Aly

APG (Aly Package Manager) é o gestor de pacotes oficial para instalar, gerir e publicar pacotes Aly.

---

## 1. Comandos CLI

### Inicializar um novo projeto

```bash
apg init myproject
```

Cria um novo directório com `manifest.toml`:

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

Lê `manifest.toml` e instala todas as dependências listadas.

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

### Pesquisar no registo

```bash
apg search json
apg search --tag web
```

---

## 2. Dependências

Os pacotes são resolvidos a partir do registo de pacotes Aly. As dependências são bloqueadas em `apg.lock` para compilações reproduzíveis.

```bash
apg install  # Instalar a partir do ficheiro de bloqueio
apg update   # Actualizar todas as dependências para as versões compatíveis mais recentes
```

---

## 3. Desenvolvimento Local

```bash
apg add --path ../my-lib  # Adicionar pacote local
```

---

## 4. Registo

O registo predefinido é `https://registry.apm.io`. Registos personalizados podem ser configurados:

```bash
apg config set registry https://my-registry.com
```
