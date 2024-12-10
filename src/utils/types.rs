use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use super::interface::{ProviderClaimData, ProviderData};

#[derive(Debug, Serialize, Deserialize)]
pub struct SignedClaim {
    pub claim_data: ProviderClaimData,
    pub signatures: Vec<Vec<u8>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ClaimInfo {
    pub context: String,
    pub provider: String,
    pub parameters: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum AnyClaimInfo {
    ClaimInfo(ClaimInfo),
    Identifier { identifier: String },
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CompleteClaimData {
    pub owner: String,
    #[serde(rename = "timestampS")]
    pub timestamp_s: u64,
    pub epoch: u64,
    #[serde(flatten)]
    pub claim_info: AnyClaimInfo,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SignedClaimNew {
    pub claim: CompleteClaimData,
    pub signatures: Vec<Vec<u8>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateVerificationRequest {
    #[serde(rename = "providerIds")]
    pub provider_ids: Vec<String>,
    #[serde(rename = "applicationSecret", skip_serializing_if = "Option::is_none")]
    pub application_secret: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct InitSessionResponse {
    #[serde(rename = "sessionId")]
    pub session_id: String,
    pub provider: ProviderData,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateSessionResponse {
    pub success: bool,
    pub message: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum SessionStatus {
    SessionInit,
    SessionStarted,
    UserInitVerification,
    UserStartedVerification,
    ProofGenerationStarted,
    ProofGenerationSuccess,
    ProofGenerationFailed,
    ProofSubmitted,
    ProofSubmissionFailed,
    ProofManualVerificationSubmited,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ProofRequestOptions {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub log: Option<bool>,
    #[serde(rename = "acceptAiProviders", skip_serializing_if = "Option::is_none")]
    pub accept_ai_providers: Option<bool>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ProofPropertiesJson {
    #[serde(rename = "applicationId")]
    pub application_id: String,
    #[serde(rename = "providerId")]
    pub provider_id: String,
    #[serde(rename = "sessionId")]
    pub session_id: String,
    pub context: serde_json::Value,
    #[serde(rename = "requestedProof")]
    pub requested_proof: serde_json::Value,
    pub signature: String,
    #[serde(rename = "redirectUrl", skip_serializing_if = "Option::is_none")]
    pub redirect_url: Option<String>,
    #[serde(rename = "timeStamp")]
    pub timestamp: String,
    #[serde(rename = "appCallbackUrl", skip_serializing_if = "Option::is_none")]
    pub app_callback_url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub options: Option<ProofRequestOptions>,
    #[serde(rename = "sdkVersion")]
    pub sdk_version: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TemplateData {
    #[serde(rename = "sessionId")]
    pub session_id: String,
    #[serde(rename = "providerId")]
    pub provider_id: String,
    #[serde(rename = "applicationId")]
    pub application_id: String,
    pub signature: String,
    pub timestamp: String,
    #[serde(rename = "callbackUrl")]
    pub callback_url: String,
    pub context: String,
    pub parameters: HashMap<String, String>,
    #[serde(rename = "redirectUrl")]
    pub redirect_url: String,
    #[serde(rename = "acceptAiProviders")]
    pub accept_ai_providers: bool,
    #[serde(rename = "sdkVersion")]
    pub sdk_version: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StatusUrlResponse {
    pub message: String,
    pub session: Option<SessionInfo>,
    #[serde(rename = "providerId")]
    pub provider_id: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SessionInfo {
    pub id: String,
    #[serde(rename = "appId")]
    pub app_id: String,
    #[serde(rename = "httpProviderId")]
    pub http_provider_id: Vec<String>,
    #[serde(rename = "sessionId")]
    pub session_id: String,
    pub proofs: Option<Vec<serde_json::Value>>,
    #[serde(rename = "statusV2")]
    pub status_v2: String,
} 