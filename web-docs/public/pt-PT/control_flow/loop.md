# Loops em Aly

Laços de repetição (Loops) permitem-lhe repetir instruções de bloco. Aly unifica loops na palavra-chave `loop`.

---

## 1. Loop Condicional (Estilo While)

Avalia a expressão antes de entrar no loop. Repete enquanto a condição for `true`.

```aly
let i = 1
loop i lte 5 {
    print(i)
    i = i + 1
}
```

---

## 2. Loop de Iteração (Estilo For)

Pode especificar inicializações de loop, condições e incrementos usando pontos e vírgulas:

```aly
loop let i = 0; i lt 5; i = i + 1 {
    print(i)
}
```

---

## 3. Instruções de Controlo de Loop

* **`break` / `pare`**: Sai do loop imediatamente.
* **`continue`**: Salta directamente para a próxima iteração do loop, ignorando as instruções abaixo.
