# Path
Utilities for manipulating filesystem paths.

| Function | Parameters | Returns | Description |
| ------------------------------------------------- | ---------- | ---------- | ------------------------------------------------- |
| `path_join(...segments)` | `segments: string` | `string` (quoted) | Join path components using the OS separator, returning the normalized absolute path as a quoted string. Empty segments are ignored. |
| `path_dirname(path)` | `path: string` | `string` (quoted) | Return the directory portion of `path`. If `path` has no parent, returns an empty quoted string. |
| `path_basename(path)` | `path: string` | `string` (quoted) | Return the final component (filename) of `path`. Returns an empty quoted string if none exists. |
| `path_extname(path)` | `path: string` | `string` (quoted) | Return the file extension, including the leading dot (e.g., `".txt"`). Returns an empty quoted string if no extension exists. |
| `path_resolve(...segments)` | `segments: string` | `string` (quoted) | Resolve `segments` against the current working directory, producing an absolute path. If `segments` is empty or relative, it is joined with `cwd()`. |
| `path_parse(path)` | `path: string` | `object` (quoted) | Parse `path` and return a dictionary with keys `root`, `dir`, `base`, `ext`, and `name`, each mapping to the respective component. Missing components are empty strings. |

```aly
# Join segments
print(path_join("home" "user" "documents" "file.txt"))   # → "/home/user/documents/file.txt"

# Dirname
print(path_dirname("/var/log/syslog"))                    # → "/var/log"

# Basename
print(path_basename("/etc/ssl/certs/ca-certificates.crt"))  # → "ca-certificates.crt"

# Extname
print(path_extname("archive.tar.gz"))                     # → ".gz"

# Resolve (absolute)
print(path_resolve("tmp" "data"))                         # → "/home/user/tmp/data"

# Parse
let info = path_parse("/tmp/cache/old.txt")
print(info)   # → {"root":"/", "dir":"/tmp/cache", "base":"old", "ext":".txt", "name":"old"}
```

### Notes
- All results are returned as quoted strings or objects; callers should strip the quotes when using the values.
- The functions use the Rust `std::path` API, guaranteeing platform‑specific separators and semantics.
- Empty inputs yield empty quoted results rather than panicking.