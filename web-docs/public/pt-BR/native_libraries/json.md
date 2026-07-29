# JSON Module in Aly

The `json` module provides parsing and serialization of JSON data.

---

## 1. Parsing JSON

```aly
import json

let data = json.parse('{"name": "Aly", "version": 1.0}')
print(data.name)      # Outputs: Aly
print(data.version)   # Outputs: 1.0
```

Parsed JSON maps to native Aly types:
- JSON object -> Aly object
- JSON array -> Aly vector
- JSON string -> Aly string
- JSON number -> Aly int or float
- JSON boolean -> Aly bool
- JSON null -> Aly `None`

---

## 2. Serializing to JSON

```aly
import json

let obj = {name: "Aly", version: 1.0}
let text = json.stringify(obj)
print(text)
# Outputs: {"name": "Aly", "version": 1.0}
```

The `stringify` function converts Aly values back to JSON text format.