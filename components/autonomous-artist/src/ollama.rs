use anyhow::Result;
use serde::Deserialize;
use wstd::{
    http::{Client, IntoBody, Request},
    io::AsyncRead,
};

// Ollama response structures
#[derive(Deserialize, Debug)]
#[serde(untagged)]
pub enum OllamaChatResponse {
    Success(OllamaChatSuccessResponse),
    Error { error: String },
}

#[derive(Deserialize, Debug)]
pub struct OllamaChatSuccessResponse {
    pub message: OllamaChatMessage,
}

#[derive(Deserialize, Debug)]
pub struct OllamaChatMessage {
    pub content: String,
}

pub async fn query_ollama(prompt: &str) -> Result<String, String> {
    let req = Request::post("http://localhost:11434/api/chat")
        .body(
            serde_json::to_vec(&serde_json::json!({
                // https://github.com/ollama/ollama/blob/main/docs/api.md
                "model": "llama3.1",
                "messages": [{
                    "role": "system",
                    "content": "You are an Avante Garde philosopher, Gilles Deleuze. Write only in Haiku."
                }, {
                    "role": "user",
                    "content": prompt
                }],

                // Core options for deterministic output
                "options": {
                    // Sampling strategy (deterministic focus)
                    "temperature": 0.0,        // [0.0-2.0] 0.0 for most deterministic
                    "top_k": 1,               // [1-100] 1 for strict selection
                    "top_p": 0.1,             // [0.0-1.0] 0.1 for narrow sampling
                    "min_p": 0.0,             // [0.0-1.0] Alternative to top_p (disabled)

                    // Context and length control
                    "num_ctx": 4096,          // [512-8192] Context window size
                    // Limited for haiku output
                    "num_predict": 75,       // [-1, 1-N] Max tokens to generate (-1 = infinite)

                    // Deterministic generation
                    "seed": 42,              // Fixed seed for reproducibility
                },

                // API behavior
                "stream": false,             // No streaming for consistent response
            }))
            .unwrap()
            .into_body(),
        )
        .unwrap();

    let mut res = Client::new().send(req).await.map_err(|e| e.to_string())?;

    if res.status() != 200 {
        return Err(format!("Ollama API error: status {}", res.status()));
    }

    let mut body_buf = Vec::new();
    res.body_mut().read_to_end(&mut body_buf).await.unwrap();

    let resp = String::from_utf8_lossy(&body_buf);
    let resp = serde_json::from_str::<OllamaChatResponse>(format!(r#"{}"#, resp).as_str());

    match resp {
        Ok(OllamaChatResponse::Success(success)) => Ok(success.message.content),
        Ok(OllamaChatResponse::Error { error }) => Err(error),
        Err(e) => Err(format!("Failed to parse response: {}", e)),
    }
}
