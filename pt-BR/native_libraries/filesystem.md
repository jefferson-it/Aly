# Operações de Sistema de Arquivos em Aly

O módulo `fs` fornece funções de leitura, escrita e manipulação do sistema de arquivos.

---

## 1. Leitura de Arquivos

```aly
import fs

let content = fs.read("caminho/para/arquivo.txt")
print(content)
```

Retorna o conteúdo do arquivo como uma string. Retorna string vazia em caso de erro.

---

## 2. Escrita de Arquivos

```aly
import fs

fs.write("output.txt", "Olá Aly")
```

Escreve uma string em um arquivo, sobrescrevendo o conteúdo existente.

---

## 3. Adição a Arquivos

```aly
import fs

fs.append("log.txt", "Nova entrada de log\n")
```

Adiciona conteúdo ao final de um arquivo existente.

---

## 4. Verificação de Existência de Arquivo

```aly
import path

if path.exists("config.json") {
    print("Arquivo encontrado")
}
```

---

## 5. Copiar, Mover e Excluir

```aly
import fs

fs.copy("source.txt", "backup.txt")
fs.move("temp.txt", "permanent.txt")
fs.remove("old_file.txt")
```

---

## 6. Operações de Diretório

```aly
import fs

fs.create_dir("nova_pasta")
fs.remove_dir("pasta_vazia")

let items = fs.list_dir(".")
# Retorna vetor de nomes de arquivos no diretório
```

---

## 7. Metadados de Arquivo

```aly
import fs

let info = fs.metadata("file.txt")
print(info.size)     # Tamanho do arquivo em bytes
print(info.modified) # Timestamp da última modificação
```
