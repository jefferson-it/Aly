# Console
I/O functions for printing and styled output.

| Function | Parameters | Returns | Description |
| ------------------------------------------------- | ---------- | ---------- | ------------------------------------------------- |
| `console_log(text)` | `text: string` | `string` (quoted) | Print `text` to stdout with default styling. |
| `console_warn(text)` | `text: string` | `string` (quoted) | Print `text` in yellow. |
| `console_error(text)` | `text: string` | `string` (quoted) | Print `text` in red to stderr. |
| `console_info(text)` | `text: string` | `string` (quoted) | Print `text` in blue. |
| `console_success(text)` | `text: string` | `string` (quoted) | Print `text` in green. |
| `console_progress(current, total, ?label?)` | `current: int`, `total: int`, `label?: string` | `string` (quoted) | Render a progress bar; optional label defaults to `"Progress"`; bar turns green when complete, cyan otherwise. |

All functions return a quoted `"None"` string; output is written directly to the terminal.

```aly
console_log("starting…")           # prints starting…
console_warn("deprecated!")        # yellow warning
console_error("boom!")             # red error
console_info("info message")       # blue info
console_success("done!")           # green success

# Progress bar
console_progress("27", "54", "Downloading")   # indeterminate until finished
```

### Notes
- Styling uses the Rust `crossterm` crate (colors, attributes).
- `console_progress` flushes stdout after each update.
- Errors inside the functions are caught; invalid numeric arguments default to `0`/`100`.