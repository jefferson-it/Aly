# APG — Gestor de Paquetes de Aly

APG (Aly Package Manager) es el gestor de paquetes oficial para instalar, gestionar y publicar paquetes de Aly.

---

## 1. Comandos CLI

### Inicializar un nuevo proyecto

```bash
apg init myproject
```

Crea un nuevo directorio con `manifest.toml`:

```toml
[package]
name = "myproject"
version = "0.1.0"
description = "My Aly project"
```

### Instalar dependencias

```bash
apg install
```

Lee `manifest.toml` e instala todas las dependencias listadas.

### Añadir una dependencia

```bash
apg add http-client
apg add http-client@1.2.0  # Versión específica
```

### Eliminar una dependencia

```bash
apg remove http-client
```

### Publicar un paquete

```bash
apg publish
```

### Buscar en el registro

```bash
apg search json
apg search --tag web
```

---

## 2. Dependencias

Los paquetes se resuelven desde el registro de paquetes de Aly. Las dependencias se bloquean en `apg.lock` para compilaciones reproducibles.

```bash
apg install  # Instalar desde el archivo de bloqueo
apg update   # Actualizar todas las dependencias a las versiones compatibles más recientes
```

---

## 3. Desarrollo Local

```bash
apg add --path ../my-lib  # Añadir paquete local
```

---

## 4. Registro

El registro predeterminado es `https://registry.apm.io`. Se pueden configurar registros personalizados:

```bash
apg config set registry https://my-registry.com
```