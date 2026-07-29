# Ferramentas em Aly

Aly inclui um conjunto de ferramentas de desenvolvimento para formatação, linting, protocolo de servidor de linguagem, debug, testes e recarga a quente.

---

## 1. Formatador de Código (`fmt`)

```bash
aly fmt input.aly          # Formatar um único ficheiro
aly fmt input.aly -o output.aly  # Formatar e escrever para ficheiro
```

O formatador padroniza a indentação, espaçamento e posicionamento de chavetas.

---

## 2. Linter (`linter`)

```bash
aly lint input.aly
```

Verifica problemas comuns:
- Variáveis não utilizadas
- Código inalcançável
- Incompatibilidades de tipos
- Valores de retorno em falta

---

## 3. Protocolo de Servidor de Linguagem (`lsp`)

```bash
aly lsp
```

Fornece funcionalidades de IDE através de LSP:
- Auto-completar
- Ir para a definição
- Informação de tipo ao passar o rato
- Relatório de diagnósticos
- Acções de código

---

## 4. Depurador (`debugger`)

```bash
aly debug input.aly
```

Funcionalidades de debug interactivo:
- Avançar passo a passo na execução
- Pontos de interrupção
- Inspecção de variáveis
- Rastreamentos de pilha de chamadas

---

## 5. Executante de Testes (`test_runner`)

```bash
aly test
```

Descobre e executa funções de teste:
```aly
fun test_addition() {
    assert(2 + 2 eq 4)
}

fun test_subtraction() {
    assert(5 - 3 eq 2)
}
```

---

## 6. Recarga a Quente (`hotreload`)

```bash
aly run --watch input.aly
```

Detecta automaticamente alterações de ficheiros e recarrega o script sem reiniciar, preservando o estado da aplicação sempre que possível.

---

## 7. Gerador de Documentação (`doc`)

```bash
aly doc input.aly -o docs/
```

Gera documentação Markdown a partir de comentários e anotações no código fonte.

---

## 8. Gerador JNI (`jni_gen`)

```bash
aly jni-gen input.aly
```

Gera ligações JNI para chamar Aly a partir de código Java.
