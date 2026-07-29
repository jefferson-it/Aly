# Módulo JSON em Aly

O módulo `json` fornece análise e serialização de dados JSON.

---

## 1. Análise de JSON

```aly
import json

let data = json.parse('{"name": "Aly", "version": 1.0}')
print(data.name)      # Resultado: Aly
print(data.version)   # Resultado: 1.0
```

JSON analisado mapeia para tipos nativos Aly:
- Objecto JSON -> Objecto Aly
- Array JSON -> Vetor Aly
- Cadeia de caracteres JSON -> Cadeia de caracteres Aly
- Número JSON -> Inteiro ou ponto flutuante Aly
- Booleano JSON -> Booleano Aly
- Nulo JSON -> `None` Aly

---

## 2. Serialização para JSON

```aly
import json

let obj = {name: "Aly", version: 1.0}
let text = json.stringify(obj)
print(text)
# Resultado: {"name": "Aly", "version": 1.0}
```

A função `stringify` converte valores Aly de volta para o formato textual JSON.
