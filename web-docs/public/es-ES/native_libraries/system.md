# Sistema, Entorno y Utilidades de SO en Aly

Aly proporciona bibliotecas para acceder a variables de entorno centrales, verificar configuraciones, leer perfiles de configuración y gestionar terminales del sistema.

---

## 1. Perfiles de Entorno (`dotenv`)

Carga variables `.env` directamente en los enlaces de entorno activos:
```aly
import dotenv

dotenv.load() # Lee el archivo .env en la raíz del proyecto

let api_key = dotenv.get("API_KEY")
print("API Key: $api_key")
```

---

## 2. Generación de UUID (`uuid`)

Generar cadenas de identificadores únicos de forma nativa:
```aly
import uuid

let id = uuid.v4()
print("Generated Unique ID: $id")
```

---

## 3. Control de Terminal (`console`)

Controlar salidas, colores y leer entradas de terminal estándar:
```aly
import console

# Limpiar pantalla de terminal
console.clear()

# Salidas de terminal coloreadas usando colores ANSI
console.print_color("Green success text", "green")
console.print_color("Red error message", "red")

# Leer entrada del usuario directamente
let name = console.input("Enter name: ")
print("Hello, $name")
```