//! Response validation example
//! 
//! Run with: cargo run --example validate_response

use base64::{engine::general_purpose, Engine};
use rustpayment::{generate_signature, validate_esewa_response, EsewaPaymentResponse};

fn main() {
    let secret_key = "8gBm/:&EnhH.1/q";

    println!("eSewa Response Validation Demo\n");
    println!("================================\n");

    // Simulate a valid response from eSewa
    let response = EsewaPaymentResponse {
        transaction_code: "000D13A".to_string(),
        status: "COMPLETE".to_string(),
        total_amount: "110.0".to_string(),
        transaction_uuid: "id-1234567890-abcdef".to_string(),
        product_code: "EPAYTEST".to_string(),
        signed_field_names:
            "transaction_code,status,total_amount,transaction_uuid,product_code,signed_field_names"
                .to_string(),
        signature: generate_signature(
            "110.0",
            "id-1234567890-abcdef",
            "EPAYTEST",
            secret_key,
        ),
    };

    // Encode to base64 (this is what eSewa sends)
    let json = serde_json::to_string(&response).unwrap();
    let encoded = general_purpose::STANDARD.encode(json.as_bytes());

    println!("Encoded response data:");
    println!("{}\n", encoded);

    // Validate the response
    match validate_esewa_response(&encoded, secret_key) {
        Ok(result) => {
            println!(" Validation successful!\n");
            println!("Signature Valid: {}", result.signature_valid);
            println!("Transaction Code: {}", result.response.transaction_code);
            println!("Status: {}", result.response.status);
            println!("Amount: {}", result.response.total_amount);
            println!("UUID: {}", result.response.transaction_uuid);

            if result.signature_valid {
                println!("\n Payment verified - you can process this order!");
            } else {
                println!("\n  Warning: Invalid signature - do not process!");
            }
        }
        Err(e) => {
            eprintln!(" Validation error: {}", e);
        }
    }

    println!("\n\n--- Testing Invalid Signature ---\n");

    // Test with invalid signature
    let mut invalid_response = response.clone();
    invalid_response.signature = "INVALID_SIGNATURE".to_string();

    let json = serde_json::to_string(&invalid_response).unwrap();
    let encoded = general_purpose::STANDARD.encode(json.as_bytes());

    match validate_esewa_response(&encoded, secret_key) {
        Ok(result) => {
            if result.signature_valid {
                println!("  Signature incorrectly validated as valid!");
            } else {
                println!(" Invalid signature correctly detected!");
                println!("   Do NOT process this payment.");
            }
        }
        Err(e) => {
            eprintln!(" Validation error: {}", e);
        }
    }
}
