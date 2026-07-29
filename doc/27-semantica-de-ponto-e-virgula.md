# Semântica de `;`

O caractere `;` só é permitido imediatamente após cabeçalhos de loops
(`for`, `while`, `until`, `repeat`) para separar múltiplas declarações
iniciais. Ele **não** funciona como terminador de statements em geral;
statements são finalizados implicitamente por quebra de linha ou por
bloco `{ … }`.

**Uso válido**
```aly
for i = 1; i <= 10; i++ { print(i) }
while cond; { body }
repeat 5; { … }
```

**Uso inválido (erro)**
```aly
print("x");   // erro – ';' fora de cabeçalho de loop
```

Documentar que apenas múltiplas declarações em um cabeçalho de loop
podem usar `;`, caso contrário a quebra de linha basta.