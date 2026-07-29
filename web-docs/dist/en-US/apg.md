# APG — Aly Package Manager

APG (Aly Package Manager) is the official package manager for installing, managing, and publishing Aly packages.

---

## 1. CLI Commands

### Initialize a new project

```bash
apg init myproject
```

Creates a new directory with `manifest.toml`:

```toml
[package]
name = "myproject"
version = "0.1.0"
description = "My Aly project"
```

### Install dependencies

```bash
apg install
```

Reads `manifest.toml` and installs all listed dependencies.

### Add a dependency

```bash
apg add http-client
apg add http-client@1.2.0  # Specific version
```

### Remove a dependency

```bash
apg remove http-client
```

### Publish a package

```bash
apg publish
```

### Search registry

```bash
apg search json
apg search --tag web
```

---

## 2. Dependencies

Packages are resolved from the Aly package registry. Dependencies are locked in `apg.lock` for reproducible builds.

```bash
apg install  # Install from lock file
apg update   # Update all dependencies to latest compatible versions
```

---

## 3. Local Development

```bash
apg add --path ../my-lib  # Add local package
```

---

## 4. Registry

The default registry is `https://registry.apm.io`. Custom registries can be configured:

```bash
apg config set registry https://my-registry.com
```