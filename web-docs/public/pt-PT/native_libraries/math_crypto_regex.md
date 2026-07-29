# Matemática, Criptografia e Regex em Aly

O Aly expõe utilitários matemáticos nativos, primitivas criptográficas (hashes, encriptação) e motores de correspondência de expressões regulares.

---

## 1. Funções Matemáticas Avançadas (`math`)

Expõe trigonometria padrão, logaritmos e potências:
```aly
import math

let value = math.sin(3.14159 / 2) # Sine calculations
let root = math.sqrt(16)          # Square root (4)
let log_val = math.log10(100)     # Logarithm base 10 (2)

print("Root: $root, Log: $log_val")
```

---

## 2. Hashes Criptográficos e AES (`crypto`)

Gere hashes de saídas de texto com segurança (MD5, SHA1, SHA256, SHA512) ou encripte/desencripte com AES-256-GCM:
```aly
import crypto

let sha = crypto.sha256("Password123")
print("SHA256 Hash: $sha")

# AES Encryption
let key = "32byte_super_secure_key_needed..."
let secret = "Sensitive information"
let ciphertext = crypto.aes_encrypt(secret, key)
let decrypted = crypto.aes_decrypt(ciphertext, key)

print("Decrypted: $decrypted")
```

---

## 3. Expressões Regulares (`regex`)

Faça a correspondência, capture e substitua texto utilizando padrões de regex padrão:
```aly
import regex

let pattern = "^[a-zA-Z0-9+_.-]+@[a-zA-Z0-9.-]+$" # Email validator pattern
let is_valid = regex.match("user@example.com", pattern)
print("Valid email: $is_valid")

let cleaned = regex.replace("Hello 123 World", "[0-9]+", "Aly")
print("Cleaned text: $cleaned") # Hello Aly World
```
