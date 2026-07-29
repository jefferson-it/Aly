# Bibliotecas Padrão no Aly

O Aly inclui bibliotecas padrão integradas que podem ser importadas para lidar com tarefas operacionais básicas, como carimbos de data/hora, pausas (sleeps), formatação JSON/CSV e estruturas de arquivos.

---

## 1. Operações de Tempo e Pausa (`timer`, `sys`)

* **`import timer`**: Concentra-se em agendamento e atrasos.
  * `timer.sleep(ms)`: Interrompe a execução do programa pelos milissegundos especificados.
* **`import sys`**: Interface com métricas do sistema.
  * `sys.time()`: Retorna o tempo Unix em segundos.

```aly
import timer
import sys

print("Start: " + sys.time())
timer.sleep(1000)
print("End: " + sys.time())
```

---

## 2. Caminhos de Arquivo (`path`)

A biblioteca `path` fornece funções utilitárias de resolução de caminhos de arquivos multiplataforma.

* `path.exists(filepath)`: Retorna `true` se o arquivo existir.
* `path.join(part1, part2)`: Concatena caminhos de diretório de forma segura.

```aly
import path

if path.exists("config.json") {
    print("Found configuration file.")
}
```
