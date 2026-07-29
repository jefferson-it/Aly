# Inteligência Artificial & Aprendizagem Automática em Aly

Aly fornece módulos nativos (`ml.*`, `llm.*`, `embed.*`) para interfacciar com backends de IA/ML diretamente a partir de scripts, permitindo inferência neural, operações de chat LLM e extração de incorporações.

---

## 1. Aprendizagem Automática Principal (`ml`)

O módulo `ml` gere sessões de inferência e modelos usando **Candle** (framework de ML puro em Rust) ou **ONNX Runtime**.

### Referência da API:
* `ml.backend_info()`: Devolve o estado dos backends disponíveis.
* `ml.create(session_name, backend)`: Inicializa uma sessão (por exemplo, usando o backend `"candle"` ou `"onnx"`).
* `ml.load_model(model_name, path, backend)`: Registra um ficheiro de pesos de modelo.
* `ml.infer(session_name, model_name, input_data)`: Executa inferência do modelo.

```aly
# Verificar backends suportados
print(ml.backend_info())

# Configurar sessão
let session = ml.create("sess_0", "candle")
ml.load_model("my_model", "models/weights.bin", "candle")

# Executar inferência
let result = ml.infer("sess_0", "my_model", "sample text data")
print("Resultado da inferência: ")
```

---

## 2. LLMs e GGUF (`llm`)

Para geração de texto e IA conversacional, Aly dispõe de ligações para carregar modelos GGUF locais.

### Referência da API:
* `llm.load_gguf(model_name, filepath, context_size)`: Carrega um ficheiro de pesos GGUF para a memória.
* `llm.generate(model_name, prompt, max_tokens)`: Gera uma conclusão de texto.
* `llm.chat(model_name, system_prompt, user_prompt)`: Executa voltas de instrução conversacional.

```aly
# Carregar modelo
llm.load_gguf("llama3", "models/llama3-8b.gguf", 2048)

# Gerar resposta
let prompt = "Explique recursão numa frase."
let answer = llm.generate("llama3", prompt, 128)
print("Resposta: ")
```

---

## 3. Incorporações de Texto (`embed`)

Gerar representações vetoriais de texto para pesquisa semântica e classificação.

### Referência da API:
* `embed.create(model_name, backend, dimension)`: Configura um modelo de incorporação vetorial.
* `embed.encode(model_name, text, normalize_bool)`: Calcula o vetor de incorporação de texto.

```aly
embed.create("bert", "candle", 384)
let vector = embed.encode("bert", "Hello Aly", true)
print("Representação vetorial: ")
```
