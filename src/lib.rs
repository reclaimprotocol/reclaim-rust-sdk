//! Reclaim Protocol SDK for Rust
//! 
//! This crate provides functionality to verify Reclaim Protocol proofs.

mod reclaim;
mod utils;
mod contract_data;
pub use reclaim::verify_proof;
pub use utils::types::{ClaimInfo, SignedClaim};
pub use utils::interface::Proof;
pub use utils::errors::ProofNotVerifiedError;