# Schemas (Class-like OOP) in Aly

Schemas provide a class-like blueprint system to instantiate structured objects with default field values and member methods.

---

## 1. Schema Definition

A schema is declared using the `schema` keyword.

```aly
schema Person {
    name: "John",
    age: 25,
    fun greet() {
        print("Hello, my name is " + @name + " and I am " + @age)
    }
}
```

* **`@` prefix**: Used inside schema methods to reference fields belonging to the current instance (equivalent to `this` or `self`).

---

## 2. Instantiating Schemas (`new`)

Instances are created using the `new` keyword.

### Default Constructor
If no arguments are passed, fields are initialized to their default values:
```aly
let p = new Person()
p.greet() # Outputs: Hello, my name is John and I am 25
```

### Constructor Arguments
Passing arguments to `new Schema(...)` overrides default field values in order of their declaration:
```aly
let custom = new Person("Jefferson", 30)
custom.greet() # Outputs: Hello, my name is Jefferson and I am 30
```
