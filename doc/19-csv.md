# CSV
Parse comma‑separated values into Aly‑Lang objects.

| Function | Parameters | Returns | Description |
| ------------------------------------------------- | ---------- | ---------- | ------------------------------------------------- |
| `csv_parse(input_string)` | `input: string` | `string` (quoted) | Parse CSV data from `input`. The first row is interpreted as headers and maps to object properties. Returns a quoted representation of a vector of objects. Errors produce an empty result and a runtime warning. |

```aly
let data = csv_parse('name,age,city\nAlice,30,Brazil\nBob,25,Canada')
print(data)   # → "[{name: "Alice", age: "30", city: "Brazil"}, {name: "Bob", age: "25", city: "Canada"}]"
```

### Notes
- Headers are used as property names for each row’s object.
- Fields are stored as strings (`ValueData::String`).
- The function tolerates flexible quoting and ignores extra whitespace.