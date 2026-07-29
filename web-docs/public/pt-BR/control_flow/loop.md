# Laços de Repetição no Aly

Laços de repetição permitem repetir instruções de um bloco. O Aly unifica os laços na palavra-chave `loop`.

---

## 1. Laço Condicional (Estilo While)

Avalia a expressão antes de entrar no laço. Repete enquanto a condição for `true`.

```aly
let i = 1
loop i lte 5 {
    print(i)
    i = i + 1
}
```

---

## 2. Laço de Iteração (Estilo For)

Você pode especificar inicializações, condições e incrementos do laço usando ponto e vírgula:

```aly
loop let i = 0; i lt 5; i = i + 1 {
    print(i)
}
```

---

## 3. Declarações de Controle de Laço

* **`break` / `pare`**: Sai do laço imediatamente.
* **`continue`**: Pula diretamente para a próxima iteração do laço, ignorando as instruções abaixo.
