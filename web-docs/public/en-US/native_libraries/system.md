# System, Environment, and OS Utilities in Aly

Aly provides libraries to access core environment variables, check configurations, read configuration profiles, and manage system terminals.

---

## 1. Environment Profiles (`dotenv`)

Load `.env` variables directly into active environment bindings:
```aly
import dotenv

dotenv.load() # Reads .env file in project root

let api_key = dotenv.get("API_KEY")
print("API Key: $api_key")
```

---

## 2. UUID Generation (`uuid`)

Generate unique identifier strings natively:
```aly
import uuid

let id = uuid.v4()
print("Generated Unique ID: $id")
```

---

## 3. Terminal Control (`console`)

Control outputs, colors, and read standard terminal inputs:
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
