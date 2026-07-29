# JOT Data Format in Aly

JOT is a JSON-like data format and query language built into Aly for structured data manipulation.

---

## 1. JOT Values

JOT supports the following value types:

- **Null**: Absence of value
- **Boolean**: `true` or `false`
- **Integer**: 64-bit signed integers
- **Float**: 64-bit floating-point numbers
- **String**: Text strings
- **Array**: Ordered list of JOT values
- **Object**: Key-value map of JOT values

---

## 2. JOT Syntax

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

## 3. Querying with `pick`

The `pick` method traverses nested objects using a sequence of keys:

```aly
let val = data.pick(["details", "author"])
print(val)  # Outputs: "Jefferson"
```

Returns `None` if any key in the chain does not exist.

---

## 4. Converting to String

```aly
let text = data.to_string()
print(text)
```

Outputs the JOT value as a formatted string, indented for readability.

---

## 5. JOT Runtime

The JOT runtime (`jot.rs`) provides parsing, formatting, and conversion utilities. It serves as the internal representation for structured data throughout the Aly runtime.