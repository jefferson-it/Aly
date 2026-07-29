# Inteligência Artificial e Machine Learning em Aly

Aly fornece módulos nativos (`ml.*`, `llm.*`, `embed.*`) para interagir com backends de IA/ML diretamente dos scripts, permitindo inferência neural, operações de chat com LLM e extração de embeddings.

---

## 1. Machine Learning Principal (`ml`)

O módulo `ml` gerencia sessões de inferência e modelos usando **Candle** (framework de ML puro em Rust) ou **ONNX Runtime**.

### Referência da API:
* `ml.backend_info()`: Retorna o status dos backends disponíveis.
* `ml.create(session_name, backend)`: Inicializa uma sessão (por exemplo, usando backend `"candle"` ou `"onnx"`).
* `ml.load_model(model_name, path, backend)`: Registra um arquivo de pesos de modelo.
* `ml.infer(session_name, model_name, input_data)`: Executa a inferência do modelo.

```aly
# Check supported backends
print(ml.backend_info())

# Setup session
let session = ml.create("sess_0", "candle")
ml.load_model("my_model", "models/weights.bin", "candle")

# Run inference
let result = ml.infer("sess_0", "my_model", "sample text data")
print("Saída da inferência: $result")
```

---

## 2. LLMs e GGUF (`llm`)

Para geração de texto e IA conversacional, Aly apresenta bindings para carregar modelos GGUF locais.

### Referência da API:
* `llm.load_gguf(model_name, filepath, context_size)`: Carrega um arquivo de pesos GGUF na memória.
* `llm.generate(model_name, prompt, max_tokens)`: Gera a conclusão de texto.
* `llm.chat(model_name, system_prompt, user_prompt)`: Executa turnos de instrução conversacional.

```aly
# Load model
llm.load_gguf("llama3", "models/llama3-8b.gguf", 2048)

# Generate response
let prompt = "Explain recursion in one sentence."
let answer = llm.generate("llama3", prompt, 128)
print("Resposta: $answer")
```

---

## 3. Text Embeddings (`embed`)

Gera representações em vetor de texto para busca semântica e classificação.

### Referência da API:
* `embed.create(model_name, backend, dimension)`: Configura um modelo de embedding em vetor.
* `embed.encode(model_name, text, normalize_bool)`: Calcula o vetor de embedding de texto.

```aly
embed.create("bert", "candle", 384)
let vector = embed.encode("bert", "Hello Aly", true)
print("Representação em vetor: $vector")
```