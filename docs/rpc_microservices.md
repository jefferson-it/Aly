# RPC, Microservices & APIs in Aly

Aly provides modules for building RPC servers, microservices, GraphQL APIs, and gRPC services.

---

## 1. HTTP RPC

```aly
import rpc

let server = rpc.create(":8080")
rpc.register(server, "add", fun(a, b) { return a + b })
rpc.register(server, "greet", fun(name) { return "Hello, " + name })
rpc.start(server)
```

Call from a client:
```aly
import rpc

let client = rpc.connect("http://localhost:8080")
let sum = rpc.call(client, "add", [3, 4])
print(sum)  # Outputs: 7
```

---

## 2. Microservices

```aly
import microservices

let service = microservices.create("user-service", 9090)
microservices.health_check(service, "/health")
microservices.register_discovery(service, "consul:8500")
microservices.start(service)
```

---

## 3. Serverless Functions

```aly
import serverless

let handler = serverless.handler(fun(event) {
    return {statusCode: 200, body: "Hello " + event.name}
})

serverless.serve(handler, 8080)
```

---

## 4. GraphQL

```aly
import graphql

let schema = graphql.parse("""
    type Query {
        hello: String
        user(id: Int): User
    }
    type User {
        name: String
        age: Int
    }
""")

let resolvers = {
    hello: fun() { return "World" },
    user: fun(args) { return db.find_user(args.id) }
}

let server = graphql.create_server(schema, resolvers, 4000)
graphql.start(server)
```

---

## 5. gRPC

```aly
import grpc

let stub = grpc.connect("localhost:50051")
let response = grpc.call(stub, "Greeter.SayHello", {name: "Aly"})
print(response.message)

let server = grpc.create_server(":50051")
grpc.register_service(server, "Greeter", {
    SayHello: fun(req) { return {message: "Hello " + req.name} }
})
grpc.start(server)
```

---

## 6. Proxy

```aly
import proxy

let p = proxy.create(":8080", "https://backend:3000")
proxy.on_request(p, fun(req) {
    req.headers["X-Forwarded-For"] = req.remote_addr
})
proxy.start(p)
```