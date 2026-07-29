# Programación de Redes en Aly

Aly proporciona módulos nativos robustos para operaciones de red, desde sockets TCP/UDP de bajo nivel hasta solicitudes de cliente HTTP de alto nivel y servidores de API.

---

## 1. Cliente y Servidor HTTP (`http`)

El módulo `http` le permite iniciar servidores de API y realizar solicitudes de cliente HTTP.

### Cliente HTTP (respaldado por curl)
Realiza solicitudes HTTP GET y POST:
```aly
import http

let response = http.get("https://api.github.com/repos/jefferson-it/aly")
print("Response Status: " + response.status)
print("Response Body: " + response.body)
```

### Servidor HTTP
Crear un servidor microservicio:
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

## 2. Sockets de Bajo Nivel (`net`)

Para implementaciones de protocolos personalizados, Aly proporciona envoltorios de conexión TCP y UDP de bajo nivel.

### Servidor TCP
```aly
import net

let socket = net.tcp_listen("127.0.0.1:9000")

loop {
    let client = net.tcp_accept(socket)
    net.tcp_send(client, "Hello TCP client!")
    net.tcp_close(client)
}
```

### Cliente TCP
```aly
import net

let conn = net.tcp_connect("127.0.0.1:9000")
let data = net.tcp_recv(conn, 1024)
print("Received: $data")
net.tcp_close(conn)
```

---

## 3. WebSocket

Comunicación bidireccional en tiempo real:
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

Protocolo de mensajería ligero de publicación/suscripción:
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

> **Nota**: MQTT requiere la característica `mqtt` de Cargo.

---

## 5. TLS / SSL

Conexiones de socket seguras:
```aly
import tls

let conn = tls.connect("example.com", 443)
tls.send(conn, "GET / HTTP/1.1\r\nHost: example.com\r\n\r\n")
let resp = tls.recv(conn, 4096)
print(resp)
tls.close(conn)
```