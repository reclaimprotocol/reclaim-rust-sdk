use ethers::utils::keccak256;
use crate::utils::types::ClaimInfo;

use super::errors::ProofNotVerifiedError;
use super::interface::{ProviderClaimData, WitnessData, BeaconState};

pub fn fetch_witness_list_for_claim(
    state: &BeaconState,
    params: &str,
    timestamp_s: u64,
) -> Vec<WitnessData> {
    let identifier = params;
    let complete_input = format!(
        "{}\n{}\n{}\n{}",
        identifier, state.epoch, state.witnesses_required_for_claim, timestamp_s
    );
    
    let complete_hash = keccak256(complete_input.as_bytes());
    let mut witnesses_left = state.witnesses.clone();
    let mut selected_witnesses = Vec::new();
    let mut byte_offset = 0;

    for _ in 0..state.witnesses_required_for_claim {
        let random_seed = u32::from_be_bytes(complete_hash[byte_offset..byte_offset + 4].try_into().unwrap());
        let witness_index = (random_seed as usize) % witnesses_left.len();
        let witness = witnesses_left[witness_index].clone();
        selected_witnesses.push(witness);

        // Remove selected witness by swapping with last element and popping
        let last_idx = witnesses_left.len() - 1;
        witnesses_left.swap(witness_index, last_idx);
        witnesses_left.pop();
        
        byte_offset = (byte_offset + 4) % complete_hash.len();
    }

    selected_witnesses
}

pub fn get_identifier_from_claim_info(claim_info: &ClaimInfo) -> Result<String, ProofNotVerifiedError> {
    let str = format!(
        "{}\n{}\n{}",
        claim_info.provider,
        claim_info.parameters,
        claim_info.context
    );
    
    let hash = keccak256(str.as_bytes());
    Ok(format!("0x{}", hex::encode(hash).to_lowercase()))
}

pub fn create_sign_data_for_claim(
   data: &ProviderClaimData 
) -> String {
    format!(
        "{}\n{}\n{}\n{}",
        data.identifier,
        data.owner,
        data.timestamp_s,
        data.epoch
    )
}