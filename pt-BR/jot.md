# Formato de Dados JOT no Aly

JOT é um formato de dados e linguagem de consulta semelhante ao JSON, integrado ao Aly para manipulação de dados estruturados.

---

## 1. Valores JOT

O JOT suporta os seguintes tipos de valores:

- **Null**: Ausência de valor
- **Boolean**: `true` ou `false`
- **Integer**: Inteiros com sinal de 64 bits
- **Float**: Números de ponto flutuante de 64 bits
- **String**: Cadeias de texto (strings)
- **Array**: Lista ordenada de valores JOT
- **Object**: Mapa de chave-valor de valores JOT

---

## 2. Sintaxe JOT

```aly
let data = {
    "name": "Aly",
    "version": 1.0,
    "tags": ["dynamic", "fun"],
    "details": {
        "author": "Jefferson",
        "year": 2025
    }
}
```

---

## 3. Consultando com `pick`

O método `pick` percorre objetos aninhados usando uma sequência de chaves:

```aly
let val = data.pick(["details", "author"])
print(val)  # Outputs: "Jefferson"
```

Retorna `None` se qualquer chave na cadeia não existir.

---

## 4. Convertendo para String

```aly
let text = data.to_string()
print(text)
```

Gera o valor JOT como uma string formatada, recuada (indentada) para legibilidade.

---

## 5. Runtime do JOT

O runtime do JOT (`jot.rs`) fornece utilitários de parsing, formatação e conversão. Ele serve como a representação interna para dados estruturados em todo o runtime do Aly.