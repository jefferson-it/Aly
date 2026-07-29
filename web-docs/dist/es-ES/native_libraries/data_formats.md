# Formatos de Datos y Compresión en Aly

Aly tiene codificadores, decodificadores, compresores y módulos de análisis nativos para trabajar con múltiples formatos de datos comunes.

---

## 1. Codecs (Base64 y Hex)

Codificar y decodificar bytes/cadenas a formato Hex y Base64:
```aly
import codec

let original = "Hello Aly"
let b64 = codec.base64_encode(original)
print("Base64: $b64")

let hex = codec.hex_encode(original)
print("Hex: $hex")
```

---

## 2. Operaciones CSV

Leer y escribir Valores Separados por Comas (CSV) tabulares:
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

## 3. Compresión GZIP

Comprimir y descomprimir cargas de datos:
```aly
import gzip

let text = "Large text content to compress..."
let compressed = gzip.compress(text)
let decompressed = gzip.decompress(compressed)

print("Restored: $decompressed")
```

---

## 4. Archivos Tar

Crear y extraer archivos tar:
```aly
import tar

tar.compress("archive.tar", ["file1.txt", "file2.txt"])
tar.decompress("archive.tar", "output_dir/")
```

---

## 5. Generación de PDF

Generar documentos PDF programáticamente:
```aly
import pdf

let doc = pdf.create()
pdf.add_page(doc)
pdf.set_font(doc, "Arial", 12)
pdf.text(doc, 10, 10, "Hello Aly PDF")
pdf.save(doc, "output.pdf")
```