# Artificial Intelligence & Machine Learning in Aly

Aly provides native modules (`ml.*`, `llm.*`, `embed.*`) to interface with AI/ML backends directly from scripts, allowing neural inference, LLM chat operations, and embedding extraction.

---

## 1. Core Machine Learning (`ml`)

The `ml` module manages inference sessions and models using either **Candle** (pure Rust ML framework) or **ONNX Runtime**.

### API Reference:
* `ml.backend_info()`: Returns status of available backends.
* `ml.create(session_name, backend)`: Initializes a session (e.g. using backend `"candle"` or `"onnx"`).
* `ml.load_model(model_name, path, backend)`: Registers a model weights file.
* `ml.infer(session_name, model_name, input_data)`: Runs model inference.

```aly
# Check supported backends
print(ml.backend_info())

# Setup session
let session = ml.create("sess_0", "candle")
ml.load_model("my_model", "models/weights.bin", "candle")

# Run inference
let result = ml.infer("sess_0", "my_model", "sample text data")
print("Inference output: $result")
```

---

## 2. LLMs and GGUF (`llm`)

For text generation and conversational AI, Aly features bindings to load local GGUF models.

### API Reference:
* `llm.load_gguf(model_name, filepath, context_size)`: Loads a GGUF weights file into memory.
* `llm.generate(model_name, prompt, max_tokens)`: Generates text completion.
* `llm.chat(model_name, system_prompt, user_prompt)`: Executes conversational instruction turns.

```aly
# Load model
llm.load_gguf("llama3", "models/llama3-8b.gguf", 2048)

# Generate response
let prompt = "Explain recursion in one sentence."
let answer = llm.generate("llama3", prompt, 128)
print("Answer: $answer")
```

---

## 3. Text Embeddings (`embed`)

Generate vector representations of text for semantic search and classification.

### API Reference:
* `embed.create(model_name, backend, dimension)`: Configures a vector embedding model.
* `embed.encode(model_name, text, normalize_bool)`: Computes text embedding vector.

```aly
embed.create("bert", "candle", 384)
let vector = embed.encode("bert", "Hello Aly", true)
print("Vector representation: $vector")
```
