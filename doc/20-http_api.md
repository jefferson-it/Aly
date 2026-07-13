# HTTP API
Built‑in HTTP server exposing diagnostic and control endpoints.

| Function | Parameters | Returns | Description |
| ------------------------------------------------- | ---------- | ---------- | ------------------------------------------------- |
| `http_api_serve(port)` | `port: u16` | `string` (quoted) | Launch a background HTTP server on `localhost:<port>`. Endpoints: `/api/vars` (list runtime variables), `/api/doc` (JSON schema of variables), `/api/run` (execute a script snippet). Returns `"None"` and runs asynchronously. |

The server is intended for introspection and remote control of the Aly‑Lang runtime. It uses the embedded Axum router; all responses are JSON‑encoded.

### Example

```aly
# Start server on port 8080
http_api_serve(8080)

# List variables (GET /api/vars)
print(http_get("http://localhost:8080/api/vars"))

# Get documentation (GET /api/doc)
print(http_get("http://localhost:8080/api/doc"))

# Run a snippet (POST /api/run)
print(http_post("http://localhost:8080/api/run", '{"code":"print(1+1)"}'))
```

### Notes
- The server does **not** block the caller; it spawns a Tokio runtime internally.
- Endpoints panic (via internal error handling) if request payloads are malformed.
- CORS is not enabled; clients must run on the same host or use a reverse proxy.