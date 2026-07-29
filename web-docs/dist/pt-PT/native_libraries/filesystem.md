# Operações de Sistema de Ficheiros em Aly

O módulo `fs` fornece funções de leitura, escrita e manipulação de sistema de ficheiros.

---

## 1. Leitura de Ficheiros

```aly
import fs

let content = fs.read("path/to/file.txt")
print(content)
```

Devolve o conteúdo do ficheiro como uma cadeia de caracteres. Devolve uma cadeia vazia em caso de erro.

---

## 2. Escrita de Ficheiros

```aly
import fs

fs.write("output.txt", "Hello Aly")
```

Escreve uma cadeia de caracteres num ficheiro, sobrescrevendo o conteúdo existente.

---

## 3. Acrescentar a Ficheiros

```aly
import fs

fs.append("log.txt", "Nova entrada de log\n")
```

Adiciona conteúdo ao final de um ficheiro existente.

---

## 4. Verificação de Existência de Ficheiro

```aly
import path

if path.exists("config.json") {
    print("Ficheiro encontrado")
}
```

---

## 5. Copiar, Mover e Eliminar

```aly
import fs

fs.copy("source.txt", "backup.txt")
fs.move("temp.txt", "permanent.txt")
fs.remove("old_file.txt")
```

---

## 6. Operações de Directório

```aly
import fs

fs.create_dir("nova_pasta")
fs.remove_dir("pasta_vazia")

let items = fs.list_dir(".")
# Devolve vetor de nomes de ficheiros no directório
```

---

## 7. Metadados de Ficheiro

```aly
import fs

let info = fs.metadata("file.txt")
print(info.size)     # Tamanho do ficheiro em bytes
print(info.modified) # Carimbo de data da última modificação
```
