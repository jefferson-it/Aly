# Sistema, Ambiente e Utilitários de SO em Aly

O Aly fornece bibliotecas para aceder a variáveis de ambiente principais, verificar configurações, ler perfis de configuração e gerir terminais do sistema.

---

## 1. Perfis de Ambiente (`dotenv`)

Carregue variáveis `.env` diretamente nas vinculações de ambiente ativas:
```aly
import dotenv

dotenv.load() # Reads .env file in project root

let api_key = dotenv.get("API_KEY")
print("API Key: $api_key")
```

---

## 2. Geração de UUID (`uuid`)

Gere cadeias de identificadores únicos de forma nativa:
```aly
import uuid

let id = uuid.v4()
print("Generated Unique ID: $id")
```

---

## 3. Controlo de Terminal (`console`)

Controle saídas, cores e leia as entradas padrão do terminal:
```aly
import console

# Clear terminal screen
console.clear()

# Colored terminal outputs using ANSI colors
console.print_color("Green success text", "green")
console.print_color("Red error message", "red")

# Read user input directly
let name = console.input("Enter name: ")
print("Hello, $name")
```
