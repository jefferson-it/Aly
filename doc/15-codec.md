# Codec
Documentation for Aly‑Lang’s encoding and decoding utilities.

| Function | Parameters | Returns | Description |
| ------------------------------------------------- | ---------- | ---------- | ------------------------------------------------- |
| `base64_encode(string)` | `input: string` | `string` (quoted) | Encode `input` using standard Base64 encoding. |
| `base64_decode(string)` | `input: string` | `string` (quoted) | Decode a Base64‑encoded string. Returns an empty quoted string on invalid input. |
| `hex_encode(string)` | `input: string` | `string` (quoted) | Encode `input` as hexadecimal (lower‑case). |
| `hex_decode(string)` | `input: string` | `string` (quoted) | Decode a hexadecimal string. Returns an empty quoted string on invalid input. |

All functions treat the result as a quoted string; invalid Base64 or hex input yields an empty quoted value.

```aly
# Base64 encode/decode
print(base64_encode("hello world"))   # → "aGVsbG8gd29ybGQ="
print(base64_decode("aGVsbG8gd29ybGQ="))  # → "hello world"

# Hex encode/decode
print(hex_encode("Alicia"))           # → "416c69636961"
print(hex_decode("416c69636961"))     # → "Alicia"
```

### Error handling
- Invalid Base64 or hex characters cause a runtime warning and return an empty quoted string.
- Decoding functions never panic; they gracefully return `""`.

### Notes
- Encodings are performed via the Rust `base64` and `hex` crates, guaranteeing RFC‑4648 compliance for Base64 and standard hex encoding.
- The functions operate on UTF‑8 strings; byte sequences are encoded directly.