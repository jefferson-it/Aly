# Matemáticas, Criptografía y Regex en Aly

Aly expone utilidades matemáticas nativas, primitivas criptográficas (hashes, cifrado) y motores de coincidencia de expresiones regulares.

---

## 1. Funciones Matemáticas Avanzadas (`math`)

Expone trigonometría estándar, logaritmos y potencias:
```aly
import math

let value = math.sin(3.14159 / 2) # Cálculos de seno
let root = math.sqrt(16)          # Raíz cuadrada (4)
let log_val = math.log10(100)     # Logaritmo base 10 (2)

print("Root: $root, Log: $log_val")
```

---

## 2. Hashes Criptográficos y AES (`crypto`)

Genera hashes de texto de forma segura (MD5, SHA1, SHA256, SHA512) o cifra/descifra con AES-256-GCM:
```aly
import crypto

let sha = crypto.sha256("Password123")
print("SHA256 Hash: $sha")

# Cifrado AES
let key = "32byte_super_secure_key_needed..."
let secret = "Sensitive information"
let ciphertext = crypto.aes_encrypt(secret, key)
let decrypted = crypto.aes_decrypt(ciphertext, key)

print("Decrypted: $decrypted")
```

---

## 3. Expresiones Regulares (`regex`)

Coincidencia, captura y reemplazo de texto usando patrones regex estándar:
```aly
import regex

let pattern = "^[a-zA-Z0-9+_.-]+@[a-zA-Z0-9.-]+$" # Patrón de validación de email
let is_valid = regex.match("user@example.com", pattern)
print("Valid email: $is_valid")

let cleaned = regex.replace("Hello 123 World", "[0-9]+", "Aly")
print("Cleaned text: $cleaned") # Hello Aly World
```