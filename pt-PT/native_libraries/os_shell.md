# Utilitários de Sistema e Consola em Aly

O módulo `os` fornece consultas de informação do sistema. O módulo `shell` fornece execução de comandos e gestão de processos.

---

## 1. Informação do Sistema Operativo (`os`)

```aly
import os

print(os.platform())     # Por exemplo, "linux", "macos", "windows"
print(os.arch())         # Por exemplo, "x86_64", "aarch64"
print(os.type())         # Família do SO: "unix", "windows"
print(os.release())      # Versão de lançamento do kernel
print(os.hostname())     # Nome do sistema
print(os.homedir())      # Caminho do directório pessoal
print(os.tmpdir())       # Caminho do directório temporário
print(os.pid())          # ID do processo actual
print(os.ppid())         # ID do processo progenitor
print(os.endianness())   # "LE" ou "BE"
print(os.eol())          # Carácter(es) de fim de linha
print(os.uptime())       # Tempo de actividade do sistema em segundos
```

### Informação de Memória

```aly
print(os.totalmem())   # Memória total do sistema em bytes
print(os.freemem())      # Memória disponível em bytes
print(os.usedmem())    # Memória utilizada em bytes
print(os.loadavg())    # Média de carga [1min, 5min, 15min]
```

### Informação da CPU

```aly
let cpus = os.cpus()     # Vetor de objectos de CPU
let user = os.userinfo() # Objecto com uid, gid, username, shell, homedir
let net = os.network_interfaces()  # Vetor de objectos de interface de rede
```

### Chamada de Sistema Bruta

```aly
let result = os.syscall(syscall_number, arg1, arg2, arg3)
# Interface de chamada de sistema bruta específica de Linux
```

---

## 2. Execução de Comandos de Consola (`shell`)

### Execução Básica

```aly
import shell

let output = shell.exec("ls -la")
print(output)  # stdout do comando

let lines = shell.exec_lines("cat file.txt")
# Devolve vetor de linhas

let result = shell.exec_status("git status")
print(result.code)     # Código de saída (inteiro)
print(result.stdout)   # Texto stdout
print(result.stderr)   # Texto stderr
```

### Processos em Segundo Plano

```aly
let pid = shell.spawn("long-running-task.sh")
# Criação não bloqueante, devolve PID

let code = shell.wait(pid)          # Aguardar conclusão
let out = shell.wait_output(pid)    # Aguardar e recolher saída
shell.kill(pid)                     # Eliminar processo
shell.pid_exists(pid)               # Verificar se ainda está registado
```

### Pipelines de Consola

```aly
let result = shell.pipe(["cat file.txt", "grep hello", "wc -l"])
# Encaminha a saída de um comando como entrada para o seguinte

let lines = shell.pipe_lines(["ps aux", "grep aly"])
```

### Variáveis de Ambiente

```aly
let val = shell.env_get("PATH")
shell.env_set("MINHA_VAR", "meuvalor")
shell.env_remove("MINHA_VAR")
let all = shell.env_list()  # Objecto com todas as variáveis de ambiente
```

### Directório de Trabalho

```aly
shell.cd("/home/user/projetos")
let dir = shell.pwd()
```

### Gestão de Processos

```aly
let found = shell.which("python")     # Caminho para o executável
let exists = shell.is_command("git")  # Está no PATH?
shell.signal(1234, "SIGTERM")         # Enviar sinal
let procs = shell.process_list()      # Listar processos (ps aux)
let procs = shell.process_list("aly") # Filtrar por nome
shell.exit(0)                           # Sair do processo
```

### Consola Personalizada

```aly
let out = shell.run_in_shell("echo hi", "/bin/bash")
```

### Pseudo-TTY

```aly
let output = shell.pty("top -b -n 1")
# Executa comando com alocação PTY (para programas que necessitam de uma consola/terminal)
```
