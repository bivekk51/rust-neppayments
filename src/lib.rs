//! # RustPayment - eSewa Payment Integration Library
//!
//! A Rust library for integrating eSewa payment gateway into your applications.
//! Provides functions for initiating payments, generating signatures, and validating responses.
//!
//! ## Features
//! - Generate HMAC-SHA256 signatures for eSewa payments
//! - Initiate payment requests and get redirect URLs
//! - Validate and decode eSewa payment responses
//!
//! ## Example
//! ```rust,no_run
//! # use rustpayment::{pay_with_esewa, EsewaPaymentRequest, EsewaEnvironment};
//! # async fn example() {
//! let request = EsewaPaymentRequest {
//!     amount: "100".to_string(),
//!     tax_amount: "10".to_string(),
//!     total_amount: "110".to_string(),
//!     transaction_uuid: "id-123-abc".to_string(),
//!     product_code: "EPAYTEST".to_string(),
//!     product_service_charge: "0".to_string(),
//!     product_delivery_charge: "0".to_string(),
//!     success_url: "http://example.com/success".to_string(),
//!     failure_url: "http://example.com/failure".to_string(),
//!     signed_field_names: "total_amount,transaction_uuid,product_code".to_string(),
//! };
//! 
//! let secret_key = "8gBm/:&EnhH.1/q";
//! 
//! match pay_with_esewa(request, secret_key, EsewaEnvironment::Sandbox).await {
//!     Ok(url) => println!("Redirect to: {}", url),
//!     Err(e) => eprintln!("Payment error: {}", e),
//! }
//! # }
//! ```

use base64::{engine::general_purpose, Engine};
use hmac::{Hmac, Mac};
use reqwest::Client;
use serde::{Deserialize, Serialize};
use sha2::Sha256;

/// Represents the payment request data required by eSewa
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EsewaPaymentRequest {
    pub amount: String,
    pub tax_amount: String,
    pub total_amount: String,
    pub transaction_uuid: String,
    pub product_code: String,
    pub product_service_charge: String,
    pub product_delivery_charge: String,
    pub success_url: String,
    pub failure_url: String,
    pub signed_field_names: String,
}

/// Represents the decoded response from eSewa after payment
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EsewaPaymentResponse {
    pub transaction_code: String,
    pub status: String,
    pub total_amount: String,
    pub transaction_uuid: String,
    pub product_code: String,
    pub signed_field_names: String,
    pub signature: String,
}

/// Represents the validation result including the decoded data and signature validity
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ValidationResult {
    pub signature_valid: bool,
    pub response: EsewaPaymentResponse,
}

/// Error types for the payment library
#[derive(Debug)]
pub enum PaymentError {
    NetworkError(String),
    InvalidResponse(String),
    SignatureError(String),
    DecodeError(String),
}

/// Which eSewa environment to use for requests
#[derive(Debug, Clone, Copy)]
pub enum EsewaEnvironment {
    /// Use the eSewa RC / sandbox endpoint (testing)
    Sandbox,
    /// Use the production eSewa endpoint (real-world)
    Production,
}

impl std::fmt::Display for PaymentError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            PaymentError::NetworkError(msg) => write!(f, "Network error: {}", msg),
            PaymentError::InvalidResponse(msg) => write!(f, "Invalid response: {}", msg),
            PaymentError::SignatureError(msg) => write!(f, "Signature error: {}", msg),
            PaymentError::DecodeError(msg) => write!(f, "Decode error: {}", msg),
        }
    }
}

impl std::error::Error for PaymentError {}

/// Generates an HMAC-SHA256 signature for eSewa payment
///
/// # Arguments
/// * `total_amount` - Total payment amount
/// * `transaction_uuid` - Unique transaction identifier
/// * `product_code` - eSewa product code (e.g., "EPAYTEST")
/// * `secret_key` - Secret key provided by eSewa
///
/// # Returns
/// Base64-encoded HMAC-SHA256 signature
///
/// # Example
/// ```
/// use rustpayment::generate_signature;
///
/// let signature = generate_signature("110", "id-123-abc", "EPAYTEST", "8gBm/:&EnhH.1/q");
/// println!("Signature: {}", signature);
/// ```
pub fn generate_signature(
    total_amount: &str,
    transaction_uuid: &str,
    product_code: &str,
    secret_key: &str,
) -> String {
    let data = format!(
        "total_amount={},transaction_uuid={},product_code={}",
        total_amount, transaction_uuid, product_code
    );

    let mut mac = Hmac::<Sha256>::new_from_slice(secret_key.as_bytes())
        .expect("HMAC can take key of any size");
    mac.update(data.as_bytes());
    let result = mac.finalize();
    let code_bytes = result.into_bytes();
    general_purpose::STANDARD.encode(code_bytes)
}

/// Initiates a payment with eSewa and returns the redirect URL
///
/// # Arguments
/// * `request` - Payment request details
/// * `secret_key` - Secret key provided by eSewa
///
/// # Returns
/// Result containing the payment URL to redirect the user to, or a PaymentError
///
/// ## Example
/// ```rust,no_run
/// # use rustpayment::{pay_with_esewa, EsewaPaymentRequest, EsewaEnvironment};
/// # async fn example() {
/// let request = EsewaPaymentRequest {
///     amount: "100".to_string(),
///     tax_amount: "10".to_string(),
///     total_amount: "110".to_string(),
///     transaction_uuid: "id-123-abc".to_string(),
///     product_code: "EPAYTEST".to_string(),
///     product_service_charge: "0".to_string(),
///     product_delivery_charge: "0".to_string(),
///     success_url: "http://example.com/success".to_string(),
///     failure_url: "http://example.com/failure".to_string(),
///     signed_field_names: "total_amount,transaction_uuid,product_code".to_string(),
/// };
/// 
/// match pay_with_esewa(request, "8gBm/:&EnhH.1/q", EsewaEnvironment::Sandbox).await {
///     Ok(url) => println!("Redirect to: {}", url),
///     Err(e) => eprintln!("Error: {}", e),
/// }
/// # }
/// ```
pub async fn pay_with_esewa(
    request: EsewaPaymentRequest,
    secret_key: &str,
    env: EsewaEnvironment,
) -> Result<String, PaymentError> {
    // Generate signature
    let signature = generate_signature(
        &request.total_amount,
        &request.transaction_uuid,
        &request.product_code,
        secret_key,
    );

    // Build form parameters
    let params = [
        ("amount", request.amount.as_str()),
        ("failure_url", request.failure_url.as_str()),
        ("product_delivery_charge", request.product_delivery_charge.as_str()),
        ("product_service_charge", request.product_service_charge.as_str()),
        ("product_code", request.product_code.as_str()),
        ("signature", &signature),
        ("signed_field_names", request.signed_field_names.as_str()),
        ("success_url", request.success_url.as_str()),
        ("tax_amount", request.tax_amount.as_str()),
        ("total_amount", request.total_amount.as_str()),
        ("transaction_uuid", request.transaction_uuid.as_str()),
    ];

    // Send POST request
    let client = Client::new();
    // Choose endpoint based on environment
    let url = match env {
        EsewaEnvironment::Sandbox => "https://rc-epay.esewa.com.np/api/epay/main/v2/form",
        EsewaEnvironment::Production => "https://epay.esewa.com.np/api/epay/main/v2/form",
    };

    let response = client
        .post(url)
        .form(&params)
        .send()
        .await
        .map_err(|e| PaymentError::NetworkError(e.to_string()))?;

    let status = response.status();
    let final_url = response.url().to_string();

    if status.as_u16() == 200 {
        Ok(final_url)
    } else {
        Err(PaymentError::InvalidResponse(format!(
            "Expected status 200, got {}",
            status
        )))
    }
}

/// Validates and decodes eSewa payment response
///
/// # Arguments
/// * `encoded_data` - Base64-encoded JSON data from eSewa callback
/// * `secret_key` - Secret key provided by eSewa
///
/// # Returns
/// Result containing ValidationResult with decoded data and signature validity status
///
/// # Example
/// ```
/// use rustpayment::validate_esewa_response;
///
/// let encoded = "eyJ0cmFuc2FjdGlvbl9jb2RlIjoiMDAwRDEzQSIsInN0YXR1cyI6IkNPTVBMRVRFIn0=";
/// let secret_key = "8gBm/:&EnhH.1/q";
///
/// match validate_esewa_response(encoded, secret_key) {
///     Ok(result) => {
///         println!("Valid: {}", result.signature_valid);
///         println!("Status: {}", result.response.status);
///     }
///     Err(e) => eprintln!("Validation error: {}", e),
/// }
/// ```
pub fn validate_esewa_response(
    encoded_data: &str,
    secret_key: &str,
) -> Result<ValidationResult, PaymentError> {
    // Decode base64
    let decoded_bytes = general_purpose::STANDARD
        .decode(encoded_data)
        .map_err(|e| PaymentError::DecodeError(format!("Base64 decode failed: {}", e)))?;

    let decoded_str = String::from_utf8(decoded_bytes)
        .map_err(|e| PaymentError::DecodeError(format!("UTF-8 decode failed: {}", e)))?;

    // Parse JSON
    let response: EsewaPaymentResponse = serde_json::from_str(&decoded_str)
        .map_err(|e| PaymentError::DecodeError(format!("JSON parse failed: {}", e)))?;

    // Verify signature
    let computed_signature = generate_signature(
        &response.total_amount,
        &response.transaction_uuid,
        &response.product_code,
        secret_key,
    );

    let signature_valid = computed_signature == response.signature;

    Ok(ValidationResult {
        signature_valid,
        response,
    })
}

/// Generates a transaction UUID in the format: `id-<milliseconds>-<random>`
///
/// # Returns
/// A unique transaction identifier string
///
/// # Example
/// ```
/// use rustpayment::generate_transaction_uuid;
///
/// let uuid = generate_transaction_uuid();
/// println!("Transaction UUID: {}", uuid);
/// ```
pub fn generate_transaction_uuid() -> String {
    use rand::Rng;
    use std::time::{SystemTime, UNIX_EPOCH};

    let now_ms = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("time went backwards")
        .as_millis();

    const CHARS: &[u8] = b"0123456789abcdefghijklmnopqrstuvwxyz";
    let mut rng = rand::rng();
    let rand_part: String = (0..9)
        .map(|_| {
            let idx = rng.random_range(0..CHARS.len());
            CHARS[idx] as char
        })
        .collect();

    format!("id-{}-{}", now_ms, rand_part)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generate_signature() {
        let signature = generate_signature("110", "id-123-abc", "EPAYTEST", "8gBm/:&EnhH.1/q");
        assert!(!signature.is_empty());
        assert!(signature.len() > 20); // HMAC-SHA256 base64 is ~44 chars
    }

    #[test]
    fn test_generate_signature_consistency() {
        let sig1 = generate_signature("110", "id-123", "EPAYTEST", "8gBm/:&EnhH.1/q");
        let sig2 = generate_signature("110", "id-123", "EPAYTEST", "8gBm/:&EnhH.1/q");
        assert_eq!(sig1, sig2, "Same input should produce same signature");
    }

    #[test]
    fn test_generate_transaction_uuid() {
        let uuid1 = generate_transaction_uuid();
        let uuid2 = generate_transaction_uuid();
        
        assert!(uuid1.starts_with("id-"));
        assert!(uuid2.starts_with("id-"));
        assert_ne!(uuid1, uuid2, "UUIDs should be unique");
    }

    #[test]
    fn test_validate_esewa_response_valid() {
        // Create a test response
        let test_data = EsewaPaymentResponse {
            transaction_code: "000D13A".to_string(),
            status: "COMPLETE".to_string(),
            total_amount: "110.0".to_string(),
            transaction_uuid: "id-123-abc".to_string(),
            product_code: "EPAYTEST".to_string(),
            signed_field_names: "transaction_code,status,total_amount,transaction_uuid,product_code,signed_field_names".to_string(),
            signature: generate_signature("110.0", "id-123-abc", "EPAYTEST", "8gBm/:&EnhH.1/q"),
        };

        let json_str = serde_json::to_string(&test_data).unwrap();
        let encoded = general_purpose::STANDARD.encode(json_str.as_bytes());

        let result = validate_esewa_response(&encoded, "8gBm/:&EnhH.1/q").unwrap();
        
        assert!(result.signature_valid);
        assert_eq!(result.response.status, "COMPLETE");
        assert_eq!(result.response.transaction_code, "000D13A");
    }

    #[test]
    fn test_validate_esewa_response_invalid_signature() {
        let test_data = EsewaPaymentResponse {
            transaction_code: "000D13A".to_string(),
            status: "COMPLETE".to_string(),
            total_amount: "110.0".to_string(),
            transaction_uuid: "id-123-abc".to_string(),
            product_code: "EPAYTEST".to_string(),
            signed_field_names: "transaction_code,status,total_amount,transaction_uuid,product_code,signed_field_names".to_string(),
            signature: "invalid_signature".to_string(),
        };

        let json_str = serde_json::to_string(&test_data).unwrap();
        let encoded = general_purpose::STANDARD.encode(json_str.as_bytes());

        let result = validate_esewa_response(&encoded, "8gBm/:&EnhH.1/q").unwrap();
        
        assert!(!result.signature_valid);
    }

    #[test]
    fn test_validate_esewa_response_invalid_base64() {
        let result = validate_esewa_response("not-valid-base64!!!", "8gBm/:&EnhH.1/q");
        assert!(result.is_err());
    }

    #[test]
    fn test_esewa_payment_request_serialization() {
        let request = EsewaPaymentRequest {
            amount: "100".to_string(),
            tax_amount: "10".to_string(),
            total_amount: "110".to_string(),
            transaction_uuid: "id-123".to_string(),
            product_code: "EPAYTEST".to_string(),
            product_service_charge: "0".to_string(),
            product_delivery_charge: "0".to_string(),
            success_url: "http://test.com/success".to_string(),
            failure_url: "http://test.com/failure".to_string(),
            signed_field_names: "total_amount,transaction_uuid,product_code".to_string(),
        };

        let json = serde_json::to_string(&request).unwrap();
        let deserialized: EsewaPaymentRequest = serde_json::from_str(&json).unwrap();
        
        assert_eq!(request.amount, deserialized.amount);
        assert_eq!(request.transaction_uuid, deserialized.transaction_uuid);
    }
}
