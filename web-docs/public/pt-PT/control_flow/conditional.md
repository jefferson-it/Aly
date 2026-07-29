# Condicionais no Aly

As condicionais permitem-lhe orientar o fluxo de execução com base em expressões booleanas.

---

## 1. Declarações `if` / `elif` / `else`

A declaração `if` avalia uma condição lógica. As chavetas `{}` são necessárias para delimitar os escopos dos blocos.

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

O Aly utiliza nomes de texto em estilo prefixo para os operadores de comparação padrão:
* **`eq`**: Igual (`==`)
* **`ne`**: Diferente (`!=`)
* **`gt`**: Maior Que (`>`)
* **`lt`**: Menor Que (`<`)
* **`gte`**: Maior ou Igual Que (`>=`)
* **`lte`**: Menor ou Igual Que (`<=`)

---

## 3. Operadores Lógicos

* **`and`**: AND Lógico. Retorna `true` apenas se ambas as expressões forem verdadeiras.
* **`or`**: OR Lógico. Retorna `true` se pelo menos uma expressão for verdadeira.
* **`not` / `!`**: NOT Lógico. Inverte o valor booleano da expressão.
