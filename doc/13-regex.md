# Regex
Documentation for Aly‑Lang’s regular expression utilities in the standard library.

| Function | Parameters | Returns | Description |
| ------------------------------------------------- | ---------- | ---------- | ------------------------------------------------- |
| `regex_match(pattern, text)` | `pattern: string`, `text: string` | `string` | Test whether `pattern` matches `text`. Returns a quoted JSON‑like string containing capture groups or `"None"` if no match. |
| `regex_test(pattern, text)` | `pattern: string`, `text: string` | `string` | Boolean test – returns `"true"` if the pattern matches, otherwise `"false"`. |
| `regex_replace(pattern, text, replacement)` | `pattern: string`, `text: string`, `replacement: string` | `string` | Replace all occurrences of `pattern` in `text` with `replacement`. Returns the modified string as a quoted result. |

All functions accept a regular expression syntax compatible with Rust’s `regex` crate. Patterns are validated at runtime; an invalid pattern triggers a runtime error and returns `"None"` (or `false` for `regex_test`).

```aly
# Basic match
let caps = regex_match("[0-9]+", "abc123def")
print(caps)   # → "123"

# Test match
let ok = regex_test("^[a-z]+$", "hello")
print(ok)     # → "true"

# Replace
let new = regex_replace("cat", "the cat sat on the mat", "cat", "dog")
print(new)    # → "dog the dog sat on the mat"
```

### Error handling
- If the regular expression is malformed, the function panics with a `RuntimeError` indicating the invalid pattern.
- When no captures are found, `regex_match` returns `"None"`.

### Notes
- The functions operate on UTF‑8 strings; byte indices are used internally.
- Capture groups are returned in the order they appear; they are accessible via indexing in downstream code.