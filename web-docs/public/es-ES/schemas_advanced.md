# Esquemas Avanzados en Aly

Más allá de las definiciones básicas de esquema, Aly admite herencia, métodos estáticos, genéricos y constructores personalizados.

---

## 1. Herencia de Esquemas

Un esquema puede extender otro esquema usando la cláusula `extends`, heredando todos los campos y métodos:

```aly
schema Animal {
    name: "unknown",
    fun speak() {
        print(@name + " makes a sound")
    }
}

schema Dog extends Animal {
    name: "Rex",
    fun speak() {
        print(@name + " barks")
    }
}

let pet = new Dog()
pet.speak()  # Outputs: Rex barks
```

Los esquemas hijos heredan campos y métodos del padre. Los métodos pueden ser sobrescritos.

---

## 2. Métodos Estáticos

Los métodos estáticos pertenecen al esquema en sí, no a las instancias:

```aly
schema MathUtils {
    static fun square(n) {
        return n * n
    }
    
    static fun cube(n) {
        return n * n * n
    }
}

# Llamado en el esquema, no en una instancia
print(MathUtils.square(5))  # Outputs: 25
print(MathUtils.cube(3))    # Outputs: 27
```

Los métodos estáticos no pueden acceder a campos de instancia `@`.

---

## 3. Genéricos (Parámetros de Plantilla)

Los esquemas pueden declarar parámetros de tipo genérico:

```aly
schema Box(T) {
    value: null,
    
    fun get() {
        return @value
    }
    
    fun set(val) {
        @value = val
    }
}

let intBox = new Box(42)
let strBox = new Box("Hello")
```

---

## 4. Cuerpo del Constructor Personalizado

Un esquema puede definir un constructor personalizado que se ejecuta después de la inicialización de campos:

```aly
schema User {
    name: "guest",
    role: "viewer",
    
    fun __init__() {
        print("Created user: " + @name)
    }
}

let u = new User("Alice")
```

El cuerpo del constructor es opcional y se ejecuta después de establecer los valores de campo predeterminados.

---

## 5. Acceso a Campos en Métodos

Use `@nombre_campo` para hacer referencia a los campos de la instancia dentro de los métodos:

```aly
schema Counter {
    count: 0,
    fun increment() {
        @count = @count + 1
    }
}
```

El prefijo `@` es obligatorio: es equivalente a `this.` o `self.` en otros lenguajes.