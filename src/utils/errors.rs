use thiserror::Error;
use url::ParseError as UrlParseError;
use serde_json::Error as JsonError;
use hex::FromHexError;
use rustc_hex::FromHexError as RustcFromHexError;

#[derive(Error, Debug)]
#[error("Operation timed out: {0}")]
pub struct TimeoutError(pub String);

#[derive(Error, Debug)]
#[error("Proof verification failed: {0}")]
pub struct ProofNotVerifiedError(pub String);

#[derive(Error, Debug)]
#[error("Session not started: {0}")]
pub struct SessionNotStartedError(pub String);

#[derive(Error, Debug)]
#[error("Provider not found: {0}")]
pub struct ProviderNotFoundError(pub String);

#[derive(Error, Debug)]
#[error("Failed to build proof request: {0}")]
pub struct BuildProofRequestError(pub String);

#[derive(Error, Debug)]
#[error("Failed to generate signature: {0}")]
pub struct SignatureGeneratingError(pub String);

#[derive(Error, Debug)]
#[error("Signature not found: {0}")]
pub struct SignatureNotFoundError(pub String);

#[derive(Error, Debug)]
#[error("Invalid signature: {0}")]
pub struct InvalidSignatureError(pub String);

#[derive(Error, Debug)]
#[error("Failed to update session: {0}")]
pub struct UpdateSessionError(pub String);

#[derive(Error, Debug)]
#[error("Failed to initialize session: {0}")]
pub struct InitSessionError(pub String);

#[derive(Error, Debug)]
#[error("Provider failed: {0}")]
pub struct ProviderFailedError(pub String);

#[derive(Error, Debug)]
#[error("Invalid parameter: {0}")]
pub struct InvalidParamError(pub String);

#[derive(Error, Debug)]
#[error("Application error: {0}")]
pub struct ApplicationError(pub String);

#[derive(Error, Debug)]
#[error("Initialization error: {0}")]
pub struct InitError(pub String);

#[derive(Error, Debug)]
#[error("Available parameters error: {0}")]
pub struct AvailableParamsError(pub String);

#[derive(Error, Debug)]
#[error("Backend server error: {0}")]
pub struct BackendServerError(pub String);

#[derive(Error, Debug)]
#[error("Failed to get status URL: {0}")]
pub struct GetStatusUrlError(pub String);

#[derive(Error, Debug)]
#[error("No provider parameters: {0}")]
pub struct NoProviderParamsError(pub String);

#[derive(Error, Debug)]
#[error("Failed to set parameters: {0}")]
pub struct SetParamsError(pub String);

#[derive(Error, Debug)]
#[error("Failed to add context: {0}")]
pub struct AddContextError(pub String);

#[derive(Error, Debug)]
#[error("Failed to set signature: {0}")]
pub struct SetSignatureError(pub String);

#[derive(Error, Debug)]
#[error("Failed to get app callback URL: {0}")]
pub struct GetAppCallbackUrlError(pub String);

#[derive(Error, Debug)]
#[error("Failed to get request URL: {0}")]
pub struct GetRequestUrlError(pub String);

#[derive(Error, Debug)]
#[error("Status URL error: {0}")]
pub struct StatusUrlError(pub String);

#[derive(Error, Debug)]
#[error("Proof submission failed: {0}")]
pub struct ProofSubmissionFailedError(pub String); 

// beacon not found
#[derive(Error, Debug)]
#[error("Beacon not found: {0}")]
pub struct BeaconNotFoundError(pub String);


// get Becaon State Error
#[derive(Error, Debug)]
#[error("Failed to get Beacon State: {0}")]
pub struct GetBeaconStateError(pub String);


// close Beacon Error
#[derive(Error, Debug)]
#[error("Failed to close Beacon: {0}")]
pub struct CloseBeaconError(pub String);

// Unsupported Chain Error
#[derive(Error, Debug)]
#[error("Unsupported Chain: {0}")]
pub struct UnsupportedChainError(pub String);

impl From<UrlParseError> for ProofNotVerifiedError {
    fn from(err: UrlParseError) -> Self {
        ProofNotVerifiedError(err.to_string())
    }
}

impl From<JsonError> for ProofNotVerifiedError {
    fn from(err: JsonError) -> Self {
        ProofNotVerifiedError(err.to_string())
    }
}

impl From<FromHexError> for ProofNotVerifiedError {
    fn from(err: FromHexError) -> Self {
        ProofNotVerifiedError(err.to_string())
    }
}

impl From<RustcFromHexError> for ProofNotVerifiedError {
    fn from(err: RustcFromHexError) -> Self {
        ProofNotVerifiedError(err.to_string())
    }
}