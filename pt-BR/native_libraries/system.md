# Sistema, Ambiente e Utilitários de SO em Aly

Aly fornece bibliotecas para acessar variáveis de ambiente principais, verificar configurações, ler perfis de configuração e gerenciar terminais do sistema.

---

## 1. Perfis de Ambiente (`dotenv`)

Carrega variáveis `.env` diretamente nas vinculações de ambiente ativo:
```aly
import dotenv

dotenv.load() # Reads .env file in project root

let api_key = dotenv.get("API_KEY")
print("Chave da API: $api_key")
```

---

## 2. Geração de UUID (`uuid`)

Gera strings de identificador único nativamente:
```aly
import uuid

let id = uuid.v4()
print("ID Único Gerado: $id")
```

---

## 3. Controle de Terminal (`console`)

Controla saídas, cores e lê entradas padrão de terminal:
```aly
import console

# Clear terminal screen
console.clear()

# Colored terminal outputs using ANSI colors
console.print_color("Green success text", "green")
console.print_color("Red error message", "red")

# Read user input directly
let name = console.input("Enter name: ")
print("Olá, $name")
```
