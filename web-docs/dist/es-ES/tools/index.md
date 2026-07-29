# Herramientas en Aly

Aly incluye un conjunto de herramientas de desarrollo para formateo, análisis estático, protocolo de servidor de lenguaje, depuración, pruebas y recarga en caliente.

---

## 1. Formateador de Código (`fmt`)

```bash
aly fmt input.aly          # Formatear un solo archivo
aly fmt input.aly -o output.aly  # Formatear y escribir en archivo
```

El formateador estandariza la indentación, el espaciado y la colocación de llaves.

---

## 2. Linter (`linter`)

```bash
aly lint input.aly
```

Verifica problemas comunes:
- Variables no utilizadas
- Código inalcanzable
- Incompatibilidades de tipos
- Valores de retorno faltantes

---

## 3. Protocolo de Servidor de Lenguaje (`lsp`)

```bash
aly lsp
```

Proporciona funciones de IDE a través de LSP:
- Autocompletado
- Ir a la definición
- Información de tipo al pasar el cursor
- Informes de diagnóstico
- Acciones de código

---

## 4. Depurador (`debugger`)

```bash
aly debug input.aly
```

Funciones de depuración interactiva:
- Recorrer la ejecución paso a paso
- Puntos de interrupción
- Inspección de variables
- Rastreos de la pila de llamadas

---

## 5. Ejecutor de Pruebas (`test_runner`)

```bash
aly test
```

Descubre y ejecuta funciones de prueba:
```aly
fun test_addition() {
    assert(2 + 2 eq 4)
}

fun test_subtraction() {
    assert(5 - 3 eq 2)
}
```

---

## 6. Recarga en Caliente (`hotreload`)

```bash
aly run --watch input.aly
```

Detecta automáticamente cambios en los archivos y recarga el script sin reiniciar, preservando el estado de la aplicación cuando sea posible.

---

## 7. Generador de Documentación (`doc`)

```bash
aly doc input.aly -o docs/
```

Genera documentación Markdown a partir de comentarios y anotaciones del código fuente.

---

## 8. Generador JNI (`jni_gen`)

```bash
aly jni-gen input.aly
```

Genera enlaces JNI para llamar a Aly desde código Java.