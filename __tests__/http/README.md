# HTTP Server Examples

This directory contains examples for Aly's HTTP server functionality (Express.js-style).

## Examples

### 1. Basic Server (`basic_server.aly`)
Simple server with basic GET and POST routes.

**Run:**
```bash
cargo run -- run __tests__/http/basic_server.aly
```

**Test:**
```bash
curl http://localhost:3000
curl http://localhost:3000/hello/World
curl -X POST http://localhost:3000/api/data -d '{"test":"value"}'
```

---

### 2. REST API (`rest_api.aly`)
Full REST API with in-memory storage, demonstrating CRUD operations.

**Run:**
```bash
cargo run -- run __tests__/http/rest_api.aly
```

**Test:**
```bash
# Create user
curl -X POST http://localhost:3000/api/users -d '{"name":"Alice","email":"alice@example.com"}'

# Get all users
curl http://localhost:3000/api/users

# Get single user
curl http://localhost:3000/api/users/1

# Update user
curl -X PUT http://localhost:3000/api/users/1 -d '{"name":"Alice Updated"}'

# Delete user
curl -X DELETE http://localhost:3000/api/users/1
```

---

### 3. Middleware (`middleware.aly`)
Demonstrates middleware pattern for logging, CORS, and auth.

**Run:**
```bash
cargo run -- run __tests__/http/middleware.aly
```

---

## API Reference

### Server Setup

- `http.serve(port)` - Start server on specified port
- `http.listen(callback)` - Begin listening (blocking)

### Routing

- `http.get(path, handler)` - Register GET route
- `http.post(path, handler)` - Register POST route
- `http.put(path, handler)` - Register PUT route
- `http.delete(path, handler)` - Register DELETE route
- `http.patch(path, handler)` - Register PATCH route

### Handler Function

Handler receives a request object:
```aly
fun handler(req) {
    # req.method - HTTP method
    # req.path - Request path
    # req.body - Request body
    # req.get_header(name) - Get request header
    "Response body"
}
```

### Middleware

- `http.use(middleware)` - Apply middleware to all routes

### Static Files

- `http.static(route_prefix, directory)` - Serve static files
