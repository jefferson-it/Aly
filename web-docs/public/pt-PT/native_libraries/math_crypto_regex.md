# Matemática, Criptografia e Regex em Aly

Aly expõe utilitários matemáticos nativos, primitivas criptográficas (hash, encriptação) e motores de correspondência de expressões regulares.

---

## 1. Funções Matemáticas Avançadas (`math`)

Expõe trigonometria padrão, logaritmos e potências:
```aly
import math

let value = math.sin(3.14159 / 2) # Cálculos de seno
let root = math.sqrt(16)          # Raiz quadrada (4)
let log_val = math.log10(100)     # Logaritmo base 10 (2)

print("Raiz: , Log: ")
```

---

## 2. Hashes Criptográficos e AES (`crypto`)

Hash de saídas de texto de forma segura (MD5, SHA1, SHA256, SHA512) ou encriptar/desencriptar com AES-256-GCM:
```aly
import crypto

let sha = crypto.sha256("Password123")
print("Hash SHA256: ")

# Encriptação AES
let key = "32byte_super_secure_key_needed..."
let secret = "Informação sensível"
let ciphertext = crypto.aes_encrypt(secret, key)
let decrypted = crypto.aes_decrypt(ciphertext, key)

print("Desencriptado: ")
```

---

## 3. Expressões Regulares (`regex`)

Corresponder, capturar e substituir texto utilizando padrões regex padrão:
```aly
import regex

let pattern = "^[a-zA-Z0-9+_.-]+@[a-zA-Z0-9.-]+$" # Padrão de validação de email
let is_valid = regex.match("user@example.com", pattern)
print("Email válido: ")

let cleaned = regex.replace("Hello 123 World", "[0-9]+", "Aly")
print("Texto limpo: ") # Hello Aly World
```
