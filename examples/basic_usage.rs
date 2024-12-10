use reclaim_rust_sdk::verify_proof;
use serde_json::json;

#[tokio::main]
async fn main() {
    // Test case 1: Valid proof
    test_valid_proof().await;
    
    // Test case 2: Invalid proof (wrong signature)
    test_invalid_proof().await;
}

async fn test_valid_proof() {
    println!("\n=== Testing Valid Proof ===");

    let claim_data = json!({
        "provider": "http",
        "parameters": "{\"additionalClientOptions\":{},\"body\":\"{\\\"includeGroups\\\":false,\\\"includeLogins\\\":false,\\\"includeVerificationStatus\\\":true}\",\"geoLocation\":\"\",\"headers\":{\"accept\":\"application/json\",\"user-agent\":\"Mozilla/5.0 (Linux; Android 13; M2101K6P Build/TKQ1.221013.002) AppleWebKit/537.36 (KHTML, like Gecko) Version/4.0 Chrome/129.0.6668.71 Mobile Safari/537.36\"},\"method\":\"POST\",\"paramValues\":{\"email\":\"providers@creatoros.co\",\"userName\":\"providerreclaim\"},\"responseMatches\":[{\"invert\":false,\"type\":\"contains\",\"value\":\"\\\"email\\\":\\\"{{email}}\\\"\"},{\"invert\":false,\"type\":\"contains\",\"value\":\"\\\"userName\\\":\\\"{{userName}}\\\"\"}],\"responseRedactions\":[{\"jsonPath\":\"$.email\",\"regex\":\"\\\"email\\\":\\\"(.*)\\\"\",\"xPath\":\"\"},{\"jsonPath\":\"$.userName\",\"regex\":\"\\\"userName\\\":\\\"(.*)\\\"\",\"xPath\":\"\"}],\"url\":\"https://www.kaggle.com/api/i/users.UsersService/GetCurrentUser\"}",
        "owner": "0xf70bc71a0cbfe10db345b32774212e16088cd6e3",
        "timestampS": 1728288586,
        "context": "{\"contextAddress\":\"0x00000000000\",\"contextMessage\":\"Example context message\",\"extractedParameters\":{\"email\":\"providers@creatoros.co\",\"userName\":\"providerreclaim\"},\"providerHash\":\"0x59f7ca945580a9570e9988a11f5683e9ba95a9445141b34200de4a96f648544a\"}",
        "identifier": "0xc6956f125ddd0ead0aa685dd32220fd5fbf1b3b66de601027d0674a0707da53e",
        "epoch": 1
      });   
    
    
    let proof_json = json!({
        "identifier": "0xc6956f125ddd0ead0aa685dd32220fd5fbf1b3b66de601027d0674a0707da53e",
        "claimData": claim_data,
        "signatures": [
          "0x47fec0522a1a1692941a10c3e1dbb13a05248d7bee5104f1bc87446c22bfc7a26f19cd6923b7aff32654517b269045e5451d8f5a4f56df91b34f151543e9a9bb1c"
        ],
        "witnesses": [
          {
            "id": "0x244897572368eadf65bfbc5aec98d8e5443a9072",
            "url": "wss://witness.reclaimprotocol.org/ws"
          }
        ],
        "publicData": {

        }
      });

    run_test(proof_json, "Valid Proof Test").await;
}

async fn test_invalid_proof() {
    println!("\n=== Testing Invalid Proof ===");
    let claim_data = json!({
        "provider": "http",
        "parameters": "{\"additionalClientOptions\":{},\"body\":\"{\\\"includeGroups\\\":false,\\\"includeLogins\\\":false,\\\"includeVerificationStatus\\\":true}\",\"geoLocation\":\"\",\"headers\":{\"accept\":\"application/json\",\"user-agent\":\"Mozilla/5.0 (Linux; Android 13; M2101K6P Build/TKQ1.221013.002) AppleWebKit/537.36 (KHTML, like Gecko) Version/4.0 Chrome/129.0.6668.71 Mobile Safari/537.36\"},\"method\":\"POST\",\"paramValues\":{\"email\":\"providers@creatoros.co\",\"userName\":\"providerreclaim\"},\"responseMatches\":[{\"invert\":false,\"type\":\"contains\",\"value\":\"\\\"email\\\":\\\"{{email}}\\\"\"},{\"invert\":false,\"type\":\"contains\",\"value\":\"\\\"userName\\\":\\\"{{userName}}\\\"\"}],\"responseRedactions\":[{\"jsonPath\":\"$.email\",\"regex\":\"\\\"email\\\":\\\"(.*)\\\"\",\"xPath\":\"\"},{\"jsonPath\":\"$.userName\",\"regex\":\"\\\"userName\\\":\\\"(.*)\\\"\",\"xPath\":\"\"}],\"url\":\"https://www.kaggle.com/api/i/users.UsersService/GetCurrentUser\"}",
        "owner": "0xf70bc71a0cbfe10db345b32774212e16088cd6e3",
        "timestampS": 1728288586,
        "context": "{\"contextAddress\":\"0x00000000000\",\"contextMessage\":\"Example context message\",\"extractedParameters\":{\"email\":\"providers@creatoros.co\",\"userName\":\"providerreclaim\"},\"providerHash\":\"0x59f7ca945580a9570e9988a11f5683e9ba95a9445141b34200de4a96f648544a\"}",
        "identifier": "0xc6956f125ddd0ead0aa685dd32220fd5fbf1b3b66de601027d0674a0707da33e",
        "epoch": 1
      });   
    
    
    let proof_json = json!({
        "identifier": "0xc6956f125ddd0ead0aa685dd32220fd5fbf1b3b66de601027d0674a0707da33e",
        "claimData": claim_data,
        "signatures": [
          "0x47fec0522a1a1692941a10c3e1dbb13a05248d7bee5104f1bc87446c22bfc7a26f19cd6923b6aff32654517b269045e5451d8f5a4f56df91b34f151543e9a9bb1c"
        ],
        "witnesses": [
          {
            "id": "0x244897572368eadf65bfbc5aec98d8e5443a9072",
            "url": "wss://witness.reclaimprotocol.org/ws"
          }
        ],
        "publicData": {

        }
      });

    run_test(proof_json, "Invalid Proof Test").await;
}

async fn run_test(proof_json: serde_json::Value, test_name: &str) {
    let proof: reclaim_rust_sdk::Proof = match serde_json::from_value(proof_json) {
        Ok(p) => p,
        Err(e) => {
            println!("❌ {} - Failed to parse proof: {}", test_name, e);
            return;
        }
    };

    println!("Testing proof for identifier: {}", proof.identifier);

    match verify_proof(&proof).await {
        Ok(is_valid) => {
            println!("✅ {} - Verification result: {}", test_name, if is_valid { "Valid" } else { "Invalid" });
        }
        Err(e) => {
            println!("❌ {} - Error verifying proof: {:?}", test_name, e);
        }
    }
} 