# Sistema, Entorno y Utilidades del SO en Aly

Aly proporciona librerías para acceder a variables de entorno centrales, verificar configuraciones, leer perfiles de configuración y administrar terminales del sistema.

---

## 1. Perfiles de Entorno (`dotenv`)

Cargue variables `.env` directamente en los enlaces del entorno activo:
```aly
import dotenv

dotenv.load() # Reads .env file in project root

let api_key = dotenv.get("API_KEY")
print("API Key: $api_key")
```

---

## 2. Generación de UUID (`uuid`)

Genere cadenas de texto de identificadores únicos nativamente:
```aly
import uuid

let id = uuid.v4()
print("Generated Unique ID: $id")
```

---

## 3. Control de Terminal (`console`)

Controle salidas, colores y lea entradas estándar de la terminal:
```aly
import console

# Clear terminal screen
console.clear()

# Colored terminal outputs using ANSI colors
console.print_color("Green success text", "green")
console.print_color("Red error message", "red")

# Read user input directly
let name = console.input("Enter name: ")
print("Hello, $name")
```