# Security Module in Aly

The `security` module provides JWT, OAuth2, OpenID Connect, TLS, hashing, signatures, ACME (Let's Encrypt), and input sanitization.

---

## 1. JWT (JSON Web Tokens)

```aly
import security

let token = security.jwt.encode({user: "alice", role: "admin"}, "secret-key")
print(token)

let decoded = security.jwt.decode(token, "secret-key")
print(decoded.user)   # Outputs: alice
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
print(id_token.sub)     # Subject identifier
print(id_token.email)   # User email
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

## 5. Cryptographic Hashing

```aly
import security

let h1 = security.hash.md5("hello")
let h2 = security.hash.sha1("hello")
let h3 = security.hash.sha256("hello")
let h4 = security.hash.sha512("hello")
```

---

## 6. Digital Signatures

```aly
import security

let keypair = security.signature.generate()
let sig = security.signature.sign("message", keypair.private)
let valid = security.signature.verify("message", sig, keypair.public)
print(valid)  # Outputs: true
```

---

## 7. ACME (Let's Encrypt)

```aly
import security

let cert = security.acme.request("example.com", "admin@example.com")
# Automatically obtains and installs TLS certificate
```

---

## 8. Input Sanitization

```aly
import security

let clean = security.sanitize.html("<script>alert('xss')</script>")
print(clean)  # Strips or escapes dangerous HTML

let safe = security.sanitize.sql("Robert'; DROP TABLE Students;--")
print(safe)  # Escapes SQL injection attempts
```