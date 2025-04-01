#[allow(warnings)]
mod bindings;
use alloy_sol_types::{SolType, SolValue};
use anyhow::Result;
use base64;
use base64::Engine;
use bindings::{export, Guest, TriggerAction};
use serde::{Deserialize, Serialize};
use wstd::{
    http::{Client, IntoBody, Request},
    io::AsyncRead,
    runtime::block_on,
};
mod trigger;
use trigger::{decode_trigger_event, encode_trigger_output};

// NFT Metadata structure
#[derive(Serialize)]
struct NFTMetadata {
    name: String,
    description: String,
    image: String,
    attributes: Vec<Attribute>,
}

#[derive(Serialize)]
struct Attribute {
    trait_type: String,
    value: String,
}

// Ollama response structures
#[derive(Deserialize, Debug)]
#[serde(untagged)]
enum OllamaChatResponse {
    Success(OllamaChatSuccessResponse),
    Error { error: String },
}

#[derive(Deserialize, Debug)]
struct OllamaChatSuccessResponse {
    message: OllamaChatMessage,
}

#[derive(Deserialize, Debug)]
struct OllamaChatMessage {
    content: String,
}

struct Component;

impl Guest for Component {
    fn run(action: TriggerAction) -> std::result::Result<Option<Vec<u8>>, String> {
        let trigger_info = decode_trigger_event(action.data)?;

        let prompt = trigger_info.data;

        block_on(async move {
            // Decode the ABI-encoded string first
            let decoded = alloy_sol_types::sol_data::String::abi_decode(&prompt, false)
                .map_err(|e| format!("Failed to decode ABI string: {}", e))?;

            // Query Ollama
            let response = query_ollama(&decoded.to_string()).await?;

            // Create NFT metadata
            let metadata = NFTMetadata {
                name: "AI Generated NFT".to_string(),
                description: response.to_string(),
                image: "ipfs://placeholder".to_string(),
                attributes: vec![Attribute {
                    trait_type: "Prompt".to_string(),
                    value: decoded.to_string(),
                }],
            };

            // Serialize to JSON and convert to data URI
            let json = serde_json::to_string(&metadata)
                .map_err(|e| format!("JSON serialization error: {}", e))?;
            let data_uri = format!(
                "data:application/json;base64,{}",
                base64::engine::general_purpose::STANDARD.encode(json)
            );

            Ok(Some(encode_trigger_output(trigger_info.triggerId, data_uri.abi_encode())))
        })
    }
}

async fn query_ollama(prompt: &str) -> Result<String, String> {
    // TODO experiment with generate endpoint
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

                // TODO: figure out how to use this, prompt should mention structured output
                // "format": "json",

                // Structured output control (haven't figured out how to use this yet)
                // "format": {
                //     "type": "object",
                //     "properties": {
                //         "name": { "type": "string" },
                //         "description": { "type": "string" },
                //     },
                //     "required": ["name", "description"]
                // },

                // Core options for deterministic output
                "options": {
                    // Sampling strategy (deterministic focus)
                    "temperature": 0.0,        // [0.0-2.0] 0.0 for most deterministic
                    "top_k": 1,               // [1-100] 1 for strict selection
                    "top_p": 0.1,             // [0.0-1.0] 0.1 for narrow sampling
                    "min_p": 0.0,             // [0.0-1.0] Alternative to top_p (disabled)

                    // Repetition control
                    // "repeat_last_n": 64,      // [-1, 0-N] tokens to look back (-1 = num_ctx)
                    // "repeat_penalty": 1.2,     // [0.0-2.0] Higher = less repetition

                    // Mirostat sampling (alternative to temperature)
                    // "mirostat": 0,         // [0-2] 0=disabled, 1=v1, 2=v2
                    // "mirostat_tau": 5.0,   // [0.0-10.0] Lower = more focused
                    // "mirostat_eta": 0.1,   // [0.0-1.0] Learning rate

                    // Context and length control
                    "num_ctx": 4096,          // [512-8192] Context window size
                    // TODO bump this number when we can support higher per service gas configs
                    "num_predict": 75,       // [-1, 1-N] Max tokens to generate (-1 = infinite)

                    // Stop sequences (model-specific)
                    // "stop": [
                    //     "\n\n",              // Common stop
                    //     "###",               // Common stop
                    //     "<|im_start|>",      // Chat format
                    //     "<|im_end|>",        // Chat format
                    //     "```",               // Code blocks
                    //     "USER:",             // Chat roles
                    //     "ASSISTANT:"         // Chat roles
                    // ],

                    // Deterministic generation
                    "seed": 42,              // Fixed seed for reproducibility

                    // System resource management
                    // "num_thread": 8,         // CPU threads to use
                    // "num_gpu": 1,            // Number of GPUs to use
                    // "num_batch": 2,       // Batch size for prompt processing
                    // "num_keep": 5,        // Number of tokens to keep from initial prompt

                    // Memory management
                    // "low_vram": false,    // Optimize for low VRAM GPUs
                    // "main_gpu": 0,        // Main GPU index
                    // "numa": false,        // NUMA acceleration
                    // "use_mmap": true,     // Memory-mapped I/O
                    // "use_mlock": false,   // Lock memory
                },

                // API behavior
                "stream": false,             // No streaming for consistent response
                // "keep_alive": "5m",         // Model keep-alive duration

                // Raw mode (bypass template system)
                // "raw": true,             // Enable if using custom prompt format
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

export!(Component with_types_in bindings);
