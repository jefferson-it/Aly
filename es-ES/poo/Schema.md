# Esquemas (OOP similar a clases) en Aly

Los esquemas proporcionan un sistema de plano similar a clases para instanciar objetos estructurados con valores de campo predeterminados y métodos de miembro.

---

## 1. Definición de Esquema

Un esquema se declara usando la palabra clave `schema`.

```aly
schema Person {
    name: "John",
    age: 25,
    fun greet() {
        print("Hello, my name is " + @name + " and I am " + @age)
    }
}
```

* **Prefijo `@`**: Se usa dentro de los métodos del esquema para hacer referencia a los campos pertenecientes a la instancia actual (equivalente a `this` o `self`).

---

## 2. Instanciación de Esquemas (`new`)

Las instancias se crean usando la palabra clave `new`.

### Constructor Predeterminado
Si no se pasan argumentos, los campos se inicializan con sus valores predeterminados:
```aly
let p = new Person()
p.greet() # Outputs: Hello, my name is John and I am 25
```

### Argumentos del Constructor
Pasar argumentos a `new Esquema(...)` sobrescribe los valores de campo predeterminados en el orden de su declaración:
```aly
let custom = new Person("Jefferson", 30)
custom.greet() # Outputs: Hello, my name is Jefferson and I am 30
```