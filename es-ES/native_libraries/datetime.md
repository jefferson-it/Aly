# Utilerías de Fecha y Hora en Aly

Aly proporciona utilerías integradas para el análisis, formato y manipulación de valores de fecha y hora.

---

## 1. Referencia de la API

| Función | Parámetros | Retorno | Descripción |
| ------- | ---------- | ------- | ----------- |
| `datetime_now()` | *ninguno* | `string` | Devuelve la fecha y hora local actual como una cadena ISO-8601 (`"2026-07-29T13:42:24-03:00"`). |
| `datetime_utc()` | *ninguno* | `string` | Devuelve la fecha y hora UTC actual como una cadena ISO-8601 (`"2026-07-29T16:42:24Z"`). |
| `datetime_from_iso(iso_str)` | `iso_str: string` | `string` | Analiza una cadena de fecha y hora ISO-8601. Devuelve `"Invalid ISODate: ..."` en caso de fallo. |
| `datetime_parse(date_str, format)` | `date_str: string`, `format: string` | `string` | Analiza una `date_str` utilizando un `format` personalizado (ej: `"%d/%m/%Y"`). Devuelve una cadena ISO-8601. |
| `datetime_from_timestamp(ts_str)` | `ts_str: string` | `string` | Convierte una marca de tiempo Unix (en segundos) en una cadena de fecha y hora ISO-8601. |
| `datetime_duration(seconds_str)` | `seconds_str: string` | `string` | Produce un valor de `Duration` (representado como una cadena que contiene un entero de segundos). |

---

## 2. Ejemplos

```aly
# Obtener la fecha y hora local actual
print(datetime_now())      # ej: "2026-07-29T13:42:24-03:00"

# Obtener la fecha y hora UTC actual
print(datetime_utc())      # ej: "2026-07-29T16:42:24Z"

# Analizar cadena ISO-8601
print(datetime_from_iso("2026-07-29T13:42:24-03:00"))

# Analizar fecha con formato personalizado
print(datetime_parse("29/07/2026", "%d/%m/%Y"))       # → "2026-07-29T00:00:00+00:00"

# Convertir marca de tiempo Unix a fecha y hora ISO
print(datetime_from_timestamp("1782658944"))

# Crear duración en segundos
print(datetime_duration("3600"))                      # → "3600"
```

---

## 3. Notas de Implementación

- Las funciones de fecha y hora devuelven valores envueltos en cadenas para una serialización y transferencia seguras.
- El análisis y formato de zona horaria se gestionan internamente utilizando la librería de Rust `chrono`, coincidiendo con la zona horaria local del sistema.
- Los errores se devuelven como cadenas descriptivas en lugar de provocar pánicos en tiempo de ejecución.
