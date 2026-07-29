# Condicionais no Aly

Condicionais permitem direcionar o fluxo de execução com base em expressões booleanas.

---

## 1. Declarações `if` / `elif` / `else`

A declaração `if` avalia uma condição lógica. Chaves `{}` são obrigatórias para delimitar escopos de bloco.

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

O Aly usa nomes textuais no estilo prefixo para operadores de comparação padrão:
* **`eq`**: Igual (`==`)
* **`ne`**: Diferente (`!=`)
* **`gt`**: Maior que (`>`)
* **`lt`**: Menor que (`<`)
* **`gte`**: Maior ou Igual (`>=`)
* **`lte`**: Menor ou Igual (`<=`)

---

## 3. Operadores Lógicos

* **`and`**: AND lógico. Retorna `true` apenas se ambas as expressões forem verdadeiras.
* **`or`**: OR lógico. Retorna `true` se pelo menos uma expressão for verdadeira.
* **`not` / `!`**: NOT lógico. Inverte o valor booleano da expressão.
