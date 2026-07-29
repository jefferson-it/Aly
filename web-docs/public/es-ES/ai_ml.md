# Inteligencia Artificial y Aprendizaje Automático en Aly

Aly proporciona módulos nativos (`ml.*`, `llm.*`, `embed.*`) para interfaz con backends de IA/ML directamente desde scripts, permitiendo inferencia neuronal, operaciones de chat LLM y extracción de embeddings.

---

## 1. Aprendizaje Automático Principal (`ml`)

El módulo `ml` gestiona sesiones de inferencia y modelos utilizando ya sea **Candle** (framework de ML puro en Rust) o **ONNX Runtime**.

### Referencia de la API:
* `ml.backend_info()`: Devuelve el estado de los backends disponibles.
* `ml.create(session_name, backend)`: Inicializa una sesión (por ejemplo usando el backend `"candle"` o `"onnx"`).
* `ml.load_model(model_name, path, backend)`: Registra un archivo de pesos del modelo.
* `ml.infer(session_name, model_name, input_data)`: Ejecuta la inferencia del modelo.

```aly
# Verificar backends compatibles
print(ml.backend_info())

# Configurar sesión
let session = ml.create("sess_0", "candle")
ml.load_model("my_model", "models/weights.bin", "candle")

# Ejecutar inferencia
let result = ml.infer("sess_0", "my_model", "sample text data")
print("Resultado de la inferencia: $result")
```

---

## 2. LLMs y GGUF (`llm`)

Para generación de texto e IA conversacional, Aly cuenta con enlaces para cargar modelos GGUF locales.

### Referencia de la API:
* `llm.load_gguf(model_name, filepath, context_size)`: Carga un archivo de pesos GGUF en memoria.
* `llm.generate(model_name, prompt, max_tokens)`: Genera completado de texto.
* `llm.chat(model_name, system_prompt, user_prompt)`: Ejecuta turnos de instrucción conversacional.

```aly
# Cargar modelo
llm.load_gguf("llama3", "models/llama3-8b.gguf", 2048)

# Generar respuesta
let prompt = "Explain recursion in one sentence."
let answer = llm.generate("llama3", prompt, 128)
print("Respuesta: $answer")
```

---

## 3. embeddings de Texto (`embed`)

Generar representaciones vectoriales de texto para búsqueda semántica y clasificación.

### Referencia de la API:
* `embed.create(model_name, backend, dimension)`: Configura un modelo de embedding vectorial.
* `embed.encode(model_name, text, normalize_bool)`: Calcula el vector de embedding de texto.

```aly
embed.create("bert", "candle", 384)
let vector = embed.encode("bert", "Hello Aly", true)
print("Representación vectorial: $vector")
```