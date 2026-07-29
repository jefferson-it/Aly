# DotEnv
Load environment variables from a `.env` file into the process environment.

| Function | Parameters | Returns | Description |
| ------------------------------------------------- | ---------- | ---------- | ------------------------------------------------- |
| `dotenv(path?)` | `path: string` (optional) | `string` (quoted) | If `path` is omitted, reads `.env` from the working directory. Otherwise, reads the specified file (quotes may be removed). Each `key=value` line (optionally prefixed with `export`) is parsed; values are stripped of surrounding quotes. Variables are set via `std::env::set_var`. The function returns `"None"` after processing. |

```aly
# Default: load .env in cwd
dotenv()

# Load a specific file
dotenv(".env.secrets")

# Access a variable later (via standard env lookup)
let api_key = sys.env("API_KEY")
```

### Remarks
- Keys must be non‑empty; malformed lines are silently ignored.
- Values may be quoted with `"` or `'`; the quotes are removed before storage.
- Existing environment variables are overwritten.
- Errors (e.g., file not found) are logged as `RuntimeError` but do not abort execution.