# Standard Library

The standard library exposes three built-in namespaces — `fs`, `str` and `sys`
— called with method syntax: `namespace.method(args)`.

```aly
print(str.upper("hello"))     # HELLO
print(sys.platform())         # linux
```

---

## `fs` — filesystem

| Function                | Returns  | Description                                   |
| ----------------------- | -------- | --------------------------------------------- |
| `fs.read(path)`         | string   | Read the whole file as text.                  |
| `fs.write(path, text)`  | bool     | Write `text` to a file (overwrites).          |
| `fs.append(path, text)` | bool     | Append `text` to a file, creating if needed.  |
| `fs.exists(path)`       | bool     | Whether the path exists.                       |
| `fs.remove(path)`       | bool     | Delete a file or directory (recursive).       |
| `fs.mkdir(path)`        | bool     | Create a directory (with parents).            |
| `fs.list(path)`         | vector   | Names of entries in a directory.              |
| `fs.is_dir(path)`       | bool     | Whether the path is a directory.              |

```aly
let path = "notes.txt"
fs.write(path, "first line")
fs.append(path, "\nsecond line")
print(fs.read(path))
fs.remove(path)
```

Boolean-returning functions report success (`true`) or failure (`false`).

---

## `str` — string manipulation

| Function                     | Returns | Description                                  |
| ---------------------------- | ------- | -------------------------------------------- |
| `str.upper(s)`               | string  | Uppercase.                                   |
| `str.lower(s)`               | string  | Lowercase.                                   |
| `str.trim(s)`                | string  | Remove leading/trailing whitespace.          |
| `str.contains(s, needle)`    | bool    | Whether `s` contains `needle`.               |
| `str.replace(s, from, to)`   | string  | Replace all occurrences.                     |
| `str.split(s, sep)`          | vector  | Split into parts (empty `sep` splits chars). |
| `str.starts_with(s, prefix)` | bool    | Prefix test.                                 |
| `str.ends_with(s, suffix)`   | bool    | Suffix test.                                 |
| `str.index_of(s, needle)`    | int     | Byte index of `needle`, or `-1`.             |
| `str.repeat(s, n)`           | string  | Repeat `s` `n` times.                        |

```aly
print(str.split("a,b,c", ","))     # [a, b, c]
print(str.index_of("hello", "l"))  # 2
print(str.repeat("ab", 3))         # ababab
```

---

## `sys` — system

| Function          | Returns | Description                                  |
| ----------------- | ------- | -------------------------------------------- |
| `sys.env(name)`   | string  | Environment variable value (empty if unset). |
| `sys.args()`      | vector  | Process arguments.                           |
| `sys.time()`      | int     | Unix timestamp in seconds.                   |
| `sys.platform()`  | string  | OS name (`linux`, `windows`, `macos`, ...).  |
| `sys.cwd()`       | string  | Current working directory.                   |
| `sys.exit(code)`  | —       | Terminate the process with an exit code.     |

```aly
print(sys.platform())
print(sys.cwd())

if sys.env("DEBUG") eq "1" {
    print("debug mode")
}
```

---

## `http` — HTTP client

| Function                            | Returns | Description                                              |
| ----------------------------------- | ------- | -------------------------------------------------------- |
| `http.get(url)`                     | string  | Send a GET request, return the response body.            |
| `http.post(url, body)`              | string  | Send a POST request with JSON body.                      |
| `http.put(url, body)`               | string  | Send a PUT request with JSON body.                       |
| `http.delete(url)`                  | string  | Send a DELETE request.                                   |
| `http.patch(url, body)`             | string  | Send a PATCH request with JSON body.                     |
| `http.request(method, url, body)`   | string  | Send any HTTP method (`body` can be empty).              |
| `http.status_code(url)`             | int     | Return only the HTTP status code.                        |
| `http.head(url)`                    | string  | Return JSON with `status` and `headers`.                 |

All request functions panic on failure (connection error, non-2xx status).

```aly
# GET example
let response = http.get("https://api.example.com/data")
print(response)

# POST example
http.post("https://api.example.com/create", '{"name":"Aly"}')

# Status code check
let status = http.status_code("https://example.com")
if status eq 200 {
    print("OK")
}

# Inspect headers
let info = http.head("https://example.com")
print(info)   # {"status": 200, "headers": {...}}
```

---

## Math & numeric helpers

These are global functions (no namespace) — see
[Functions](08-functions.md#built-in-functions):

`pow`, `sqrt`, `random`, `round`, `roundUp`, `roundDown`, `to_fixed`.
