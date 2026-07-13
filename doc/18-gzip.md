# Gzip
Compression and decompression utilities.

| Function | Parameters | Returns | Description |
| ------------------------------------------------- | ---------- | ---------- | ------------------------------------------------- |
| `gzip_compress(input_string)` | `input: string` | `string` (quoted) | Compress `input` using gzip and return the compressed binary as a quoted string. Returns an empty quoted string on failure. |
| `gzip_decompress(compressed_string)` | `input: string` | `string` (quoted) | Decompress a gzip‑compressed string and return the original text as a quoted string. Returns an empty quoted string on failure. |

```aly
let plain = "hello world"
let compressed = gzip_compress(plain)          # → quoted binary string
let restored = gzip_decompress(compressed)     # → "hello world"
```

### Error handling
- Invalid or corrupt data results in an empty quoted value; a runtime error is logged internally.

### Notes
- Uses the Rust `flate2` crate for gzip handling.
- The functions operate on UTF‑8 strings; they convert to bytes internally.