# Reclaim Protocol Rust SDK Integration Guide

This guide will walk you through integrating the Reclaim Protocol Rust SDK into your application. We'll demonstrate how to use the SDK to verify proofs in your Rust applications.

## Prerequisites

Before we begin, make sure you have:

1. An application ID from Reclaim Protocol.
2. An application secret from Reclaim Protocol.
3. A provider ID for the specific service you want to verify.

You can obtain these details from the [Reclaim Developer Portal](https://dev.reclaimprotocol.org/).

## Step 1: Add the dependency

Add the Reclaim Protocol SDK to your `Cargo.toml` file:

```toml
[dependencies]
reclaim-sdk = "0.1.0"
tokio = { version = "1.0", features = ["full"] }
```

## Step 2: Basic Usage

Here's a simple example demonstrating how to verify proofs using the Reclaim SDK:

```rust
use reclaim_sdk::{ReclaimClient, ProofRequest, VerificationResult};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize the Reclaim client
    let proof_json = r#"
   {
       <!-- your proof data object -->
   }"#;
   <!-- proof should be a Proof struct data -->
    let proof: reclaim_rust_sdk::Proof = serde_json::from_value(proof_json)?;
    // Verify the proof
    match client.verify_proof(proof).await {
        Ok(result) => {
            println!("Proof verification successful!");
            println!("Extracted data: {:?}", result.extracted_data);
            println!("Context: {:?}", result.context);
        },
        Err(e) => {
            println!("Proof verification failed: {}", e);
        }
    }

    Ok(())
}
```

## Best Practices

1. **Error Handling**: Always implement proper error handling for verification failures.
2. **Async Operations**: Use async/await for better performance in production applications.
3. **Security**: Keep your Application Secret secure and never expose it in public repositories.

## Next Steps

Explore the [Reclaim Protocol documentation](https://docs.reclaimprotocol.org/) for more advanced features and best practices for integrating the SDK into your production applications.

## Contributing to Our Project

We welcome contributions to our project! If you find any issues or have suggestions for improvements, please open an issue or submit a pull request.

## Security Note

Always keep your Application Secret secure. Never expose it in client-side code or public repositories.

## Code of Conduct

Please read and follow our [Code of Conduct](https://github.com/reclaimprotocol/.github/blob/main/Code-of-Conduct.md) to ensure a positive and inclusive environment for all contributors.

## Security

If you discover any security-related issues, please refer to our [Security Policy](https://github.com/reclaimprotocol/.github/blob/main/SECURITY.md) for information on how to responsibly disclose vulnerabilities.

## Contributor License Agreement

Before contributing to this project, please read and sign our [Contributor License Agreement (CLA)](https://github.com/reclaimprotocol/.github/blob/main/CLA.md).

## Indie Hackers

For Indie Hackers: [Check out our guidelines and potential grant opportunities](https://github.com/reclaimprotocol/.github/blob/main/Indie-Hackers.md)

## License

This project is licensed under a [custom license](https://github.com/reclaimprotocol/.github/blob/main/LICENSE). By contributing to this project, you agree that your contributions will be licensed under its terms.

Thank you for your contributions!
