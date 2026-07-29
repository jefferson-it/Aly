# Ferramentas no Aly

O Aly acompanha um conjunto de ferramentas de desenvolvimento para formatação, linting, protocolo de servidor de linguagem, depuração, teste e recarga rápida.

---

## 1. Formatador de Código (`fmt`)

```bash
aly fmt input.aly          # Formatar um único arquivo
aly fmt input.aly -o output.aly  # Formatar e salvar em arquivo
```

O formatador padroniza indentação, espaçamento e posicionamento de chaves.

---

## 2. Linter (`linter`)

```bash
aly lint input.aly
```

Verifica problemas comuns:
- Variáveis não utilizadas
- Código inalcançável
- Incompatibilidades de tipo
- Valores de retorno ausentes

---

## 3. Protocolo de Servidor de Linguagem (`lsp`)

```bash
aly lsp
```

Fornece recursos de IDE através do LSP:
- Autocompletar
- Ir para definição
- Informação de tipo ao passar o mouse
- Relatório de diagnósticos
- Ações de código

---

## 4. Depurador (`debugger`)

```bash
aly debug input.aly
```

Recursos de depuração interativa:
- Execução passo a passo
- Pontos de interrupção
- Inspeção de variáveis
- Rastreamento de pilha de chamadas

---

## 5. Executor de Testes (`test_runner`)

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

## 6. Recarga Rápida (`hotreload`)

```bash
aly run --watch input.aly
```

Detecta automaticamente alterações em arquivos e recarrega o script sem reiniciar, preservando o estado da aplicação quando possível.

---

## 7. Gerador de Documentação (`doc`)

```bash
aly doc input.aly -o docs/
```

Gera documentação em Markdown a partir de comentários e anotações no código-fonte.

---

## 8. Gerador JNI (`jni_gen`)

```bash
aly jni-gen input.aly
```

Gera bindings JNI para chamar Aly a partir de código Java.
