use ethers::providers::{Http, Provider};
use ethers::contract::Contract;
use serde::{Deserialize, Serialize};

// Provider-related interfaces
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProviderData {
    pub http_provider_id: String,
    pub name: String,
    pub url: String,
    pub login_url: String,
    pub response_selections: Vec<ResponseSelection>,
    pub body_sniff: Option<BodySniff>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResponseSelection {
    pub invert: bool,
    pub response_match: String,
    pub x_path: Option<String>,
    pub json_path: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BodySniff {
    pub enabled: bool,
    pub regex: Option<String>,
    pub template: Option<String>,
}

// Proof-related interfaces
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Proof {
    pub identifier: String,
    #[serde(rename = "claimData")]
    pub claim_data: ProviderClaimData,
    pub signatures: Vec<String>,
    pub witnesses: Vec<WitnessData>,
    #[serde(rename = "publicData")]
    pub public_data: Option<std::collections::HashMap<String, String>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WitnessData {
    pub id: String,
    pub url: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProviderClaimData {
    pub provider: String,
    pub parameters: String,
    pub owner: String,
    #[serde(rename = "timestampS")]
    pub timestamp_s: u64,
    pub context: String,
    pub identifier: String,
    pub epoch: u64,
}

// Request-related interfaces
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RequestedProof {
    pub url: String,
    pub parameters: std::collections::HashMap<String, String>,
}

// Context and Beacon interfaces
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Context {
    #[serde(rename = "contextAddress")]
    pub context_address: String,
    #[serde(rename = "contextMessage")]
    pub context_message: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BeaconState {
    pub witnesses: Vec<WitnessData>,
    pub epoch: u64,
    #[serde(rename = "witnessesRequiredForClaim")]
    pub witnesses_required_for_claim: u64,
    #[serde(rename = "nextEpochTimestampS")]
    pub next_epoch_timestamp_s: u64,
}


#[derive(Debug, Clone)]
pub struct BeaconImpl {
    pub contract: Contract<Provider<Http>>,
    pub epoch_data: BeaconState,
}