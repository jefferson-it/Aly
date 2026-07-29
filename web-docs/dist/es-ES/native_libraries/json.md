# Módulo JSON en Aly

El módulo `json` proporciona análisis y serialización de datos JSON.

---

## 1. Análisis de JSON

```aly
import json

let data = json.parse('{"name": "Aly", "version": 1.0}')
print(data.name)      # Outputs: Aly
print(data.version)   # Outputs: 1.0
```

El JSON analizado se mapea a tipos nativos de Aly:
- Objeto JSON -> Objeto Aly
- Array JSON -> Vector Aly
- Cadena JSON -> Cadena Aly
- Número JSON -> int o float de Aly
- Booleano JSON -> bool de Aly
- JSON null -> `None` de Aly

---

## 2. Serialización a JSON

```aly
import json

let obj = {name: "Aly", version: 1.0}
let text = json.stringify(obj)
print(text)
# Outputs: {"name": "Aly", "version": 1.0}
```

La función `stringify` convierte los valores de Aly de nuevo al formato de texto JSON.