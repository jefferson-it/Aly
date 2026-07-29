# Operaciones del Sistema de Archivos en Aly

El módulo `fs` proporciona funciones de lectura, escritura y manipulación del sistema de archivos.

---

## 1. Lectura de Archivos

```aly
import fs

let content = fs.read("path/to/file.txt")
print(content)
```

Devuelve el contenido del archivo como una cadena. Devuelve una cadena vacía en caso de error.

---

## 2. Escritura de Archivos

```aly
import fs

fs.write("output.txt", "Hello Aly")
```

Escribe una cadena en un archivo, sobrescribiendo el contenido existente.

---

## 3. Anexar a Archivos

```aly
import fs

fs.append("log.txt", "New log entry\n")
```

Añade contenido al final de un archivo existente.

---

## 4. Verificación de Existencia de Archivo

```aly
import path

if path.exists("config.json") {
    print("File found")
}
```

---

## 5. Copiar, Mover y Eliminar

```aly
import fs

fs.copy("source.txt", "backup.txt")
fs.move("temp.txt", "permanent.txt")
fs.remove("old_file.txt")
```

---

## 6. Operaciones de Directorio

```aly
import fs

fs.create_dir("new_folder")
fs.remove_dir("empty_folder")

let items = fs.list_dir(".")
# Returns vector of filenames in directory
```

---

## 7. Metadatos de Archivo

```aly
import fs

let info = fs.metadata("file.txt")
print(info.size)     # File size in bytes
print(info.modified) # Last modified timestamp
```