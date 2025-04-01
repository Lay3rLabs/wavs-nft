use crate::bindings::wavs::worker::layer_types::{TriggerData, TriggerDataEthContractEvent};
use alloy_sol_types::SolValue;
use anyhow::Result;
use solidity::TriggerInfo;
use wavs_wasi_chain::decode_event_log_data;

pub fn decode_trigger_event(trigger_data: TriggerData) -> Result<TriggerInfo, String> {
    match trigger_data {
        TriggerData::EthContractEvent(TriggerDataEthContractEvent { log, .. }) => {
            let event: solidity::NewTrigger =
                decode_event_log_data!(log).map_err(|e| e.to_string())?;
            solidity::TriggerInfo::abi_decode(&event._0, false).map_err(|e| e.to_string())
        }
        _ => Err("Unsupported trigger data type".to_string()),
    }
}

pub fn encode_trigger_output(trigger_id: u64, output: impl AsRef<[u8]>) -> Vec<u8> {
    solidity::DataWithId { triggerId: trigger_id, data: output.as_ref().to_vec().into() }
        .abi_encode()
}

mod solidity {
    use alloy_sol_macro::sol;
    pub use ITypes::*;

    // imports DataWithId, TriggerInfo, NewTrigger, and TriggerId
    sol!("../../src/interfaces/ITypes.sol");
}
