# HTTP
Networking layer providing client and server HTTP operations.

| Category | Function | Parameters | Returns | Description |
| ------------------------------------------------- | ---------- | ---------- | ---------- | ------------------------------------------------- |
| **Client – GET** | `http_get(url)` | `url: string` | `string` (quoted) | Perform an HTTP GET request to `url`. Returns the response body as a quoted string, or an empty quoted value on error. |
| **Client – POST** | `http_post(url, body)` | `url: string`, `body: string` | `string` (quoted) | Send a POST request with JSON `body` to `url`. Returns response body as quoted string. |
| **Client – PUT** | `http_put(url, body)` | `url: string`, `body: string` | `string` (quoted) | Send a PUT request with JSON `body` to `url`. |
| **Client – DELETE** | `http_delete(url)` | `url: string` | `string` (quoted) | Send a DELETE request to `url`. |
| **Client – PATCH** | `http_patch(url, body)` | `url: string`, `body: string` | `string` (quoted) | Send a PATCH request with JSON `body` to `url`. |
| **Client – REQUEST** | `http_request(method, url, body)` | `method: string`, `url: string`, `body: string` | `string` (quoted) | Send a request with arbitrary `method` (GET, POST, PUT, DELETE, PATCH). `body` may be empty. Returns response body as quoted string. |
| **Client – STATUS_CODE** | `http_status_code(url)` | `url: string` | `int` (quoted) | Perform a GET request and return the HTTP status code (as integer) in a quoted string. |
| **Client – HEAD** | `http_head(url)` | `url: string` | `string` (quoted) | Send a HEAD request and return a JSON object `"{\"status\": <code>, \"headers\": {...}}"` as a quoted string. |
| **Server – SERVE** | `http_serve(port)` | `port: u16` | `string` (quoted) | Start an HTTP server on `port`. Registeres routes via `http_route`. Runs in a background thread. Returns `"None"`. |
| **Server – ROUTE** | `http_route(method, path, handler)` | `method: string`, `path: string`, `handler: function` | `string` (quoted) | Register a handler for a given HTTP `method` and `path`. `handler` is a function accepting a single string (raw request body) and returning a `Validator`. Called for each incoming request matching the route. |

### Example Usage

```aly
# Simple GET
print(http_get("https://api.example.com/status"))   # → quoted JSON response

# POST with JSON payload
print(http_post(
    "https://api.example.com/submit",
    '{"user":"alice","action":"login"}'
))   # → quoted response

# Register a handler for GET /hello
http_route("GET", "/hello", fn(path, body) {
    print("Received hello request")
    console_log("Hello from server!")
    # Return a plain text response
    console_log("world")
    ok_str("world")
})

# Serve on port 8000 (runs in background)
http_serve(8000)
```

### Remarks
- All client functions are **blocking** and include a 30‑second timeout.
- Errors are logged as `RuntimeError` and result in empty quoted values.
- Server registration must occur before calling `http_serve`.
- The server uses Axum; CORS is permissive (`allow_all`).
- `http_status_code` extracts only the status code; headers are ignored.