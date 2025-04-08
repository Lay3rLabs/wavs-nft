use crate::bindings::host::get_eth_chain_config;
use alloy_network::Ethereum;
use alloy_primitives::{Address, TxKind, U256};
use alloy_provider::{Provider, RootProvider};
use alloy_rpc_types::TransactionInput;
use alloy_sol_types::{sol, SolCall};
use wavs_wasi_chain::ethereum::new_eth_provider;

sol! {
    interface IERC721 {
        function balanceOf(address owner) external view returns (uint256);
    }
}

pub async fn query_nft_ownership(address: Address, nft_contract: Address) -> Result<bool, String> {
    let chain_config = get_eth_chain_config("local").unwrap();
    let provider: RootProvider<Ethereum> =
        new_eth_provider::<Ethereum>(chain_config.http_endpoint.unwrap());

    let balance_call = IERC721::balanceOfCall { owner: address };
    let tx = alloy_rpc_types::eth::TransactionRequest {
        to: Some(TxKind::Call(nft_contract)),
        input: TransactionInput { input: Some(balance_call.abi_encode().into()), data: None },
        ..Default::default()
    };

    let result = provider.call(&tx).await.map_err(|e| e.to_string())?;
    let balance: U256 = U256::from_be_slice(&result);
    Ok(balance > U256::ZERO)
}
