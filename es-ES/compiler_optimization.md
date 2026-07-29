# Optimización del Compilador en Aly

El compilador de Aly incluye pasos de optimización tanto para el compilador AOT como para la VM JIT.

---

## 1. Pasos de Optimización

El compilador ejecuta una serie de pasos de optimización sobre el MIR (Representación Intermedia de Nivel Medio):

- **Propagación de Constantes**: Evalúa expresiones constantes en tiempo de compilación (`const_eval`)
- **Eliminación de Código Muerto**: Elimina código inalcanzable
- **Inlining**: Reemplaza llamadas a funciones con el cuerpo de la función cuando es beneficioso
- **Reducción de Fuerza**: Reemplaza operaciones costosas por otras más sencillas

---

## 2. Compilación Incremental

```bash
alyc -o output input.aly --incremental
```

Caché de módulos previamente compilados y solo recompila archivos cambiados:

```aly
# La caché se almacena en el directorio .aly_cache/
# Detecta automáticamente cambios mediante marcas de tiempo y hashes de archivos
```

---

## 3. Compilación Paralela

```bash
alyc -o output input.aly --parallel
```

Compila módulos independientes en paralelo usando múltiples hilos, reduciendo el tiempo total de compilación para proyectos con múltiples módulos.

---

## 4. Optimizaciones JIT

El JIT de la VM (`jit.rs`) optimiza las rutas de código más frecuentes:

- **Caché en Línea**: Almacena en caché los resultados de búsqueda de métodos (`inline_cache.rs`)
- **Convención de Llamada Rápida**: Paso de argumentos optimizado (`fastcall.rs`, `call_conventions.rs`)
- **VM Optimizada**: Ejecución de bytecode ajustada (`optimized.rs`)