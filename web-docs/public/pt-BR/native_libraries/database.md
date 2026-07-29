# Módulo de Banco de Dados em Aly

Aly fornece drivers de banco de dados embutidos para SQLite, PostgreSQL, MySQL e Redis através do módulo `db`.

---

## 1. SQLite

```aly
import db

let conn = db.sqlite.open("database.sqlite")
db.sqlite.execute(conn, "CREATE TABLE IF NOT EXISTS users (id INT, name TEXT)")
db.sqlite.execute(conn, "INSERT INTO users VALUES (1, 'Alice')")

let rows = db.sqlite.query(conn, "SELECT * FROM users")
loop let i = 0; i lt rows.len; i = i + 1 {
    print(rows[i].name)
}
db.sqlite.close(conn)
```

---

## 2. PostgreSQL

```aly
import db

let conn = db.postgres.connect("host=localhost dbname=test user=postgres")
db.postgres.execute(conn, "INSERT INTO users VALUES ($1, $2)", [1, "Bob"])
let results = db.postgres.query(conn, "SELECT * FROM users")
db.postgres.close(conn)
```

---

## 3. MySQL

```aly
import db

let conn = db.mysql.connect("localhost", "root", "password", "mydb")
db.mysql.execute(conn, "CREATE TABLE items (id INT PRIMARY KEY, name VARCHAR(100))")
db.mysql.close(conn)
```

---

## 4. Redis

```aly
import db

let redis = db.redis.connect("127.0.0.1", 6379)
db.redis.set(redis, "key", "value")
let val = db.redis.get(redis, "key")
print(val)
db.redis.close(redis)
```

---

## 5. Pool de Conexões

O módulo de banco de dados suporta pool de conexões através do sistema `registry`, permitindo reutilização de conexões de banco de dados entre requisições.
