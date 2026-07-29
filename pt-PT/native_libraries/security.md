# Módulo de Segurança em Aly

O módulo `security` fornece JWT, OAuth2, OpenID Connect, TLS, hash, assinaturas, ACME (Let's Encrypt) e sanitização de entrada.

---

## 1. JWT (JSON Web Tokens)

```aly
import security

let token = security.jwt.encode({user: "alice", role: "admin"}, "secret-key")
print(token)

let decoded = security.jwt.decode(token, "secret-key")
print(decoded.user)   # Resultado: alice
```

---

## 2. OAuth2

```aly
import security

let url = security.oauth2.authorize_url("client-id", "redirect-uri", "scope")
let token = security.oauth2.exchange_code("client-id", "client-secret", "auth-code", "redirect-uri")
let refresh = security.oauth2.refresh_token("client-id", "client-secret", token.refresh_token)
```

---

## 3. OpenID Connect (OIDC)

```aly
import security

let id_token = security.oidc.verify(token, "issuer", "client-id")
print(id_token.sub)     # Identificador do sujeito
print(id_token.email)   # Email do utilizador
```

---

## 4. TLS / SSL

```aly
import security

let ctx = security.tls.create_context("cert.pem", "key.pem")
let conn = security.tls.connect(ctx, "example.com", 443)
security.tls.send(conn, "GET / HTTP/1.1\r\nHost: example.com\r\n\r\n")
let response = security.tls.recv(conn, 4096)
security.tls.close(conn)
```

---

## 5. Hashes Criptográficos

```aly
import security

let h1 = security.hash.md5("hello")
let h2 = security.hash.sha1("hello")
let h3 = security.hash.sha256("hello")
let h4 = security.hash.sha512("hello")
```

---

## 6. Assinaturas Digitais

```aly
import security

let keypair = security.signature.generate()
let sig = security.signature.sign("mensagem", keypair.private)
let valid = security.signature.verify("mensagem", sig, keypair.public)
print(valid)  # Resultado: true
```

---

## 7. ACME (Let's Encrypt)

```aly
import security

let cert = security.acme.request("example.com", "admin@example.com")
# Obtém e instala automaticamente o certificado TLS
```

---

## 8. Sanitização de Entrada

```aly
import security

let clean = security.sanitize.html("<script>alert('xss')</script>")
print(clean)  # Elimina ou escapa HTML perigoso

let safe = security.sanitize.sql("Robert'; DROP TABLE Students;--")
print(safe)  # Escapa tentativas de injecção SQL
```
