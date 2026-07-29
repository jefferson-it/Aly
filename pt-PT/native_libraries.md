# Bibliotecas Padrão no Aly

O Aly inclui bibliotecas padrão integradas que podem ser importadas para lidar com tarefas operacionais básicas, tais como carimbos de data/hora, pausas, formatação JSON/CSV e estruturas de ficheiros.

---

## 1. Operações de Tempo e Pausa (`timer`, `sys`)

* **`import timer`**: Foca-se no agendamento e atrasos.
  * `timer.sleep(ms)`: Interrompe a execução do programa durante os milissegundos especificados.
* **`import sys`**: Comunica com as métricas do sistema.
  * `sys.time()`: Retorna o tempo Unix em segundos.

```aly
import timer
import sys

print("Start: " + sys.time())
timer.sleep(1000)
print("End: " + sys.time())
```

---

## 2. Caminhos de Ficheiros (`path`)

A biblioteca `path` fornece funções utilitárias multiplataforma para a resolução de caminhos de ficheiros.

* `path.exists(filepath)`: Retorna `true` se o ficheiro existir.
* `path.join(part1, part2)`: Concatena caminhos de diretório de forma segura.

```aly
import path

if path.exists("config.json") {
    print("Found configuration file.")
}
```
