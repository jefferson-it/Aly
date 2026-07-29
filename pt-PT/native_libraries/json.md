# Módulo JSON no Aly

O módulo `json` fornece a análise e serialização de dados JSON.

---

## 1. Analisar JSON

```aly
import json

let data = json.parse('{"name": "Aly", "version": 1.0}')
print(data.name)      # Outputs: Aly
print(data.version)   # Outputs: 1.0
```

O JSON analisado é mapeado para os tipos nativos do Aly:
- JSON object -> objeto Aly
- JSON array -> vetor Aly
- JSON string -> string Aly
- JSON number -> int ou float Aly
- JSON boolean -> bool Aly
- JSON null -> `None` do Aly

---

## 2. Serializar para JSON

```aly
import json

let obj = {name: "Aly", version: 1.0}
let text = json.stringify(obj)
print(text)
# Outputs: {"name": "Aly", "version": 1.0}
```

A função `stringify` converte os valores do Aly de volta para o formato de texto JSON.
