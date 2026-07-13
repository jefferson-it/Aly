# Crypto
Reference documentation for the cryptographic utilities available in Aly‑Lang’s standard library.

| Function | Parameters | Returns | Description |
| ------------------------------------------------- | ---------- | ---------- | ------------------------------------------------- |
| `crypto_md5(string)` | `text: string` | `string` | Compute the MD5 hash of `text` and return the hex‑encoded digest (lower‑case). |
| `crypto_sha256(string)` | `text: string` | `string` | Compute the SHA‑256 hash of `text` and return the hex‑encoded digest. |
| `crypto_sha512(string)` | `text: string` | `string` | Compute the SHA‑512 hash of `text` and return the hex‑encoded digest. |
| `crypto_hmac(algorithm, key, data)` | `algorithm: string`, `key: string`, `data: string` | `string` | Generate an HMAC signature using the given `algorithm` (`"sha256"` or `"sha512"`), `key`, and `data`. Returns the hex‑encoded digest. |

All functions return a **quoted string** containing the hexadecimal representation of the hash.

```aly
# MD5 example
print(crypto_md5("hello"))               # → "5d41402abc4b2a76b9719d911017c592"

# SHA‑256 example
print(crypto_sha256("hello"))            # → "b10a8db164e0754105b7a99be72e3fe5"

# HMAC example (SHA‑256)
print(crypto_hmac("sha256", "key", "data"))  # → "3a7bd3e2360a... (hex string)"
```

### Notes
- The functions are **pure** and do not perform I/O.
- Hash functions are based on the Rust `md5`, `sha2`, and `hmac` crates, ensuring IEEE‑754‑compatible digests.
- Invalid algorithm names in `crypto_hmac` result in an empty string and an internal error message.