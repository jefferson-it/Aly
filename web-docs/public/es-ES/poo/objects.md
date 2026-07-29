# Objetos en Aly

Los objetos en Aly son estructuras dinámicas de mapa clave-valor. Pueden contener variables, objetos anidados y funciones (que actúan como métodos).

---

## 1. Instanciación de Objetos

Un literal de objeto se declara usando llaves `{}`.

```aly
let user = {
    name: "Jefferson",
    role: "Developer",
    greet: fun(self) {
        print("Hello, my name is " + self.name)
    }
}

user.greet(user) # Outputs: Hello, my name is Jefferson
```

---

## 2. Modificaciones Dinámicas

Las propiedades y métodos pueden añadirse o sobrescribirse dinámicamente:

```aly
let player = {}
player.score = 0
player.increment = fun(self, points) {
    self.score = self.score + points
}

player.increment(player, 10)
print(player.score) # Outputs: 10
```