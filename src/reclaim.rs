use super::utils::proof_utils::{assert_valid_signed_claim, get_witnesses_for_claim};
use super::utils::witness::get_identifier_from_claim_info;
use crate::utils::errors::ProofNotVerifiedError;
use crate::utils::helper::replace_all;
use crate::utils::interface::Proof;
use crate::utils::types::{ClaimInfo, SignedClaim};

pub async fn verify_proof(proof: &Proof) -> Result<bool, ProofNotVerifiedError> {
    if proof.signatures.is_empty() {
        return Err(ProofNotVerifiedError("No signatures".to_string()));
    }

    let mut witnesses = Vec::new();

    if !proof.witnesses.is_empty() && proof.witnesses[0].url == "manual-verify" {
        witnesses.push(proof.witnesses[0].id.clone());
    } else {
        witnesses = get_witnesses_for_claim(
            proof.claim_data.epoch,
            &proof.identifier,
            proof.claim_data.timestamp_s,
        )
        .await?;
    }

    let claim_info = ClaimInfo {
        parameters: proof.claim_data.parameters.clone(),
        provider: proof.claim_data.provider.clone(),
        context: proof.claim_data.context.clone(),
    };

    let calculated_identifier = get_identifier_from_claim_info(&claim_info)?;

    let clean_identifier = replace_all(&proof.identifier, "\"", "");
    if calculated_identifier != clean_identifier {
        return Err(ProofNotVerifiedError("Identifier Mismatch".to_string()));
    }

    let signatures: Result<Vec<Vec<u8>>, _> = proof
        .signatures
        .iter()
        .map(|sig| {
            let clean_sig = sig.trim_start_matches("0x");
            hex::decode(clean_sig)
        })
        .collect();

    let signatures = signatures.map_err(|e| {
        ProofNotVerifiedError(format!("Failed to decode signature hex: {}", e))
    })?;

    let signed_claim = SignedClaim {
        claim_data: proof.claim_data.clone(),
        signatures,
    };

    match assert_valid_signed_claim(&signed_claim, &witnesses) {
        Ok(_) => Ok(true),
        Err(e) => {
            log::info!("Error verifying proof: {}", e);
            Ok(false)
        }
    }
}
