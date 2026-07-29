# Desarrollo de Juegos en Aly

Aly proporciona un módulo de desarrollo de juegos con primitivas básicas de juegos 2D.

---

## 1. Ventana de Juego

```aly
import game

let win = game.create_window("My Game", 800, 600)
game.set_fps(win, 60)
```

---

## 2. Sprites y Renderizado

```aly
let player = game.create_sprite("player.png", 100, 100)
game.draw_sprite(win, player)

let rect = game.draw_rect(win, 10, 10, 50, 50, "red")
let text = game.draw_text(win, "Score: 0", 10, 10, "white")
```

---

## 3. Manejo de Entrada

```aly
loop {
    let event = game.poll_event(win)
    if event.type eq "keydown" {
        if event.key eq "space" {
            print("Jump!")
        }
    }
    game.update(win)
}
```

---

## 4. Audio

```aly
let sound = game.load_sound("jump.wav")
game.play_sound(sound)

let music = game.load_music("background.ogg")
game.play_music(music, true)  # Loop enabled
```