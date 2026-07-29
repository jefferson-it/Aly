# Programação de Redes em Aly

Aly fornece módulos nativos robustos para operações de rede, desde sockets TCP/UDP de baixo nível até pedidos de cliente HTTP de alto nível e servidores de API.

---

## 1. Cliente e Servidor HTTP (`http`)

O módulo `http` permite-lhe criar servidores de API e executar pedidos de cliente HTTP.

### Cliente HTTP (com suporte de curl)
Efectuar pedidos HTTP GET e POST:
```aly
import http

let response = http.get("https://api.github.com/repos/jefferson-it/aly")
print("Estado da Resposta: " + response.status)
print("Corpo da Resposta: " + response.body)
```

### Servidor HTTP
Criar um servidor de microserviço:
```aly
import http

let server = http.server(8080)

http.get(server, "/users", fun(req, res) {
    res.send("Lista de utilizadores")
})

http.post(server, "/users", fun(req, res) {
    let body = req.body
    res.status(201).send("Criado: " + body)
})

http.start(server)
```

---

## 2. Sockets de Baixo Nível (`net`)

Para implementações de protocolos personalizados, Aly fornece wrappers de ligação TCP e UDP de baixo nível.

### Servidor TCP
```aly
import net

let socket = net.tcp_listen("127.0.0.1:9000")

loop {
    let client = net.tcp_accept(socket)
    net.tcp_send(client, "Olá cliente TCP!")
    net.tcp_close(client)
}
```

### Cliente TCP
```aly
import net

let conn = net.tcp_connect("127.0.0.1:9000")
let data = net.tcp_recv(conn, 1024)
print("Recebido: ")
net.tcp_close(conn)
```

---

## 3. WebSocket

Comunicação bidirecional em tempo real:
```aly
import websocket

let ws = websocket.connect("ws://echo.websocket.org")
websocket.send(ws, "Olá WebSocket")
let reply = websocket.recv(ws)
print(reply)
websocket.close(ws)
```

---

## 4. MQTT

Protocolo de mensagem leve de publicação/subscrição:
```aly
import mqtt

let client = mqtt.connect("broker.hivemq.com", 1883)
mqtt.subscribe(client, "aly/test")
mqtt.publish(client, "aly/test", "Olá MQTT")

loop {
    let msg = mqtt.poll(client)
    if msg ne None {
        print("Recebido: " + msg.payload)
    }
}
```

> **Nota**: MQTT requer a funcionalidade `mqtt` do Cargo.

---

## 5. TLS / SSL

Ligações de sockets seguras:
```aly
import tls

let conn = tls.connect("example.com", 443)
tls.send(conn, "GET / HTTP/1.1\r\nHost: example.com\r\n\r\n")
let resp = tls.recv(conn, 4096)
print(resp)
tls.close(conn)
```
