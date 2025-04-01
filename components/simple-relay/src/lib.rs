#[allow(warnings)]
mod bindings;
use alloy_sol_types::sol;
use bindings::{
    export,
    wavs::worker::layer_types::{TriggerData, TriggerDataEthContractEvent},
    Guest, TriggerAction,
};
use wavs_wasi_chain::decode_event_log_data;

sol! {
    type TriggerId is uint64;

    #[derive(Debug)]
    event NFTMinted(
        address indexed to,
        uint256 indexed tokenId,
        string dataUri,
        uint64 triggerId
    );
}

struct Component;

impl Guest for Component {
    fn run(trigger_action: TriggerAction) -> std::result::Result<Option<Vec<u8>>, String> {
        match trigger_action.data {
            TriggerData::EthContractEvent(TriggerDataEthContractEvent { log, .. }) => {
                // Decode the NewTrigger event to get the _triggerInfo bytes
                let NFTMinted { triggerId, .. } = decode_event_log_data!(log)
                    .map_err(|e| format!("Failed to decode event log data: {}", e))?;

                eprintln!("Processing Trigger ID: {}", triggerId);

                // Return the ABI-encoded triggerId
                Ok(Some(TriggerId(triggerId).abi_encode()))
            }
            _ => Err("Unsupported trigger data".to_string()),
        }
    }
}

export!(Component with_types_in bindings);
