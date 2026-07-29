# Network Programming in Aly

Aly provides robust native modules for network operations, from low-level raw TCP/UDP sockets to high-level HTTP client requests and API servers.

---

## 1. HTTP Client & Server (`http`)

The `http` module allows you to spin up API servers and perform HTTP client requests.

### HTTP Client (curl-backed)
Perform HTTP GET and POST requests:
```aly
import http

let response = http.get("https://api.github.com/repos/jefferson-it/aly")
print("Response Status: " + response.status)
print("Response Body: " + response.body)
```

### HTTP Server
Create a microservice server:
```aly
import http

let server = http.server(8080)

http.get(server, "/users", fun(req, res) {
    res.send("List of users")
})

http.post(server, "/users", fun(req, res) {
    let body = req.body
    res.status(201).send("Created: " + body)
})

http.start(server)
```

---

## 2. Low-Level Sockets (`net`)

For custom protocol implementations, Aly provides low-level TCP and UDP connection wrappers.

### TCP Server
```aly
import net

let socket = net.tcp_listen("127.0.0.1:9000")

loop {
    let client = net.tcp_accept(socket)
    net.tcp_send(client, "Hello TCP client!")
    net.tcp_close(client)
}
```

### TCP Client
```aly
import net

let conn = net.tcp_connect("127.0.0.1:9000")
let data = net.tcp_recv(conn, 1024)
print("Received: $data")
net.tcp_close(conn)
```

---

## 3. WebSocket

Real-time bidirectional communication:
```aly
import websocket

let ws = websocket.connect("ws://echo.websocket.org")
websocket.send(ws, "Hello WebSocket")
let reply = websocket.recv(ws)
print(reply)
websocket.close(ws)
```

---

## 4. MQTT

Lightweight publish/subscribe messaging protocol:
```aly
import mqtt

let client = mqtt.connect("broker.hivemq.com", 1883)
mqtt.subscribe(client, "aly/test")
mqtt.publish(client, "aly/test", "Hello MQTT")

loop {
    let msg = mqtt.poll(client)
    if msg ne None {
        print("Received: " + msg.payload)
    }
}
```

> **Note**: MQTT requires the `mqtt` Cargo feature.

---

## 5. TLS / SSL

Secure socket connections:
```aly
import tls

let conn = tls.connect("example.com", 443)
tls.send(conn, "GET / HTTP/1.1\r\nHost: example.com\r\n\r\n")
let resp = tls.recv(conn, 4096)
print(resp)
tls.close(conn)
```