#[allow(warnings)]
mod bindings;
use alloy_sol_macro::sol;
use alloy_sol_types::SolValue;
use anyhow::Result;
use base64;
use base64::Engine;
use bindings::{
    export,
    wavs::worker::layer_types::{TriggerData, TriggerDataEthContractEvent},
    Guest, TriggerAction,
};
use serde::{Deserialize, Serialize};
use wavs_wasi_chain::decode_event_log_data;
use wstd::{
    http::{Client, IntoBody, Request},
    io::AsyncRead,
    runtime::block_on,
};

sol! {
    type TriggerId is uint64;

    enum TriggerType {
        MINT,
        UPDATE
    }

    struct WavsMintResult {
        TriggerId triggerId;
        address recipient;
        string tokenURI;
    }

    event AvsMintTrigger(
        address indexed sender,
        string prompt,
        TriggerId indexed triggerId,
        TriggerType triggerType
    );
}

// NFT Metadata structure
#[derive(Serialize, Debug)]
struct NFTMetadata {
    name: String,
    description: String,
    image: String,
    attributes: Vec<Attribute>,
}

#[derive(Serialize, Debug)]
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
        // Decode the trigger event
        let AvsMintTrigger { sender, prompt, triggerId, triggerType } = match action.data {
            // Fired from an Ethereum contract event.
            TriggerData::EthContractEvent(TriggerDataEthContractEvent { log, .. }) => {
                decode_event_log_data!(log)
                    .map_err(|e| format!("Failed to decode event log data: {}", e))
            }
            // Fired from a raw data event (e.g. from a CLI command or from another component).
            TriggerData::Raw(data) => {
                unimplemented!("Raw data is not supported yet");
            }
            _ => Err("Unsupported trigger data type".to_string()),
        }?;

        eprintln!("Processing Trigger ID: {}", triggerId);
        eprintln!("Prompt: {}", &prompt);

        block_on(async move {
            // Query Ollama
            let response = query_ollama(&prompt).await?;
            eprintln!("Response: {}", response);

            // TODO generate more interesting attributes
            // TODO query eth balance of creator, if > 1, add "rich" attribute

            let attributes = vec![Attribute { trait_type: "Prompt".to_string(), value: prompt }];

            // Create NFT metadata
            let metadata = NFTMetadata {
                name: "AI Generated NFT".to_string(),
                description: response.to_string(),
                image: "ipfs://placeholder".to_string(),
                attributes,
            };
            eprintln!("Metadata: {:?}", metadata);

            // Serialize to JSON and convert to data URI
            let json = serde_json::to_string(&metadata)
                .map_err(|e| format!("JSON serialization error: {}", e))?;
            let data_uri = format!(
                "data:application/json;base64,{}",
                base64::engine::general_purpose::STANDARD.encode(json)
            );
            eprintln!("Data URI: {}", data_uri);

            Ok(Some(
                WavsMintResult { triggerId, recipient: sender, tokenURI: data_uri }.abi_encode(),
            ))
        })
    }
}

async fn query_ollama(prompt: &str) -> Result<String, String> {
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

// // Enum to specify the destination of the trigger
// #[derive(Debug)]
// pub enum Destination {
//     Ethereum,
//     CliOutput,
// }

// pub fn encode_trigger_output(trigger_id: u64, output: impl AsRef<[u8]>) -> Vec<u8> {
//     solidity::DataWithId { triggerId: trigger_id, data: output.as_ref().to_vec().into() }
//         .abi_encode()
// }

// mod solidity {
//     use alloy_sol_macro::sol;
//     pub use ITypes::*;

//     sol!("../../src/interfaces/ITypes.sol");
// }

export!(Component with_types_in bindings);
