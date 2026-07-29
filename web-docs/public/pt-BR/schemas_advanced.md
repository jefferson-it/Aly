# Avançado Schemas in Aly

Beyond basic schema definitions, Aly supports inheritance, static methods, generics, and custom constructors.

---

## 1. Schema Inheritance

A schema can extend another schema using the `extends` clause, inheriting all fields and methods:

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

Child schemas inherit fields and methods from the parent. Methods can be overridden.

---

## 2. Static Methods

Static methods belong to the schema itself, not to instances:

```aly
schema MathUtils {
    static fun square(n) {
        return n * n
    }
    
    static fun cube(n) {
        return n * n * n
    }
}

# Called on the schema, not an instance
print(MathUtils.square(5))  # Outputs: 25
print(MathUtils.cube(3))    # Outputs: 27
```

Static methods cannot access `@` instance fields.

---

## 3. Generics (Template Parameters)

Schemas can declare generic type parameters:

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

## 4. Custom Constructor Body

A schema can define a custom constructor that runs after field initialization:

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

The constructor body is optional and runs after default field values are set.

---

## 5. Field Access in Methods

Use `@fieldname` to reference instance fields inside methods:

```aly
schema Counter {
    count: 0,
    fun increment() {
        @count = @count + 1
    }
}
```

The `@` prefix is required — it is equivalent to `this.` or `self.` in other languages.