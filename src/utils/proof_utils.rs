use super::errors::ProofNotVerifiedError;
use super::interface::{BeaconImpl, BeaconState};
use super::types::SignedClaim;
use super::witness::{fetch_witness_list_for_claim, create_sign_data_for_claim};
use anyhow::Result;
use std::collections::HashSet;

use super::smart_contract::{fetch_epoch_data, make_beacon};
use ethers::core::types::Signature;
use ethers::utils::hash_message;

pub async fn get_beacon_state(beacon: &BeaconImpl, epoch: u64) -> Result<BeaconState, ProofNotVerifiedError> {
    // check if epoch is same as current epoch and arg epoch is not null or throw error
    if beacon.epoch_data.epoch != epoch {
        return Err(ProofNotVerifiedError("Beacon does not have state for the given epoch".to_string()));
    }

    // make a contract call to fetch epoch data
    let state = fetch_epoch_data(beacon.contract.clone(), epoch).await?;
    Ok(state)
}

pub async fn get_witnesses_for_claim(
    epoch: u64,
    identifier: &str,
    timestamp_s: u64,
) -> Result<Vec<String>, ProofNotVerifiedError> {  
    let beacon = make_beacon(None).await?;
    // check if beacon has state
    if beacon.epoch_data.epoch != epoch {
        return Err(ProofNotVerifiedError("Beacon does not have state for the given epoch".to_string()));
    }

    let state = get_beacon_state(&beacon, epoch).await?;
    let witness_list = fetch_witness_list_for_claim(&state, identifier, timestamp_s);
    let witnesses: Vec<String> = witness_list
        .iter()
        .map(|w| w.id.to_lowercase())
        .collect();
    Ok(witnesses)
}

pub fn recover_signers_of_signed_claim(signed_claim: &SignedClaim) -> Result<Vec<String>, ProofNotVerifiedError> {
    let data_str = create_sign_data_for_claim(&signed_claim.claim_data);
    let message = hash_message(data_str);
    
    let mut credentials = Vec::new();
    for signature_bytes in &signed_claim.signatures {
        let signature = Signature::try_from(signature_bytes.as_slice())
            .map_err(|e| ProofNotVerifiedError(format!("Invalid signature format: {}", e)))?;
            
        let signer = signature
            .recover(message)
            .map_err(|e| ProofNotVerifiedError(format!("Failed to recover signer: {}", e)))?;
            
        credentials.push(format!("{:?}", signer).to_lowercase());
    }
    
    Ok(credentials)
}

pub fn assert_valid_signed_claim(
    signed_claim: &SignedClaim,
    expected_witness_addresses: &[String],
) -> Result<(), ProofNotVerifiedError> {
    let witness_addresses = recover_signers_of_signed_claim(signed_claim)
        .map_err(|e| ProofNotVerifiedError(format!("Failed to recover signers: {}", e)))?;
    
    let mut witnesses_not_seen: HashSet<String> = expected_witness_addresses.iter().cloned().collect();
    
    for witness in witness_addresses {
        witnesses_not_seen.remove(&witness);
    }

    if !witnesses_not_seen.is_empty() {
        let missing_witnesses = witnesses_not_seen.into_iter().collect::<Vec<_>>().join(", ");
        return Err(ProofNotVerifiedError(format!(
            "Claim validation failed. Missing signatures from: {}",
            missing_witnesses
        )));
    }

    Ok(())
} 