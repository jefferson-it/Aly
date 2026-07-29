# Sistema, Ambiente e Utilitários de SO em Aly

Aly disponibiliza bibliotecas para aceder a variáveis de ambiente principais, verificar configurações, ler perfis de configuração e gerir terminais de sistema.

---

## 1. Perfis de Ambiente (`dotenv`)

Carregar variáveis `.env` directamente em ligações de ambiente activas:
```aly
import dotenv

dotenv.load() # Lê o ficheiro .env na raiz do projeto

let api_key = dotenv.get("API_KEY")
print("Chave API: ")
```

---

## 2. Geração de UUID (`uuid`)

Gerar cadeias de identificadores únicos nativamente:
```aly
import uuid

let id = uuid.v4()
print("Identificador Único Gerado: ")
```

---

## 3. Controlo de Terminal/Consola (`console`)

Controlar saídas, cores e ler entradas de terminal/consola padrão:
```aly
import console

# Limpar o ecrã do terminal/consola
console.clear()

# Saídas de terminal/consola coloridas utilizando cores ANSI
console.print_color("Texto de sucesso verde", "green")
console.print_color("Mensagem de erro vermelha", "red")

# Ler input do utilizador directamente
let name = console.input("Introduza o nome: ")
print("Olá, ")
```
