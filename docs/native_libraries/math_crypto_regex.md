# Math, Cryptography, and Regex in Aly

Aly exposes native mathematical utilities, cryptographic primitives (hashes, encryption), and regular expression match engines.

---

## 1. Advanced Math Functions (`math`)

Exposes standard trigonometry, logarithms, and powers:
```aly
import math

let value = math.sin(3.14159 / 2) # Sine calculations
let root = math.sqrt(16)          # Square root (4)
let log_val = math.log10(100)     # Logarithm base 10 (2)

print("Root: $root, Log: $log_val")
```

---

## 2. Cryptographic Hashes & AES (`crypto`)

Securely hash text outputs (MD5, SHA1, SHA256, SHA512) or encrypt/decrypt with AES-256-GCM:
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

## 3. Regular Expressions (`regex`)

Match, capture, and replace text using standard regex patterns:
```aly
import regex

let pattern = "^[a-zA-Z0-9+_.-]+@[a-zA-Z0-9.-]+$" # Email validator pattern
let is_valid = regex.match("user@example.com", pattern)
print("Valid email: $is_valid")

let cleaned = regex.replace("Hello 123 World", "[0-9]+", "Aly")
print("Cleaned text: $cleaned") # Hello Aly World
```
