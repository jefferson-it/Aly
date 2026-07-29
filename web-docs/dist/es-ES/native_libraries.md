# Bibliotecas Estándar en Aly

Aly incluye bibliotecas estándar integradas que se pueden importar para manejar tareas operativas básicas como marcas de tiempo, sleeps, formato JSON/CSV y estructuras de archivos.

---

## 1. Operaciones de Tiempo y Sleep (`timer`, `sys`)

* **`import timer`**: Se enfoca en la programación y las demoras.
  * `timer.sleep(ms)`: Detiene la ejecución del programa durante los milisegundos especificados.
* **`import sys`**: Interfiace con las métricas del sistema.
  * `sys.time()`: Devuelve el tiempo Unix en segundos.

```aly
import timer
import sys

print("Start: " + sys.time())
timer.sleep(1000)
print("End: " + sys.time())
```

---

## 2. Rutas de Archivo (`path`)

La biblioteca `path` proporciona funciones de utilidad para la resolución de rutas de archivos multiplataforma.

* `path.exists(filepath)`: Devuelve `true` si el archivo existe.
* `path.join(part1, part2)`: Concatena rutas de directorio de forma segura.

```aly
import path

if path.exists("config.json") {
    print("Found configuration file.")
}
```