# Net
Networking primitives – TCP/UDP sockets, WebSocket endpoints, and simple servers.

| Function | Parameters | Returns | Description |
| ------------------------------------------------- | ---------- | ---------- | ------------------------------------------------- |
| `net_tcp_send(host, port, data)` | `host: string`, `port: string`, `data: string` | `string` (quoted) | Open a TCP connection to `host:port`, write `data`, read the server’s response, and return it as a quoted string. Errors yield an empty quoted value. |
| `net_tcp_serve(port, response)` | `port: string`, `response: string` | `string` (quoted) | Bind to `port` and serve a single incoming connection, sending `response` back. Blocks the thread until a client connects. |
| `net_tcp_listen(port, handler_name)` | `port: string`, `handler_name: string` | `string` (quoted) | Open a listening TCP socket on `port`. For each incoming connection, invoke a user‑defined function named `handler_name` (found via reflection) with the client data as argument. Designed for simple request‑handler loops. |
| `net_udp_send(host, port, data)` | `host: string`, `port: string`, `data: string` | `string` (quoted) | Send a UDP datagram to `host:port`. Returns `"None"` regardless of success (errors are logged). |
| `net_udp_recv(port, size)` | `port: string`, `size: string` | `object` (quoted) | Bind a UDP socket to `port`, receive up to `size` bytes, and return an object with fields `data`, `addr`, and `port`. |
| `net_ws_connect(url, data)` | `url: string`, `data: string` | `string` (quoted) | Connect to a WebSocket at `url`, send `data` as text, read the first response message, close the socket, and return the response as a quoted string. |
| `net_ws_server(port, handler_name)` | `port: string`, `handler_name: string` | `string` (quoted) | Bind a WebSocket server on `port`. For each incoming connection, invoke the user‑defined function `handler_name` for each received text message, executing the body as Aly‑Lang code. Errors are logged; the server runs until the client closes. |

### Usage Example

```aly
# Simple TCP echo
print(net_tcp_send("example.com", "1234", "hello"))   # → quoted response

# Serve a static page on port 8080
net_tcp_serve("8080", "text/plain;charset=utf-8\nHello World")

# Listen on port 9000 and execute handler function `on_req`
net_tcp_listen("9000", "on_req")

# UDP broadcast
net_udp_send("255.255.255.255", "9999", "ping")

# UDP receive
let msg = net_udp_recv("1234", "1024")
print(msg)   # → quoted object {data: "...", addr: "192.168.1.5", port: "54321"}

# WebSocket client
print(net_ws_connect("ws://localhost:8080/ws", "greeting"))

# WebSocket server (calls handler function `ws_handler`)
net_ws_server("8080", "ws_handler")
```

### Notes
- All functions are **blocking** unless otherwise noted (e.g., `net_tcp_send` reads until a response is received, with a 5‑second timeout).
- Errors are logged via `RuntimeError` messages and result in empty quoted values.
- WebSocket handlers receive the message as a parameter named by `handler_name`.
- The server‑side functions spawn lightweight threads internally; they do not block the entire runtime.