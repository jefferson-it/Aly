# Types

Aly is dynamically typed. The core value types are:

| Type      | Example              | Notes                                  |
| --------- | -------------------- | -------------------------------------- |
| `int`     | `10`, `-3`           | 32-bit signed integer.                 |
| `float`   | `10.5`, `3.14`       | 32-bit floating point.                 |
| `string`  | `"hi"`, `'hi'`       | Text (see [Strings](06-strings.md)).   |
| `boolean` | `true`, `false`      | Boolean value.                         |
| `vector`  | `[1, 2, 3]`          | Ordered list (see [Vectors](09-objects-and-vectors.md)). |
| `obj`     | `{ name: "Ana" }`    | Key/value object.                      |
| `None`    | `None`               | Absence of value.                      |

## Built-in properties

Every value exposes a few properties via the `.` accessor:

### `.type`

Returns the type name as a string:

```aly
print((10).type)      # int
print("hi".type)      # string
```

### `.len`

The meaning of `.len` depends on the value's type:

| Target  | Example      | Result   | Meaning                              |
| ------- | ------------ | -------- | ------------------------------------ |
| vector  | `[1, 2, 3]`  | `3`      | number of elements                   |
| string  | `"Hello"`    | `5`      | number of characters                 |
| int     | `10`         | `2`      | number of digits                     |
| float   | `10.5`       | `[2, 1]` | `[integer_digits, decimal_digits]`   |
| obj     | `{a:1,b:2}`  | `2`      | number of keys                       |

```aly
let arr = [1, 2, 3]
print(arr.len)        # 3

let word = "Hello"
print(word.len)       # 5
```

### `.is_mut`

Whether the underlying variable is mutable:

```aly
const C = 1
# C.is_mut -> false
```

## Conversions

Values can be converted through properties:

| Property   | Effect                                  |
| ---------- | --------------------------------------- |
| `.to_int`  | Convert to integer.                     |
| `.to_float`| Convert to float.                       |
| `.to_str`  | Convert to string.                      |

```aly
let s = "42"
print(s.to_int)       # 42

let n = 3
print(n.to_float)     # 3.00
```

See also [`str.*`](11-stdlib.md) for string helpers and the numeric helper
`to_fixed(value, precision)`.
