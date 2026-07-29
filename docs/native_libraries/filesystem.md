# Filesystem Operations in Aly

The `fs` module provides filesystem read, write, and manipulation functions.

---

## 1. Reading Files

```aly
import fs

let content = fs.read("path/to/file.txt")
print(content)
```

Returns file contents as a string. Returns empty string on error.

---

## 2. Writing Files

```aly
import fs

fs.write("output.txt", "Hello Aly")
```

Writes a string to a file, overwriting existing content.

---

## 3. Appending to Files

```aly
import fs

fs.append("log.txt", "New log entry\n")
```

Adds content to the end of an existing file.

---

## 4. File Existence Check

```aly
import path

if path.exists("config.json") {
    print("File found")
}
```

---

## 5. Copying, Moving, and Deleting

```aly
import fs

fs.copy("source.txt", "backup.txt")
fs.move("temp.txt", "permanent.txt")
fs.remove("old_file.txt")
```

---

## 6. Directory Operations

```aly
import fs

fs.create_dir("new_folder")
fs.remove_dir("empty_folder")

let items = fs.list_dir(".")
# Returns vector of filenames in directory
```

---

## 7. File Metadata

```aly
import fs

let info = fs.metadata("file.txt")
print(info.size)     # File size in bytes
print(info.modified) # Last modified timestamp
```