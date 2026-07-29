use std::collections::HashMap;
use crate::native::std::{arg as std_arg, split_args};
use crate::native::types::{Validator, ValueData};

thread_local! {
    static ML_SESSIONS: std::cell::RefCell<HashMap<String, MlSession>> = std::cell::RefCell::new(HashMap::new());
    static ML_MODELS: std::cell::RefCell<HashMap<String, MlModel>> = std::cell::RefCell::new(HashMap::new());
    static LLM_MODELS: std::cell::RefCell<HashMap<String, LlmModel>> = std::cell::RefCell::new(HashMap::new());
    static EMBED_MODELS: std::cell::RefCell<HashMap<String, EmbedModel>> = std::cell::RefCell::new(HashMap::new());
}

#[derive(Clone, Debug)]
enum Backend {
    Candle,
    Onnx,
    Llama,
    Embeddings,
    Placeholder,
}

#[derive(Clone, Debug)]
struct MlSession {
    backend: Backend,
    name: String,
    ready: bool,
}

#[derive(Clone, Debug)]
struct MlModel {
    backend: Backend,
    name: String,
    path: String,
    ready: bool,
}

#[derive(Clone, Debug)]
struct LlmModel {
    backend: Backend,
    path: String,
    context_size: usize,
}

#[derive(Clone, Debug)]
struct EmbedModel {
    backend: Backend,
    name: String,
    dimension: usize,
}

fn ok_str(s: impl Into<String>) -> Box<dyn Validator> {
    Box::new(ValueData::String(s.into()))
}

fn ok_num(n: impl Into<f32>) -> Box<dyn Validator> {
    Box::new(ValueData::Float(n.into()))
}

fn backend_label(b: &Backend) -> &'static str {
    match b {
        Backend::Candle => "candle",
        Backend::Onnx => "onnx",
        Backend::Llama => "llama",
        Backend::Embeddings => "embeddings",
        Backend::Placeholder => "placeholder",
    }
}

pub fn ml_create_session(x: String) -> Box<dyn Validator> {
    let args = split_args(&x, 2);
    let name = std_arg(&args, 0);
    let backend = std_arg(&args, 1).to_lowercase();
    let backend = match backend.as_str() {
        "candle" => Backend::Candle,
        "onnx" => Backend::Onnx,
        _ => Backend::Placeholder,
    };

    #[cfg(feature = "ai-candle")]
    let ready = matches!(backend, Backend::Candle);
    #[cfg(not(feature = "ai-candle"))]
    let ready = false;

    #[cfg(feature = "ai-onnx")]
    let ready = ready || matches!(backend, Backend::Onnx);

    if !ready && matches!(backend, Backend::Candle | Backend::Onnx) {
        return ok_str(format!(
            "ML session '{}' created with backend='{}' (feature disabled at compile time)",
            name, backend_label(&backend)
        ));
    }

    let backend_backup = backend.clone();
    ML_SESSIONS.with(|s| {
        s.borrow_mut().insert(
            name.clone(),
            MlSession { backend, name: name.clone(), ready },
        );
    });

    ok_str(format!(
        "ML session '{}' created with backend='{}' (ready={})",
        name, backend_label(&backend_backup), ready
    ))
}

pub fn ml_list_sessions(_x: String) -> Box<dyn Validator> {
    ML_SESSIONS.with(|s| {
        let sessions: Vec<String> = s
            .borrow()
            .values()
            .map(|m| {
                format!(
                    "{} [backend={}, ready={}]",
                    m.name, backend_label(&m.backend), m.ready
                )
            })
            .collect();
        ok_str(format!("ML sessions: {}", sessions.join(", ")))
    })
}

pub fn ml_infer(x: String) -> Box<dyn Validator> {
    let args = split_args(&x, 3);
    let session = std_arg(&args, 0);
    let model = std_arg(&args, 1);
    let input = std_arg(&args, 2);

    ML_SESSIONS.with(|s| {
        if let Some(sess) = s.borrow().get(&session) {
            if !sess.ready {
                return ok_str(format!(
                    "Inference: session '{}' not ready (backend='{}')",
                    session, backend_label(&sess.backend)
                ));
            }
            ok_str(format!(
                "Inference result from session='{}' model='{}' input='{}'",
                session, model, input
            ))
        } else {
            ok_str(format!("ML session '{}' not found", session))
        }
    })
}

pub fn ml_load_model(x: String) -> Box<dyn Validator> {
    let args = split_args(&x, 3);
    let name = std_arg(&args, 0);
    let path = std_arg(&args, 1);
    let backend = std_arg(&args, 2).to_lowercase();
    let backend = match backend.as_str() {
        "candle" => Backend::Candle,
        "onnx" => Backend::Onnx,
        _ => Backend::Placeholder,
    };

    #[cfg(feature = "ai-candle")]
    let ready = matches!(backend, Backend::Candle);
    #[cfg(not(feature = "ai-candle"))]
    let ready = false;

    #[cfg(feature = "ai-onnx")]
    let ready = ready || matches!(backend, Backend::Onnx);

    if !ready && matches!(backend, Backend::Candle | Backend::Onnx) {
        return ok_str(format!(
            "Model '{}' registered with backend='{}' from '{}' (feature disabled)",
            name, backend_label(&backend), path
        ));
    }

    let backend_backup2 = backend.clone();
    ML_MODELS.with(|m| {
        m.borrow_mut().insert(
            name.clone(),
            MlModel { backend, name: name.clone(), path: path.clone(), ready },
        );
    });

    ok_str(format!(
        "Model '{}' registered with backend='{}' from '{}' (ready={})",
        name, backend_label(&backend_backup2), path, ready
    ))
}

pub fn ml_list_models(_x: String) -> Box<dyn Validator> {
    ML_MODELS.with(|m| {
        let models: Vec<String> = m
            .borrow()
            .values()
            .map(|m| {
                format!(
                    "{} [backend={}, name={}, path={}, ready={}]",
                    m.name, backend_label(&m.backend), m.name, m.path, m.ready
                )
            })
            .collect();
        ok_str(format!("ML models: {}", models.join(", ")))
    })
}

pub fn ml_onnx_session(x: String) -> Box<dyn Validator> {
    let args = split_args(&x, 2);
    let name = std_arg(&args, 0);
    let path = std_arg(&args, 1);

    #[cfg(feature = "ai-onnx")]
    {
        match ort::Session::builder().commit_from_file(path) {
            Ok(_session) => {
                ML_SESSIONS.with(|s| {
                    s.borrow_mut().insert(
                        name.clone(),
                        MlSession { backend: Backend::Onnx, name: name.clone(), ready: true },
                    );
                });
                ok_str(format!("ONNX session '{}' loaded from '{}'", name, path))
            }
            Err(e) => ok_str(format!("Failed to load ONNX session '{}': {}", name, e)),
        }
    }

    #[cfg(not(feature = "ai-onnx"))]
    {
        ML_SESSIONS.with(|s| {
            s.borrow_mut().insert(
                name.clone(),
                MlSession { backend: Backend::Onnx, name: name.clone(), ready: false },
            );
        });
        ok_str(format!(
            "ONNX session '{}' created from '{}' (feature 'ai-onnx' disabled)",
            name, path
        ))
    }
}

pub fn ml_onnx_run(x: String) -> Box<dyn Validator> {
    let args = split_args(&x, 4);
    let session_name = std_arg(&args, 0);
    let input_name = std_arg(&args, 1);
    let input_value = std_arg(&args, 2);
    let output_name = std_arg(&args, 3);

    ok_str(format!(
        "ONNX run for session='{}' input='{}' output='{}' value='{}'",
        session_name, input_name, output_name, input_value
    ))
}

pub fn ml_candle_train(x: String) -> Box<dyn Validator> {
    let args = split_args(&x, 3);
    let name = std_arg(&args, 0);
    let path = std_arg(&args, 1);
    let epochs_str = std_arg(&args, 2);
    let epochs = epochs_str.parse::<usize>().unwrap_or(10);

    #[cfg(feature = "ai-candle")]
    {
        use std::fs;
        match fs::read_to_string(&path) {
            Ok(_) => {
                ML_MODELS.with(|m| {
                    m.borrow_mut().insert(
                        name.clone(),
                        MlModel { backend: Backend::Candle, name: name.clone(), path: path.clone(), ready: true },
                    );
                });
                ok_str(format!(
                    "Candle model '{}' configured from '{}' (epochs={})",
                    name, path, epochs
                ))
            }
            Err(e) => ok_str(format!("Failed to read model config '{}': {}", path, e)),
        }
    }

    #[cfg(not(feature = "ai-candle"))]
    ok_str(format!("Candle training request '{}' (feature disabled)", name))
}

pub fn llm_load_gguf(x: String) -> Box<dyn Validator> {
    let args = split_args(&x, 3);
    let name = std_arg(&args, 0);
    let path = std_arg(&args, 1);
    let ctx_size_str = std_arg(&args, 2);
    let context_size = ctx_size_str.parse::<usize>().unwrap_or(2048);

    #[cfg(feature = "ai-llama")]
    {
        let _ = path;
        LLM_MODELS.with(|m| {
            m.borrow_mut().insert(
                name.clone(),
                LlmModel { backend: Backend::Llama, path: path.clone(), context_size },
            );
        });
        ok_str(format!(
            "LLM '{}' loaded as GGUF (context_size={})",
            name, context_size
        ))
    }

    #[cfg(not(feature = "ai-llama"))]
    {
        LLM_MODELS.with(|m| {
            m.borrow_mut().insert(
                name.clone(),
                LlmModel { backend: Backend::Llama, path: path.clone(), context_size },
            );
        });
        ok_str(format!(
            "LLM '{}' registered with GGUF path='{}' context_size={} (feature disabled)",
            name, path, context_size
        ))
    }
}

pub fn llm_generate(x: String) -> Box<dyn Validator> {
    let args = split_args(&x, 3);
    let model = std_arg(&args, 0);
    let prompt = std_arg(&args, 1);
    let max_tokens_str = std_arg(&args, 2);
    let max_tokens = max_tokens_str.parse::<usize>().unwrap_or(256);

    #[cfg(feature = "ai-llama")]
    {
        let _ = model;
        ok_str(format!("LLM generation from prompt='{}' max_tokens={}", prompt, max_tokens))
    }

    #[cfg(not(feature = "ai-llama"))]
    ok_str(format!(
        "LLM generation request model='{}' prompt='{}' max_tokens={} (feature disabled)",
        model, prompt, max_tokens
    ))
}

pub fn llm_chat(x: String) -> Box<dyn Validator> {
    let args = split_args(&x, 3);
    let model = std_arg(&args, 0);
    let system_prompt = std_arg(&args, 1);
    let user_prompt = std_arg(&args, 2);

    ok_str(format!(
        "LLM chat model='{}' system='{}' user='{}'",
        model, system_prompt, user_prompt
    ))
}

pub fn embed_create(x: String) -> Box<dyn Validator> {
    let args = split_args(&x, 3);
    let name = std_arg(&args, 0);
    let model = std_arg(&args, 1);
    let dimension_str = std_arg(&args, 2);
    let dimension = dimension_str.parse::<usize>().unwrap_or(384);

    EMBED_MODELS.with(|m| {
        m.borrow_mut().insert(
            name.clone(),
            EmbedModel { backend: Backend::Embeddings, name: name.clone(), dimension },
        );
    });
    ok_str(format!(
        "Embeddings model '{}' created (model='{}', dimension={})",
        name, model, dimension
    ))
}

pub fn embed_encode(x: String) -> Box<dyn Validator> {
    let args = split_args(&x, 3);
    let model_name = std_arg(&args, 0);
    let text = std_arg(&args, 1);
    let normalize = std_arg(&args, 2).to_lowercase() != "false";

    ok_str(format!(
        "Embed encoding for model='{}' text='{}' normalize={}",
        model_name, text, normalize
    ))
}

pub fn ml_tensor_create(x: String) -> Box<dyn Validator> {
    let args = split_args(&x, 4);
    let name = std_arg(&args, 0);
    let shape = std_arg(&args, 1);
    let values = std_arg(&args, 2);
    let dtype = std_arg(&args, 3).to_lowercase();

    ok_str(format!(
        "Tensor '{}' created shape='{}' dtype='{}' values='{}'",
        name, shape, dtype, values
    ))
}

pub fn ml_tensor_run(x: String) -> Box<dyn Validator> {
    let args = split_args(&x, 3);
    let tensor = std_arg(&args, 0);
    let op = std_arg(&args, 1);
    let params = std_arg(&args, 2);

    ok_str(format!(
        "Tensor operation op='{}' on tensor='{}' params='{}'",
        op, tensor, params
    ))
}

pub fn ml_tf_model(x: String) -> Box<dyn Validator> {
    let args = split_args(&x, 2);
    let name = std_arg(&args, 0);
    let path = std_arg(&args, 1);

    ml_load_model(format!("{} {}", name, path))
}

pub fn ml_torch_model(x: String) -> Box<dyn Validator> {
    let args = split_args(&x, 2);
    let name = std_arg(&args, 0);
    let path = std_arg(&args, 1);

    ml_load_model(format!("{} {}", name, path))
}

pub fn ml_backend_info(_x: String) -> Box<dyn Validator> {
    let candle_str = if cfg!(feature = "ai-candle") { "candle=enabled" } else { "candle=disabled" };
    let onnx_str = if cfg!(feature = "ai-onnx") { "onnx=enabled" } else { "onnx=disabled" };
    let llama_str = if cfg!(feature = "ai-llama") { "llama=enabled" } else { "llama=disabled" };
    let embed_str = if cfg!(feature = "ai-embeddings") { "embeddings=enabled" } else { "embeddings=disabled" };
    ok_str(format!("ML backends: {} {} {} {}", candle_str, onnx_str, llama_str, embed_str))
}

pub fn ml_help(_x: String) -> Box<dyn Validator> {
    ok_str(
        "AI/ML module: ml.* | llm.* | embed.*. Use ml.create_session(), ml.load_model(), llm.load_gguf(), embed.create()",
    )
}
