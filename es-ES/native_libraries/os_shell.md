# Utilidades de OS y Shell en Aly

El módulo `os` proporciona consultas de información del sistema. El módulo `shell` proporciona ejecución de comandos y gestión de procesos.

---

## 1. Información del SO (`os`)

```aly
import os

print(os.platform())     # e.g., "linux", "macos", "windows"
print(os.arch())         # e.g., "x86_64", "aarch64"
print(os.type())         # OS family: "unix", "windows"
print(os.release())      # Kernel release version
print(os.hostname())     # System hostname
print(os.homedir())      # Home directory path
print(os.tmpdir())       # Temp directory path
print(os.pid())          # Current process ID
print(os.ppid())         # Parent process ID
print(os.endianness())   # "LE" or "BE"
print(os.eol())          # Line ending character(s)
print(os.uptime())       # System uptime in seconds
```

### Información de Memoria

```aly
print(os.totalmem())   # Total system memory in bytes
print(os.freemem())    # Available memory in bytes
print(os.usedmem())    # Used memory in bytes
print(os.loadavg())    # [1min, 5min, 15min] load averages
```

### Información de CPU

```aly
let cpus = os.cpus()     # Vector de objetos CPU
let user = os.userinfo() # Objeto con uid, gid, username, shell, homedir
let net = os.network_interfaces()  # Vector de objetos de interfaz de red
```

### Llamada al Sistema Raw

```aly
let result = os.syscall(syscall_number, arg1, arg2, arg3)
# Interfaz de llamada al sistema raw específica de Linux
```

---

## 2. Ejecución de Comandos de Shell (`shell`)

### Ejecución Básica

```aly
import shell

let output = shell.exec("ls -la")
print(output)  # stdout del comando

let lines = shell.exec_lines("cat file.txt")
# Devuelve vector de líneas

let result = shell.exec_status("git status")
print(result.code)     # Exit code (int)
print(result.stdout)   # stdout text
print(result.stderr)   # stderr text
```

### Procesos en Segundo Plano

```aly
let pid = shell.spawn("long-running-task.sh")
# Spawn no bloqueante, devuelve PID

let code = shell.wait(pid)          # Esperar a que termine
let out = shell.wait_output(pid)    # Esperar y recopilar salida
shell.kill(pid)                     # Matar proceso
shell.pid_exists(pid)               # Verificar si aún está registrado
```

### Pipes de Shell

```aly
let result = shell.pipe(["cat file.txt", "grep hello", "wc -l"])
# Redirige la salida de un comando como entrada al siguiente

let lines = shell.pipe_lines(["ps aux", "grep aly"])
```

### Variables de Entorno

```aly
let val = shell.env_get("PATH")
shell.env_set("MY_VAR", "myvalue")
shell.env_remove("MY_VAR")
let all = shell.env_list()  # Objeto con todas las variables de entorno
```

### Directorio de Trabajo

```aly
shell.cd("/home/user/projects")
let dir = shell.pwd()
```

### Gestión de Procesos

```aly
let found = shell.which("python")     # Ruta al ejecutable
let exists = shell.is_command("git")  # ¿Está en PATH?
shell.signal(1234, "SIGTERM")         # Enviar señal
let procs = shell.process_list()      # Listar procesos (ps aux)
let procs = shell.process_list("aly") # Filtrar por nombre
shell.exit(0)                         # Salir del proceso
```

### Shell Personalizado

```aly
let out = shell.run_in_shell("echo hi", "/bin/bash")
```

### Pseudo-TTY

```aly
let output = shell.pty("top -b -n 1")
# Ejecuta el comando con asignación de PTY (para programas que necesitan un terminal)
```