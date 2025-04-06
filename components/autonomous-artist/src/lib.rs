#[allow(warnings)]
mod bindings;
mod ipfs;
mod nft;
mod ollama;

use alloy_sol_macro::sol;
use alloy_sol_types::SolValue;
use base64;
use base64::Engine;
use bindings::{
    export,
    wavs::worker::layer_types::{TriggerData, TriggerDataEthContractEvent},
    Guest, TriggerAction,
};
use nft::{Attribute, NFTMetadata};
use wavs_wasi_chain::decode_event_log_data;
use wstd::runtime::block_on;

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

        block_on(async move {
            // Query Ollama
            let response = ollama::query_ollama(&prompt).await?;
            eprintln!("Response: {}", response);

            // Check the creator's ETH balance
            let sender_address = sender.to_string();
            eprintln!("Checking balance for address: {}", sender_address);

            let attributes = vec![Attribute { trait_type: "Prompt".to_string(), value: prompt }];

            // TODO Query ETH balance and add a "wealth" attribute if balance > 1 ETH

            // Create NFT metadata
            let metadata = NFTMetadata {
                name: "AI Generated NFT".to_string(),
                description: response.to_string(),
                image: "ipfs://placeholder".to_string(), // Will be updated with real IPFS URI
                attributes,
            };
            eprintln!("Metadata: {:?}", metadata);

            // Get IPFS URL from environment or use default
            let ipfs_url = std::env::var("WAVS_ENV_IPFS_API_URL")
                .unwrap_or_else(|_| "https://node.lighthouse.storage/api/v0/add".to_string());

            // Try to generate an image based on the prompt (this part is optional)
            // In a full implementation, you might use an AI model to generate an image
            // For now, we'll skip this part

            // Serialize metadata to JSON for IPFS upload
            let json = serde_json::to_string(&metadata)
                .map_err(|e| format!("JSON serialization error: {}", e))?;

            // Upload metadata to IPFS
            let token_uri = match ipfs::upload_nft_content(
                "application/json",
                json.as_bytes(),
                &ipfs_url,
            )
            .await
            {
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
        })
    }
}

export!(Component with_types_in bindings);
