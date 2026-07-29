# Utilitários de Data e Hora no Aly

O Aly fornece utilitários integrados para análise, formatação e manipulação de valores de data e hora.

---

## 1. Referência da API

| Função | Parâmetros | Retorna | Descrição |
| ------ | ---------- | ------- | ----------- |
| `datetime_now()` | *nenhum* | `string` | Retorna a data e hora local atual como uma string ISO-8601 (`"2026-07-29T13:42:24-03:00"`). |
| `datetime_utc()` | *nenhum* | `string` | Retorna a data e hora UTC atual como uma string ISO-8601 (`"2026-07-29T16:42:24Z"`). |
| `datetime_from_iso(iso_str)` | `iso_str: string` | `string` | Analisa uma string de data e hora ISO-8601. Retorna `"Invalid ISODate: ..."` em caso de falha. |
| `datetime_parse(date_str, format)` | `date_str: string`, `format: string` | `string` | Analisa uma `date_str` utilizando um `format` personalizado (ex: `"%d/%m/%Y"`). Retorna uma string ISO-8601. |
| `datetime_from_timestamp(ts_str)` | `ts_str: string` | `string` | Converte um timestamp Unix (em segundos) numa string de data e hora ISO-8601. |
| `datetime_duration(seconds_str)` | `seconds_str: string` | `string` | Produz um valor de `Duration` (representado como uma string contendo um inteiro de segundos). |

---

## 2. Exemplos

```aly
# Obter data e hora local atual
print(datetime_now())      # ex: "2026-07-29T13:42:24-03:00"

# Obter data e hora UTC atual
print(datetime_utc())      # ex: "2026-07-29T16:42:24Z"

# Analisar string ISO-8601
print(datetime_from_iso("2026-07-29T13:42:24-03:00"))

# Analisar data com formato personalizado
print(datetime_parse("29/07/2026", "%d/%m/%Y"))       # → "2026-07-29T00:00:00+00:00"

# Converter timestamp Unix para data e hora ISO
print(datetime_from_timestamp("1782658944"))

# Criar duração em segundos
print(datetime_duration("3600"))                      # → "3600"
```

---

## 3. Notas de Implementação

- As funções de data e hora retornam valores envolvtos em strings para serialização e transferência seguras.
- A análise e a formatação de fuso horário são tratadas internamente utilizando a crate Rust `chrono`, correspondendo ao fuso horário local do sistema.
- Os erros são retornados como strings descritivas em vez de causar pânicos em tempo de execução.
