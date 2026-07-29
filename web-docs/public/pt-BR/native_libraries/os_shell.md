# Utilitários de SO e Shell no Aly

O módulo `os` fornece consultas de informações do sistema. O módulo `shell` fornece execução de comandos e gerenciamento de processos.

---

## 1. Informações do SO (`os`)

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

### Informações de Memória

```aly
print(os.totalmem())   # Total system memory in bytes
print(os.freemem())    # Available memory in bytes
print(os.usedmem())    # Used memory in bytes
print(os.loadavg())    # [1min, 5min, 15min] load averages
```

### Informações da CPU

```aly
let cpus = os.cpus()     # Vector of CPU objects
let user = os.userinfo() # Object with uid, gid, username, shell, homedir
let net = os.network_interfaces()  # Vector of network interface objects
```

### Chamada de Sistema Bruta

```aly
let result = os.syscall(syscall_number, arg1, arg2, arg3)
# Linux-specific raw syscall interface
```

---

## 2. Execução de Comando do Shell (`shell`)

### Execução Básica

```aly
import shell

let output = shell.exec("ls -la")
print(output)  # stdout of the command

let lines = shell.exec_lines("cat file.txt")
# Returns vector of lines

let result = shell.exec_status("git status")
print(result.code)     # Exit code (int)
print(result.stdout)   # stdout text
print(result.stderr)   # stderr text
```

### Processos em Segundo Plano

```aly
let pid = shell.spawn("long-running-task.sh")
# Non-blocking spawn, returns PID

let code = shell.wait(pid)          # Wait for completion
let out = shell.wait_output(pid)    # Wait and collect output
shell.kill(pid)                     # Kill process
shell.pid_exists(pid)               # Check if still tracked
```

### Pipes de Shell

```aly
let result = shell.pipe(["cat file.txt", "grep hello", "wc -l"])
# Pipes output of one command as input to the next

let lines = shell.pipe_lines(["ps aux", "grep aly"])
```

### Variáveis de Ambiente

```aly
let val = shell.env_get("PATH")
shell.env_set("MY_VAR", "myvalue")
shell.env_remove("MY_VAR")
let all = shell.env_list()  # Object with all env vars
```

### Diretório de Trabalho

```aly
shell.cd("/home/user/projects")
let dir = shell.pwd()
```

### Gerenciamento de Processos

```aly
let found = shell.which("python")     # Path to executable
let exists = shell.is_command("git")  # Is in PATH?
shell.signal(1234, "SIGTERM")         # Send signal
let procs = shell.process_list()      # List processes (ps aux)
let procs = shell.process_list("aly") # Filter by name
shell.exit(0)                         # Exit process
```

### Shell Personalizado

```aly
let out = shell.run_in_shell("echo hi", "/bin/bash")
```

### Pseudo-TTY

```aly
let output = shell.pty("top -b -n 1")
# Runs command with PTY allocation (for programs needing a terminal)
```