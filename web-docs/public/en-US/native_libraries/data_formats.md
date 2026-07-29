# Data Formats & Compression in Aly

Aly has native encoders, decoders, compressors, and parser modules for dealing with multiple common data formats.

---

## 1. Codecs (Base64 & Hex)

Encode and decode bytes/strings to Hex and Base64 format:
```aly
import codec

let original = "Hello Aly"
let b64 = codec.base64_encode(original)
print("Base64: ")

let hex = codec.hex_encode(original)
print("Hex: ")
```

---

## 2. CSV Operations

Read and write Tabular Comma-Separated Values (CSV):
```aly
import csv

let data = "name,role,active\nJefferson,Developer,true\nJohn,Tester,false"
let parsed = csv.parse(data)

loop let i = 0; i lt parsed.len; i = i + 1 {
    let row = parsed[i]
    print("User: " + row[0] + " is " + row[1])
}
```

---

## 3. GZIP Compression

Compress and decompress data payloads:
```aly
import gzip

let text = "Large text content to compress..."
let compressed = gzip.compress(text)
let decompressed = gzip.decompress(compressed)

print("Restored: ")
```

---

## 4. Tar Archiving

Create and extract tar archives:
```aly
import tar

tar.compress("archive.tar", ["file1.txt", "file2.txt"])
tar.decompress("archive.tar", "output_dir/")
```

---

## 5. PDF Generation

Generate PDF documents programmatically:
```aly
import pdf

let doc = pdf.create()
pdf.add_page(doc)
pdf.set_font(doc, "Arial", 12)
pdf.text(doc, 10, 10, "Hello Aly PDF")
pdf.save(doc, "output.pdf")
```
