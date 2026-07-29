# Módulo JSON no Aly

O módulo `json` fornece análise e serialização de dados JSON.

---

## 1. Analisando JSON

```aly
import json

let data = json.parse('{"name": "Aly", "version": 1.0}')
print(data.name)      # Outputs: Aly
print(data.version)   # Outputs: 1.0
```

JSON analisado é mapeado para os tipos nativos do Aly:
- JSON object -> objeto Aly
- JSON array -> vetor Aly
- JSON string -> string Aly
- JSON number -> int ou float Aly
- JSON boolean -> bool Aly
- JSON null -> `None` Aly

---

## 2. Serializando para JSON

```aly
import json

let obj = {name: "Aly", version: 1.0}
let text = json.stringify(obj)
print(text)
# Outputs: {"name": "Aly", "version": 1.0}
```

A função `stringify` converte valores Aly de volta para o formato de texto JSON.
