# DateTime Utilities in Aly

Aly provides built-in utilities for parsing, formatting, and manipulating date and time values.

---

## 1. API Reference

| Function | Parameters | Returns | Description |
| -------- | ---------- | ------- | ----------- |
| `datetime_now()` | *none* | `string` | Returns current local date-time as an ISO-8601 string (`"2026-07-29T13:42:24-03:00"`). |
| `datetime_utc()` | *none* | `string` | Returns current UTC date-time as an ISO-8601 string (`"2026-07-29T16:42:24Z"`). |
| `datetime_from_iso(iso_str)` | `iso_str: string` | `string` | Parses an ISO-8601 date-time string. Returns `"Invalid ISODate: ..."` on failure. |
| `datetime_parse(date_str, format)` | `date_str: string`, `format: string` | `string` | Parses a `date_str` using a custom `format` (e.g., `"%d/%m/%Y"`). Returns ISO-8601 string. |
| `datetime_from_timestamp(ts_str)` | `ts_str: string` | `string` | Converts a Unix timestamp (in seconds) to an ISO-8601 date-time string. |
| `datetime_duration(seconds_str)` | `seconds_str: string` | `string` | Produces a `Duration` value (represented as a quoted integer of seconds). |

---

## 2. Examples

```aly
# Retrieve current local time
print(datetime_now())      # e.g., "2026-07-29T13:42:24-03:00"

# Retrieve current UTC time
print(datetime_utc())      # e.g., "2026-07-29T16:42:24Z"

# Parse ISO-8601 string
print(datetime_from_iso("2026-07-29T13:42:24-03:00"))

# Parse date with custom format
print(datetime_parse("29/07/2026", "%d/%m/%Y"))       # → "2026-07-29T00:00:00+00:00"

# Convert Unix timestamp to ISO datetime
print(datetime_from_timestamp("1782658944"))

# Create duration in seconds
print(datetime_duration("3600"))                      # → "3600"
```

---

## 3. Implementation Notes

- Date-time functions return values wrapped as strings for safe serialization and transfer.
- Timezone parsing and formatting are handled under the hood using Rust's `chrono` crate, matching the system's local timezone.
- Errors are returned as descriptive strings instead of causing runtime panics.
