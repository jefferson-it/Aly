# Programação de Rede no Aly

O Aly fornece módulos nativos robustos para operações de rede, desde sockets TCP/UDP brutos de baixo nível até solicitações de cliente HTTP de alto nível e servidores de API.

---

## 1. Cliente e Servidor HTTP (`http`)

O módulo `http` permite que você inicie servidores de API e execute solicitações de cliente HTTP.

### Cliente HTTP (apoiado por curl)
Execute solicitações HTTP GET e POST:
```aly
import http

let response = http.get("https://api.github.com/repos/jefferson-it/aly")
print("Response Status: " + response.status)
print("Response Body: " + response.body)
```

### Servidor HTTP
Crie um servidor de microsserviços:
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

## 2. Sockets de Baixo Nível (`net`)

Para implementações de protocolo personalizado, o Aly fornece invólucros de conexão TCP e UDP de baixo nível.

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
print("Received: ")
net.tcp_close(conn)
```

---

## 3. WebSocket

Comunicação bidirecional em tempo real:
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

Protocolo de mensagens de publicação/assinatura leve:
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

> **Nota**: O MQTT requer o recurso do Cargo `mqtt`.

---

## 5. TLS / SSL

Conexões de socket seguras:
```aly
import tls

let conn = tls.connect("example.com", 443)
tls.send(conn, "GET / HTTP/1.1\r\nHost: example.com\r\n\r\n")
let resp = tls.recv(conn, 4096)
print(resp)
tls.close(conn)
```