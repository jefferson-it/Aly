# Formato de Dados JOT em Aly

JOT é um formato de dados semelhante a JSON e linguagem de consulta integrada em Aly para manipulação de dados estruturados.

---

## 1. Valores JOT

JOT suporta os seguintes tipos de valores:

- **Nulo**: Ausência de valor
- **Booleano**: `true` ou `false`
- **Inteiro**: Inteiros assinados de 64 bits
- **Ponto Flutuante**: Números de ponto flutuante de 64 bits
- **Cadeia de Caracteres**: Cadeias de texto
- **Array**: Lista ordenada de valores JOT
- **Objecto**: Mapa de chave-valor de valores JOT

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

## 3. Consulta com `pick`

O método `pick` percorre objectos aninhados utilizando uma sequência de chaves:

```aly
let val = data.pick(["details", "author"])
print(val)  # Resultado: "Jefferson"
```

Devolve `None` se qualquer chave na cadeia não existir.

---

## 4. Conversão para Cadeia de Caracteres

```aly
let text = data.to_string()
print(text)
```

Devolve o valor JOT como uma cadeia formatada, indentada para legibilidade.

---

## 5. Tempo de Execução JOT

O tempo de execução JOT (`jot.rs`) fornece utilitários de análise, formatação e conversão. Serve como representação interna para dados estruturados em todo o tempo de execução Aly.
