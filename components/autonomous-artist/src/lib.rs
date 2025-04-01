#[allow(warnings)]
mod bindings;
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
// Or you can import the types from a solidity file with sol!("../path/to/file.sol");
sol! {
    // Keep type definitions for internal use
    type TriggerId is uint64;

    enum TriggerType {
        MINT,
        UPDATE
    }

    #[derive(Debug)]
    struct WavsMintResult {
        TriggerId triggerId;
        address recipient;
        string tokenURI;
    }

    // Refactor event to use primitive types
    event AvsMintTrigger(
        address indexed sender,
        string prompt,
        uint64 indexed triggerId,
        uint8 triggerType
    );
}

struct Component;

impl Guest for Component {
    /// @dev This function is called when a WAVS trigger action is fired.
    fn run(action: TriggerAction) -> std::result::Result<Option<Vec<u8>>, String> {
        // Decode the trigger event
        let AvsMintTrigger { sender, prompt, triggerId, triggerType: _ } = match action.data {
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
                WavsMintResult {
                    triggerId: triggerId.into(),
                    recipient: sender,
                    tokenURI: data_uri,
                }
                .abi_encode(),
            ))
        })
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
