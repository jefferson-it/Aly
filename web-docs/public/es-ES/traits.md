# Rasgos (Traits) en Aly

Los rasgos definen interfaces: un conjunto de firmas de métodos que un esquema puede implementar. Habilitan el polimorfismo y los contratos de código.

---

## 1. Definición de un Rasgo

Un rasgo declara nombres de métodos y sus listas de parámetros:

```aly
trait Drawable {
    fun draw(canvas)
    fun resize(w, h)
}
```

---

## 2. Implementación de un Rasgo

Un esquema implementa un rasgo definiendo todos los métodos requeridos:

```aly
schema Circle {
    radius: 10,
    
    fun draw(canvas) {
        print("Drawing circle at radius " + @radius)
    }
    
    fun resize(w, h) {
        @radius = w
    }
}
```

El sistema de rasgos en tiempo de ejecución (`TraitDef`) verifica que todos los métodos del rasgo existan en el esquema (incluyendo los heredados).

---

## 3. Verificación de Rasgo

En tiempo de ejecución, `is_implemented_by` verifica si un esquema satisface un rasgo comprobando que cada método declarado en el rasgo exista en el esquema o en su cadena de padres.