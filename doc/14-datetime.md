# DateTime
Reference for Aly‑Lang’s date and time utilities in the standard library.

| Function | Parameters | Returns | Description |
| ------------------------------------------------- | ---------- | ---------- | ------------------------------------------------- |
| `datetime_now()` | *none* | `string` (quoted) | Current local date‑time as an ISO‑8601 string (`"2025-03-14T12:34:56-03:00"`). |
| `datetime_utc()` | *none* | `string` (quoted) | Current UTC date‑time as an ISO‑8601 string (`"2025-03-14T15:34:56Z"`). |
| `datetime_from_iso(iso_str)` | `iso_str: string` | `string` | Parse an ISO‑8601 date‑time (`"2025-03-14T12:34:56-03:00"`). Returns `"Invalid ISODate: …"` on failure. |
| `datetime_parse(date_str, format)` | `date_str: string`, `format: string` | `string` | Parse `date_str` according to `format` (e.g., `"%Y-%m-%d"`). Returns the resulting datetime as a quoted string or an error message. |
| `datetime_from_timestamp(ts_str)` | `ts_str: string` | `string` | Convert a Unix timestamp (seconds) to an ISO‑8601 datetime. Invalid timestamps yield `"Invalid timestamp"`. |
| `datetime_duration(seconds_str)` | `seconds_str: string` | `int` (quoted) | Produce a `Duration` value (in seconds) from the given string. |

All date‑time values are represented as quoted strings; `Duration` is a quoted integer.

```aly
# Current time
print(datetime_now())      # → "2025-03-14T12:34:56-03:00"

# UTC time
print(datetime_utc())      # → "2025-03-14T15:34:56Z"

# ISO parsing
print(datetime_from_iso('2025-03-14T12:34:56-03:00'))  # → "2025-03-14T12:34:56-03:00"

# Custom format parsing
print(datetime_parse('14/03/2025', '%d/%m/%Y'))       # → "2025-03-14T00:00:00+00:00"

# Timestamp conversion
print(datetime_from_timestamp('1740123456'))          # → "2025-02-20T03:57:36Z"

# Duration creation
print(datetime_duration('3600'))                      # → "3600"
```

### Notes
- Functions that return a `DateTime` wrap it in a quoted string for downstream consumption.
- Parsing errors return human‑readable error messages rather than panicking.
- All time handling uses the Rust `chrono` crate with system local timezone and UTC offsets.