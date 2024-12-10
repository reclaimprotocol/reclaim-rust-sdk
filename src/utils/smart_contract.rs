use std::{cell::RefCell, collections::HashMap, sync::Arc};

use ethers::{
    providers::{Http, Provider},
    types::U256,
    types::H160,
};

use ethers::contract::Contract;
use ethers::abi::Abi;

use super::errors::*;
use super::interface::{BeaconImpl, BeaconState, WitnessData};
use crate::contract_data::abi::ABI;

thread_local! {
    static CONTRACT_CONFIG: RefCell<HashMap<String, HashMap<String, String>>> = RefCell::new({
        let mut config = HashMap::new();

        // opt-goerli
        let mut opt_goerli = HashMap::new();
        opt_goerli.insert("chainName".to_string(), "opt-goerli".to_string());
        opt_goerli.insert("address".to_string(), "0xF93F605142Fb1Efad7Aa58253dDffF67775b4520".to_string());
        opt_goerli.insert("rpcUrl".to_string(), "https://opt-goerli.g.alchemy.com/v2/rksDkSUXd2dyk2ANy_zzODknx_AAokui".to_string());
        config.insert("0x1a4".to_string(), opt_goerli);

        // opt-sepolia
        let mut opt_sepolia = HashMap::new();
        opt_sepolia.insert("chainName".to_string(), "opt-sepolia".to_string());
        opt_sepolia.insert("address".to_string(), "0x6D0f81BDA11995f25921aAd5B43359630E65Ca96".to_string());
        opt_sepolia.insert("rpcUrl".to_string(), "https://opt-sepolia.g.alchemy.com/v2/aO1-SfG4oFRLyAiLREqzyAUu0HTCwHgs".to_string());
        config.insert("0xaa37dc".to_string(), opt_sepolia);

        config
    });

    static EXISTING_CONTRACTS_MAP: RefCell<HashMap<String, Contract<Provider<Http>>>> = RefCell::new(HashMap::new());

    static DEFAULT_CHAIN_ID: u64 = 11155420;
}

pub fn get_contract(chain_id: u64) -> Result<Contract<Provider<Http>>, ProofNotVerifiedError> {
    let chain_key = format!("0x{:x}", chain_id);

    // Try to get existing contract
    if let Some(contract) =
        EXISTING_CONTRACTS_MAP.with(|contracts| contracts.borrow().get(&chain_key).cloned())
    {
        return Ok(contract);
    }

    // Get contract data from config
    let contract_data = CONTRACT_CONFIG.with(|config| {
        config
            .borrow()
            .get(&chain_key)
            .cloned()
            .ok_or_else(|| ProofNotVerifiedError(format!("Unsupported chain: {}", chain_key)))
    })?;

    // Create new contract

    // Create provider from RPC URL if not exists then throw ProofNotVerifiedError error
    let provider = Arc::new(Provider::<Http>::try_from(
        contract_data["rpcUrl"].as_str(),
    )?);

    // Create contract using address and ABI
    let contract = Contract::new(
        contract_data["address"].parse::<H160>()?,
        serde_json::from_str::<Abi>(ABI)?,
        provider,
    );

    // Store in map and return
    EXISTING_CONTRACTS_MAP.with(|contracts| {
        contracts
            .borrow_mut()
            .insert(chain_key.clone(), contract.clone());
    });

    Ok(contract)
}

pub async fn fetch_epoch_data(contract: Contract<Provider<Http>>, epoch_id: u64) -> Result<BeaconState, ProofNotVerifiedError> {
    let response: (
        U256,  // id
        U256,  // timestampStart
        U256,  // timestampEnd
        Vec<(H160, String)>,  // witnesses (addr, host)
        U256,  // minimumWitnessesForClaimCreation
    ) = contract
        .method("fetchEpoch", epoch_id)
        .map_err(|e| ProofNotVerifiedError(format!("Failed to call fetchEpoch with error {}", e)))?
        .call()
        .await
        .map_err(|e| ProofNotVerifiedError(format!("Failed to fetch epoch data with error {}", e)))?;

    if response.0 == U256::from(0) {
        return Err(ProofNotVerifiedError(format!("Invalid epoch ID: {}", epoch_id)));
    }

    Ok(BeaconState {
        epoch: response.0.as_u64(),
        witnesses: response.3
            .into_iter()
            .map(|(address, url)| WitnessData { 
                id: format!("0x{:x}", address),
                url 
            })
            .collect(),
        witnesses_required_for_claim: response.4.as_u64(),
        next_epoch_timestamp_s: response.2.as_u64(),
    })
}

pub async fn make_beacon(chain_id: Option<u64>) -> Result<BeaconImpl, ProofNotVerifiedError> {
    let chain_id = chain_id.unwrap_or(DEFAULT_CHAIN_ID.with(|chain_id| *chain_id));
    let contract = get_contract(chain_id)?;
    let epoch_data = fetch_epoch_data(contract.clone(), 0).await?;
    Ok(BeaconImpl {
        contract,
        epoch_data,
    })
}
