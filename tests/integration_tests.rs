use rustpayment::{
    generate_signature, generate_transaction_uuid, validate_esewa_response, EsewaPaymentRequest,
    EsewaPaymentResponse,
};
use base64::{engine::general_purpose, Engine};

const TEST_SECRET_KEY: &str = "8gBm/:&EnhH.1/q";

#[test]
fn test_signature_generation() {
    let signature = generate_signature("110", "id-123-abc", "EPAYTEST", TEST_SECRET_KEY);
    
    // Signature should be consistent
    let signature2 = generate_signature("110", "id-123-abc", "EPAYTEST", TEST_SECRET_KEY);
    assert_eq!(signature, signature2);
    
    // Different inputs should produce different signatures
    let different = generate_signature("100", "id-123-abc", "EPAYTEST", TEST_SECRET_KEY);
    assert_ne!(signature, different);
}

#[test]
fn test_transaction_uuid_format() {
    let uuid = generate_transaction_uuid();
    
    // Should start with "id-"
    assert!(uuid.starts_with("id-"));
    
    // Should have three parts separated by dashes
    let parts: Vec<&str> = uuid.split('-').collect();
    assert_eq!(parts.len(), 3);
    assert_eq!(parts[0], "id");
    
    // Second part should be numeric (milliseconds)
    assert!(parts[1].parse::<u128>().is_ok());
    
    // Third part should be alphanumeric
    assert_eq!(parts[2].len(), 9);
    assert!(parts[2].chars().all(|c| c.is_alphanumeric()));
}

#[test]
fn test_transaction_uuid_uniqueness() {
    let mut uuids = std::collections::HashSet::new();
    
    // Generate 100 UUIDs and ensure they're all unique
    for _ in 0..100 {
        let uuid = generate_transaction_uuid();
        assert!(uuids.insert(uuid), "UUID collision detected");
    }
}

#[test]
fn test_validate_complete_payment() {
    // Create a test payment response
    let response = EsewaPaymentResponse {
        transaction_code: "TEST123".to_string(),
        status: "COMPLETE".to_string(),
        total_amount: "110.0".to_string(),
        transaction_uuid: "id-test-uuid".to_string(),
        product_code: "EPAYTEST".to_string(),
        signed_field_names: "transaction_code,status,total_amount,transaction_uuid,product_code,signed_field_names".to_string(),
        signature: generate_signature("110.0", "id-test-uuid", "EPAYTEST", TEST_SECRET_KEY),
    };
    
    // Encode to base64
    let json = serde_json::to_string(&response).unwrap();
    let encoded = general_purpose::STANDARD.encode(json.as_bytes());
    
    // Validate
    let result = validate_esewa_response(&encoded, TEST_SECRET_KEY).unwrap();
    
    assert!(result.signature_valid);
    assert_eq!(result.response.status, "COMPLETE");
    assert_eq!(result.response.transaction_code, "TEST123");
}

#[test]
fn test_validate_invalid_signature() {
    let response = EsewaPaymentResponse {
        transaction_code: "TEST123".to_string(),
        status: "COMPLETE".to_string(),
        total_amount: "110.0".to_string(),
        transaction_uuid: "id-test-uuid".to_string(),
        product_code: "EPAYTEST".to_string(),
        signed_field_names: "transaction_code,status,total_amount,transaction_uuid,product_code,signed_field_names".to_string(),
        signature: "INVALID_SIGNATURE".to_string(),
    };
    
    let json = serde_json::to_string(&response).unwrap();
    let encoded = general_purpose::STANDARD.encode(json.as_bytes());
    
    let result = validate_esewa_response(&encoded, TEST_SECRET_KEY).unwrap();
    
    assert!(!result.signature_valid, "Invalid signature should be detected");
}

#[test]
fn test_validate_malformed_base64() {
    let result = validate_esewa_response("not-valid-base64!!!", TEST_SECRET_KEY);
    assert!(result.is_err(), "Should fail on invalid base64");
}

#[test]
fn test_validate_malformed_json() {
    let invalid_json = "not json data";
    let encoded = general_purpose::STANDARD.encode(invalid_json.as_bytes());
    
    let result = validate_esewa_response(&encoded, TEST_SECRET_KEY);
    assert!(result.is_err(), "Should fail on invalid JSON");
}

#[test]
fn test_payment_request_serialization() {
    let request = EsewaPaymentRequest {
        amount: "100".to_string(),
        tax_amount: "10".to_string(),
        total_amount: "110".to_string(),
        transaction_uuid: "id-123-abc".to_string(),
        product_code: "EPAYTEST".to_string(),
        product_service_charge: "0".to_string(),
        product_delivery_charge: "0".to_string(),
        success_url: "http://example.com/success".to_string(),
        failure_url: "http://example.com/failure".to_string(),
        signed_field_names: "total_amount,transaction_uuid,product_code".to_string(),
    };
    
    // Serialize and deserialize
    let json = serde_json::to_string(&request).unwrap();
    let deserialized: EsewaPaymentRequest = serde_json::from_str(&json).unwrap();
    
    assert_eq!(request.amount, deserialized.amount);
    assert_eq!(request.total_amount, deserialized.total_amount);
    assert_eq!(request.transaction_uuid, deserialized.transaction_uuid);
}

#[test]
fn test_signature_with_different_amounts() {
    let sig1 = generate_signature("100", "id-test", "EPAYTEST", TEST_SECRET_KEY);
    let sig2 = generate_signature("200", "id-test", "EPAYTEST", TEST_SECRET_KEY);
    let sig3 = generate_signature("100.0", "id-test", "EPAYTEST", TEST_SECRET_KEY);
    
    assert_ne!(sig1, sig2, "Different amounts should produce different signatures");
    assert_ne!(sig1, sig3, "Amount format matters for signature");
}

#[test]
fn test_signature_with_different_uuids() {
    let sig1 = generate_signature("100", "id-123", "EPAYTEST", TEST_SECRET_KEY);
    let sig2 = generate_signature("100", "id-456", "EPAYTEST", TEST_SECRET_KEY);
    
    assert_ne!(sig1, sig2, "Different UUIDs should produce different signatures");
}

#[test]
fn test_signature_with_different_product_codes() {
    let sig1 = generate_signature("100", "id-123", "EPAYTEST", TEST_SECRET_KEY);
    let sig2 = generate_signature("100", "id-123", "PROD123", TEST_SECRET_KEY);
    
    assert_ne!(sig1, sig2, "Different product codes should produce different signatures");
}
