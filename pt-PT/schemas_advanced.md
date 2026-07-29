# Schemas Avançados em Aly

Para além das definições básicas de schema, Aly suporta herança, métodos estáticos, genéricos e construtores personalizados.

---

## 1. Herança de Schema

Um schema pode estender outro schema utilizando a cláusula `extends`, herdando todos os campos e métodos:

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
pet.speak()  # Resultado: Rex barks
```

Os schemas filhos herdam campos e métodos do progenitor. Os métodos podem ser substituídos.

---

## 2. Métodos Estáticos

Os métodos estáticos pertencem ao schema em si, não a instâncias:

```aly
schema MathUtils {
    static fun square(n) {
        return n * n
    }
    
    static fun cube(n) {
        return n * n * n
    }
}

# Chamado no schema, não numa instância
print(MathUtils.square(5))  # Resultado: 25
print(MathUtils.cube(3))    # Resultado: 27
```

Os métodos estáticos não podem aceder a campos de instância `@`.

---

## 3. Genéricos (Parâmetros de Modelo)

Os schemas podem declarar parâmetros de tipo genéricos:

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

## 4. Corpo do Construtor Personalizado

Um schema pode definir um construtor personalizado que é executado após a inicialização de campos:

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

O corpo do construtor é opcional e é executado após os valores predefinidos de campos serem definidos.

---

## 5. Acesso a Campos em Métodos

Utilize `@fieldname` para referenciar campos de instância dentro de métodos:

```aly
schema Counter {
    count: 0,
    fun increment() {
        @count = @count + 1
    }
}
```

O prefixo `@` é obrigatório — é equivalente a `this.` ou `self.` em outras linguagens.
