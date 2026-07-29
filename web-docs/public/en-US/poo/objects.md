# Objects in Aly

Objects in Aly are dynamic map key-value structures. They can hold variables, nested objects, and functions (acting as methods).

---

## 1. Object Instantiation

An object literal is declared using curly braces `{}`.

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

## 2. Dynamic Modifications

Properties and methods can be added or overridden dynamically:

```aly
let player = {}
player.score = 0
player.increment = fun(self, points) {
    self.score = self.score + points
}

player.increment(player, 10)
print(player.score) # Outputs: 10
```
