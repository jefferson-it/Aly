# UUID
Generate universally unique identifiers.

| Function | Parameters | Returns | Description |
| ------------------------------------------------- | ---------- | ---------- | ------------------------------------------------- |
| `uuid_v4(_unused: string)` | `_unused: string` | `string` (quoted) | Create a random UUID version 4 and return it as a quoted string (e.g., `"f47ac10b-58cc-4372-a567-0e02b2c3d479"`). |

```aly
let id = uuid_v4("")   # ignore argument
print(id)              # → "f47ac10b-58cc-4372-a567-0e02b2c3d479"
```

### Notes
- The function discards the input argument; it is only present to match a predefined API.
- UUID generation uses the Rust `uuid` crate, guaranteeing RFC 4122 compliance.
- Multiple calls produce different values.