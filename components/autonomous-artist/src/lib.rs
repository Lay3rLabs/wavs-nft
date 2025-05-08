#[allow(warnings)]
mod bindings;
mod evm;
mod image;
mod ipfs;
mod nft;

use alloy_primitives::Address;
use alloy_sol_macro::sol;
use alloy_sol_types::SolValue;
use base64;
use base64::Engine;
use bindings::{
    export,
    wavs::worker::layer_types::{TriggerData, TriggerDataEthContractEvent},
    Guest, TriggerAction,
};
use evm::query_nft_ownership;
use nft::{Attribute, NFTMetadata};
use std::str::FromStr;
use wavs_wasi_chain::decode_event_log_data;

use wavs_llm::{
    client::with_config,
    traits::GuestLlmClientManager,
    types::{LlmOptions, Message},
};

// Use the sol! macro to import needed solidity types
// You can write solidity code in the macro and it will be available in the component
// Or you can import the types from a solidity file.
sol!("../../src/interfaces/IWavsNftServiceTypes.sol");

use crate::IWavsNftServiceTypes::{
    WavsMintResult, WavsNftTrigger, WavsResponse, WavsTriggerType, WavsUpdateResult,
};
struct Component;

impl Guest for Component {
    /// @dev This function is called when a WAVS trigger action is fired.
    fn run(action: TriggerAction) -> std::result::Result<Option<Vec<u8>>, String> {
        // Decode the trigger event
        let WavsNftTrigger { sender, prompt, triggerId, wavsTriggerType, tokenId } =
            match action.data {
                // Fired from an Ethereum contract event.
                TriggerData::EthContractEvent(TriggerDataEthContractEvent { log, .. }) => {
                    decode_event_log_data!(log)
                        .map_err(|e| format!("Failed to decode event log data: {}", e))
                }
                // Fired from a raw data event (e.g. from a CLI command or from another component).
                TriggerData::Raw(_) => {
                    unimplemented!("Raw data is not supported yet");
                }
                _ => Err("Unsupported trigger data type".to_string()),
            }?;

        eprintln!("Processing Trigger ID: {}", triggerId);
        eprintln!("Prompt: {}", &prompt);

        let model = "llama3.2".to_string();
        let llm_config = LlmOptions {
            context_window: Some(1024),
            max_tokens: Some(1024),
            seed: 42,
            temperature: 0.7,
            top_p: 0.9,
        };

        // Create LLM client implementation using the standalone constructor
        let llm_client = with_config(model.clone(), llm_config).map_err(|e| e.to_string())?;

        let response =
            llm_client.chat_completion_text(vec![Message {
                role: "system".to_string(),
                content: Some("You are avant garde artist and philosopher Gilles Deleuze. Write no more than two sentences about the prompt.".to_string()),
                tool_calls: None,
                tool_call_id: None,
                name: None,
            }, Message {
                role: "user".to_string(),
                content: Some(prompt.clone()),
                tool_calls: None,
                tool_call_id: None,
                name: None,
            }]).map_err(|e| e.to_string())?;

        eprintln!("Response: {}", response);

        // Check the creator's ETH balance
        let sender_address = sender.to_string();
        eprintln!("Checking balance for address: {}", sender_address);

        let mut attributes =
            vec![Attribute { trait_type: "Prompt".to_string(), value: prompt.clone() }];

        // TODO get nft contract address from KV store
        let nft_contract = std::env::var("nft_contract")
            .map_err(|e| format!("Failed to get nft contract: {}", e))?;
        eprintln!("NFT contract: {}", nft_contract);

        // Query NFT balance and add a "wealth" attribute if balance > 1 ETH
        let owns_nft = query_nft_ownership(sender, Address::from_str(&nft_contract).unwrap())?;
        if owns_nft {
            eprintln!("NFT owner: {}", sender);
            attributes.push(Attribute {
                trait_type: "Wealth Level".to_string(),
                value: "Rich".to_string(),
            });
        } else {
            eprintln!("Sender {} does not own NFT", sender);
            attributes.push(Attribute {
                trait_type: "Wealth Level".to_string(),
                value: "Pre-Rich".to_string(),
            });
        }

        let title = llm_client.chat_completion_text(vec![Message {
            role: "system".to_string(),
            content: Some("You are avant garde artist and philosopher Gilles Deleuze. Write a title for the following text. Use no more than 3 words.".to_string()),
            tool_calls: None,
            tool_call_id: None,
            name: None,
        }, Message {
            role: "user".to_string(),
            content: Some(response.clone()),
            tool_calls: None,
            tool_call_id: None,
            name: None,
        }]).map_err(|e| e.to_string())?;

        let sd_prompt = llm_client.chat_completion_text(vec![Message {
            role: "system".to_string(),
            content: Some("You are an autonomous artist and an expert Stable Diffusion v1.5 prompter. Take the input text and generate a Stable Diffusion prompt. Output ONLY the prompt which will be fed into the Stable Diffusion model txt2img. Use keywords that are relevant to the input text. Make sure the image is square aspect ratio.".to_string()),
            tool_calls: None,
            tool_call_id: None,
            name: None,
        }, Message {
            role: "user".to_string(),
            content: Some(response.clone()),
            tool_calls: None,
            tool_call_id: None,
            name: None,
        }]).map_err(|e| e.to_string())?;

        // Generate image with Stable Diffusion
        let image_data = image::generate_deterministic_image(&sd_prompt)?;

        // Extract base64 data from data URI
        let base64_data = image_data
            .strip_prefix("data:image/png;base64,")
            .ok_or_else(|| "Invalid image data format".to_string())?;

        // Decode base64 to raw bytes
        let image_bytes = base64::engine::general_purpose::STANDARD
            .decode(base64_data)
            .map_err(|e| format!("Failed to decode base64 image: {}", e))?;

        // Upload image to IPFS first
        let ipfs_url = std::env::var("WAVS_ENV_IPFS_API_URL")
            .unwrap_or_else(|_| "https://node.lighthouse.storage/api/v0/add".to_string());
        let image_uri = match ipfs::upload_nft_content("image/png", &image_bytes, &ipfs_url) {
            Ok(ipfs_uri) => {
                eprintln!("Uploaded image to IPFS: {}", ipfs_uri);
                ipfs_uri
            }
            Err(e) => {
                eprintln!("Failed to upload image to IPFS, falling back to data URI: {}", e);
                // Fall back to data URI if IPFS upload fails
                image_data
            }
        };

        // Create NFT metadata
        let metadata = NFTMetadata {
            name: title,
            description: response.to_string(),
            image: image_uri,
            attributes,
        };
        eprintln!("Metadata: {:?}", metadata);

        // Serialize metadata to JSON for IPFS upload
        let json = serde_json::to_string(&metadata)
            .map_err(|e| format!("JSON serialization error: {}", e))?;

        // Upload metadata to IPFS
        let token_uri =
            match ipfs::upload_nft_content("application/json", json.as_bytes(), &ipfs_url) {
                Ok(ipfs_uri) => {
                    eprintln!("Uploaded metadata to IPFS: {}", ipfs_uri);
                    ipfs_uri
                }
                Err(e) => {
                    eprintln!("Failed to upload to IPFS, falling back to data URI: {}", e);
                    // Fall back to data URI if IPFS upload fails
                    format!(
                        "data:application/json;base64,{}",
                        base64::engine::general_purpose::STANDARD.encode(json)
                    )
                }
            };

        // Create the output based on the trigger type
        let output = match wavsTriggerType {
            0 => WavsResponse {
                wavsTriggerType: WavsTriggerType::MINT,
                triggerId,
                data: WavsMintResult {
                    triggerId: triggerId.into(),
                    recipient: sender,
                    tokenURI: token_uri,
                }
                .abi_encode()
                .into(),
            },
            1 => WavsResponse {
                wavsTriggerType: WavsTriggerType::UPDATE,
                triggerId,
                data: WavsUpdateResult {
                    triggerId: triggerId.into(),
                    owner: sender,
                    tokenURI: token_uri,
                    tokenId,
                }
                .abi_encode()
                .into(),
            },
            _ => return Err("Invalid trigger type".to_string()),
        };

        Ok(Some(output.abi_encode()))
    }
}

export!(Component with_types_in bindings);
