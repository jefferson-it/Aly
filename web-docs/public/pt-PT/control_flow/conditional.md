# Condicionais em Aly

As condicionais permitem-lhe orientar o fluxo de execução com base em expressões booleanas.

---

## 1. Instruções `if` / `elif` / `else`

A instrução `if` avalia uma condição lógica. Chavetas `{}` são obrigatórias para envolver blocos de escopo.

```aly
let age = 18

if age gt 60 {
    print("Senior")
} elif age gte 18 {
    print("Adult")
} else {
    print("Minor")
}
```

---

## 2. Operadores de Comparação

Aly utiliza nomes em estilo prefixo para operadores de comparação padrão:
* **`eq`**: Igual (`==`)
* **`ne`**: Diferente (`!=`)
* **`gt`**: Maior que (`>`)
* **`lt`**: Menor que (`<`)
* **`gte`**: Maior ou igual (`>=`)
* **`lte`**: Menor ou igual (`<=`)

---

## 3. Operadores Lógicos

* **`and`**: E lógico. Devolve `true` apenas se ambas as expressões forem verdadeiras.
* **`or`**: Ou lógico. Devolve `true` se pelo menos uma expressão for verdadeira.
* **`not` / `!`**: Não lógico. Inverte o valor booleano da expressão.
