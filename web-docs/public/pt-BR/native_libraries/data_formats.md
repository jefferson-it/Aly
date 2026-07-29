# Formatos de Dados e Compressão em Aly

Aly possui módulos nativos de codificadores, decodificadores, compressores e parsers para lidar com vários formatos de dados comuns.

---

## 1. Codecs (Base64 e Hex)

Codifica e decodifica bytes/strings para o formato Hex e Base64:
```aly
import codec

let original = "Hello Aly"
let b64 = codec.base64_encode(original)
print("Base64: $b64")

let hex = codec.hex_encode(original)
print("Hex: $hex")
```

---

## 2. Operações CSV

Lê e escreve Valores Separados por Vírgula Tabulares (CSV):
```aly
import csv

let data = "name,role,active\nJefferson,Developer,true\nJohn,Tester,false"
let parsed = csv.parse(data)

loop let i = 0; i lt parsed.len; i = i + 1 {
    let row = parsed[i]
    print("Usuário: " + row[0] + " é " + row[1])
}
```

---

## 3. Compressão GZIP

Comprime e descomprime payloads de dados:
```aly
import gzip

let text = "Large text content to compress..."
let compressed = gzip.compress(text)
let decompressed = gzip.decompress(compressed)

print("Restaurado: $decompressed")
```

---

## 4. Arquivamento Tar

Cria e extrai arquivos tar:
```aly
import tar

tar.compress("archive.tar", ["file1.txt", "file2.txt"])
tar.decompress("archive.tar", "output_dir/")
```

---

## 5. Geração de PDF

Gera documentos PDF programaticamente:
```aly
import pdf

let doc = pdf.create()
pdf.add_page(doc)
pdf.set_font(doc, "Arial", 12)
pdf.text(doc, 10, 10, "Hello Aly PDF")
pdf.save(doc, "output.pdf")
```
