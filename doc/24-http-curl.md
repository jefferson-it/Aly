# HTTP Client (curl)
Fetch-style HTTP client providing a flexible interface for making HTTP requests.

| Category | Function | Parameters | Returns | Description |
| ------------------------------------------------- | ---------- | ---------- | ---------- | ------------------------------------------------- |
| **Object** | `curl` | — | `object` | HTTP client object with methods for making requests. |
| **GET** | `curl.GET(url)` | `url: string` | `string` (quoted) | Perform an HTTP GET request. |
| **POST** | `curl.POST(url, body)` | `url: string`, `body: string` | `string` (quoted) | Send a POST request with JSON body. |
| **PUT** | `curl.PUT(url, body)` | `url: string`, `body: string` | `string` (quoted) | Send a PUT request with JSON body. |
| **DELETE** | `curl.DELETE(url)` | `url: string` | `string` (quoted) | Send a DELETE request. |
| **PATCH** | `curl.PATCH(url, body)` | `url: string`, `body: string` | `string` (quoted) | Send a PATCH request with JSON body. |
| **HEAD** | `curl.HEAD(url)` | `url: string` | `string` (quoted) | Send a HEAD request. |
| **STATUS_CODE** | `curl.status_code(url)` | `url: string` | `int` (quoted) | Perform a GET request and return only the HTTP status code as an integer. |
| **JSON** | `curl.json(url, body)` | `url: string`, `body: string` | `string` (quoted) | Send a POST request with JSON body and return pretty-printed JSON response. |

### Example Usage

```aly
# Simple GET request
response = curl.GET("https://api.example.com/users")
print(response)

# POST with JSON body
body = '{"username":"alice","password":"secret"}'
response = curl.POST("https://api.example.com/login", body)

# Check status code only
status = curl.status_code("https://api.example.com/health")

# JSON helper (POST + pretty response)
data = '{"id":1}'
response = curl.json("https://api.example.com/data", data)
```

### Shortcut

```aly
# Direct call acts as GET
response = curl("https://api.example.com/status")
```

### Remarks
- All requests are **blocking** and include a 30‑second timeout.
- Errors are logged as `RuntimeError` and return empty quoted strings.
- Request and response bodies are handled as raw strings; JSON encoding/decoding is the caller's responsibility.
- The client uses `reqwest` with `rustls-tls`.
