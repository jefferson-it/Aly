# Desenvolvimento de Jogos em Aly

Aly fornece um módulo de desenvolvimento de jogos com primitivas básicas para jogos 2D.

---

## 1. Janela do Jogo

```aly
import game

let win = game.create_window("Meu Jogo", 800, 600)
game.set_fps(win, 60)
```

---

## 2. Sprites e Renderização

```aly
let player = game.create_sprite("player.png", 100, 100)
game.draw_sprite(win, player)

let rect = game.draw_rect(win, 10, 10, 50, 50, "red")
let text = game.draw_text(win, "Pontos: 0", 10, 10, "white")
```

---

## 3. Manipulação de Entrada

```aly
loop {
    let event = game.poll_event(win)
    if event.type eq "keydown" {
        if event.key eq "space" {
            print("Pular!")
        }
    }
    game.update(win)
}
```

---

## 4. Áudio

```aly
let sound = game.load_sound("jump.wav")
game.play_sound(sound)

let music = game.load_music("background.ogg")
game.play_music(music, true)  # Loop ativado
```
