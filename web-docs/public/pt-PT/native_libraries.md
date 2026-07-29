# Bibliotecas Standard em Aly

Aly inclui bibliotecas standard integradas que podem ser importadas para lidar com tarefas operacionais básicas, como carimbos de data/hora, suspensões, formatação JSON/CSV e estruturas de ficheiros.

---

## 1. Operações de Tempo e Suspensão (`timer`, `sys`)

* **`import timer`**: Foca-se em agendamento e atrasos.
  * `timer.sleep(ms)`: Interrompe a execução do programa pelos milissegundos especificados.
* **`import sys`**: Interfaccia com métricas do sistema.
  * `sys.time()`: Devolve o tempo Unix em segundos.

```aly
import timer
import sys

print("Início: " + sys.time())
timer.sleep(1000)
print("Fim: " + sys.time())
```

---

## 2. Caminhos de Ficheiros (`path`)

A biblioteca `path` fornece funções utilitárias de resolução de caminhos de ficheiro multiplataforma.

* `path.exists(filepath)`: Devolve `true` se o ficheiro existir.
* `path.join(part1, part2)`: Concatena caminhos de directório de forma segura.

```aly
import path

if path.exists("config.json") {
    print("Ficheiro de configuração encontrado.")
}
```
