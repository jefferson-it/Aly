# Schemas Avançados no Aly

Além das definições básicas de schema, o Aly suporta herança, métodos estáticos, genéricos e construtores personalizados.

---

## 1. Herança de Schema

Um schema pode estender outro schema usando a cláusula `extends`, herdando todos os campos e métodos:

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

Schemas filhos herdam campos e métodos do pai. Os métodos podem ser sobrescritos.

---

## 2. Métodos Estáticos

Métodos estáticos pertencem ao próprio schema, não às instâncias:

```aly
schema MathUtils {
    static fun square(n) {
        return n * n
    }
    
    static fun cube(n) {
        return n * n * n
    }
}

# Chamado no schema, não em uma instância
print(MathUtils.square(5))  # Outputs: 25
print(MathUtils.cube(3))    # Outputs: 27
```

Métodos estáticos não podem acessar campos de instância com `@`.

---

## 3. Genéricos (Parâmetros de Template)

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

## 4. Corpo de Construtor Personalizado

Um schema pode definir um construtor personalizado que é executado após a inicialização do campo:

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

O corpo do construtor é opcional e é executado depois que os valores padrão dos campos são definidos.

---

## 5. Acesso a Campos em Métodos

Use `@fieldname` para referenciar campos de instância dentro de métodos:

```aly
schema Counter {
    count: 0,
    fun increment() {
        @count = @count + 1
    }
}
```

O prefixo `@` é obrigatório — ele é equivalente a `this.` ou `self.` em outras linguagens.