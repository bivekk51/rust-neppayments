# API Documentation

## Table of Contents
- [Functions](#functions)
  - [generate_signature](#generate_signature)
  - [pay_with_esewa](#pay_with_esewa)
  - [validate_esewa_response](#validate_esewa_response)
  - [generate_transaction_uuid](#generate_transaction_uuid)
- [Types](#types)
  - [EsewaPaymentRequest](#esewapaymentrequest)
  - [EsewaPaymentResponse](#esewapaymentresponse)
  - [ValidationResult](#validationresult)
  - [PaymentError](#paymenterror)

---

## Functions

### `generate_signature`

Generates an HMAC-SHA256 signature for eSewa payment authentication.

**Signature:**
```rust
pub fn generate_signature(
    total_amount: &str,
    transaction_uuid: &str,
    product_code: &str,
    secret_key: &str,
) -> String
```

**Parameters:**
- `total_amount: &str` - The total payment amount (including tax, fees, etc.)
- `transaction_uuid: &str` - Unique transaction identifier
- `product_code: &str` - eSewa merchant product code (e.g., "EPAYTEST" for sandbox)
- `secret_key: &str` - Secret key provided by eSewa merchant account

**Returns:** `String` - Base64-encoded HMAC-SHA256 signature

**Example:**
```rust
let sig = generate_signature("110", "id-123-abc", "EPAYTEST", "8gBm/:&EnhH.1/q");
assert!(!sig.is_empty());
```

**Notes:**
- The signature is computed over the string: `total_amount={amount},transaction_uuid={uuid},product_code={code}`
- Same inputs will always produce the same signature
- Used internally by `pay_with_esewa` but can be called directly if needed

---

### `pay_with_esewa`

Initiates a payment with eSewa and returns the payment redirect URL.

**Signature:**
```rust
pub async fn pay_with_esewa(
    request: EsewaPaymentRequest,
    secret_key: &str,
) -> Result<String, PaymentError>
```

**Parameters:**
- `request: EsewaPaymentRequest` - Complete payment request details
- `secret_key: &str` - eSewa merchant secret key

**Returns:** `Result<String, PaymentError>`
- **Ok(String)** - Payment URL to redirect the user to
- **Err(PaymentError)** - Error if request fails

**Example:**
```rust
let request = EsewaPaymentRequest {
    amount: "100".to_string(),
    tax_amount: "10".to_string(),
    total_amount: "110".to_string(),
    transaction_uuid: generate_transaction_uuid(),
    product_code: "EPAYTEST".to_string(),
    product_service_charge: "0".to_string(),
    product_delivery_charge: "0".to_string(),
    success_url: "http://example.com/success".to_string(),
    failure_url: "http://example.com/failure".to_string(),
    signed_field_names: "total_amount,transaction_uuid,product_code".to_string(),
};

let url = pay_with_esewa(request, "8gBm/:&EnhH.1/q").await?;
// Redirect user to `url`
```

**Error Handling:**
```rust
match pay_with_esewa(request, secret_key).await {
    Ok(url) => {
        // Success - redirect user
    }
    Err(PaymentError::NetworkError(e)) => {
        // Network/connection error
    }
    Err(PaymentError::InvalidResponse(e)) => {
        // Invalid API response
    }
    Err(e) => {
        // Other errors
    }
}
```

---

### `validate_esewa_response`

Validates and decodes the payment response from eSewa callback.

**Signature:**
```rust
pub fn validate_esewa_response(
    encoded_data: &str,
    secret_key: &str,
) -> Result<ValidationResult, PaymentError>
```

**Parameters:**
- `encoded_data: &str` - Base64-encoded JSON data from eSewa's `?data=` query parameter
- `secret_key: &str` - eSewa merchant secret key

**Returns:** `Result<ValidationResult, PaymentError>`
- **Ok(ValidationResult)** - Decoded response with signature validation status
- **Err(PaymentError)** - Error if decoding or parsing fails

**Example:**
```rust
// In your success callback handler
let encoded = query_params.get("data").unwrap();

match validate_esewa_response(encoded, "8gBm/:&EnhH.1/q") {
    Ok(result) => {
        if result.signature_valid {
            // Process successful payment
            println!("Transaction: {}", result.response.transaction_code);
            println!("Amount: {}", result.response.total_amount);
        } else {
            // Invalid signature - potential fraud
            log::warn!("Invalid signature detected!");
        }
    }
    Err(e) => {
        // Handle validation error
    }
}
```

**Important:**
- Always check `result.signature_valid` before processing payment
- Log all validation failures for security monitoring
- Invalid signatures may indicate tampering or fraud attempts

---

### `generate_transaction_uuid`

Generates a unique transaction identifier.

**Signature:**
```rust
pub fn generate_transaction_uuid() -> String
```

**Returns:** `String` - UUID in format `id-<milliseconds>-<random_chars>`

**Example:**
```rust
let uuid1 = generate_transaction_uuid();
let uuid2 = generate_transaction_uuid();
// Example outputs: 
// "id-1763263100223-b26yhc0gy"
// "id-1763263100224-x8kp3md9z"
assert_ne!(uuid1, uuid2);
```

**Format Details:**
- Prefix: "id-"
- Timestamp: Current time in milliseconds since UNIX epoch
- Random suffix: 9 alphanumeric characters (base36)

---

## Types

### `EsewaPaymentRequest`

Represents the complete payment request data.

```rust
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
```

**Fields:**
- `amount` - Base payment amount (without tax or fees)
- `tax_amount` - Tax amount
- `total_amount` - Total amount (amount + tax + service + delivery)
- `transaction_uuid` - Unique transaction identifier
- `product_code` - eSewa merchant product code
- `product_service_charge` - Service charge amount
- `product_delivery_charge` - Delivery charge amount
- `success_url` - URL to redirect on successful payment
- `failure_url` - URL to redirect on failed payment
- `signed_field_names` - Comma-separated list of fields to sign (typically "total_amount,transaction_uuid,product_code")

**Traits:** `Debug`, `Clone`, `Serialize`, `Deserialize`

---

### `EsewaPaymentResponse`

Represents the decoded payment response from eSewa.

```rust
pub struct EsewaPaymentResponse {
    pub transaction_code: String,
    pub status: String,
    pub total_amount: String,
    pub transaction_uuid: String,
    pub product_code: String,
    pub signed_field_names: String,
    pub signature: String,
}
```

**Fields:**
- `transaction_code` - eSewa's transaction reference code
- `status` - Payment status (e.g., "COMPLETE", "FAILED")
- `total_amount` - Total amount paid
- `transaction_uuid` - Your original transaction UUID
- `product_code` - Product code used
- `signed_field_names` - Fields included in signature
- `signature` - HMAC signature from eSewa

**Traits:** `Debug`, `Clone`, `Serialize`, `Deserialize`

**Status Values:**
- `"COMPLETE"` - Payment successful
- `"FAILED"` - Payment failed
- Other values may exist - check eSewa documentation

---

### `ValidationResult`

Result of response validation including signature check.

```rust
pub struct ValidationResult {
    pub signature_valid: bool,
    pub response: EsewaPaymentResponse,
}
```

**Fields:**
- `signature_valid` - `true` if signature matches, `false` otherwise
- `response` - Decoded payment response data

**Traits:** `Debug`, `Clone`, `Serialize`, `Deserialize`

**Usage Pattern:**
```rust
let result = validate_esewa_response(encoded, secret_key)?;
if result.signature_valid && result.response.status == "COMPLETE" {
    // Process payment
}
```

---

### `PaymentError`

Error type for payment operations.

```rust
pub enum PaymentError {
    NetworkError(String),
    InvalidResponse(String),
    SignatureError(String),
    DecodeError(String),
}
```

**Variants:**
- `NetworkError` - HTTP request failed (network issue, timeout, etc.)
- `InvalidResponse` - Server returned unexpected response (non-200 status, etc.)
- `SignatureError` - Signature generation or validation failed
- `DecodeError` - Base64 decoding or JSON parsing failed

**Traits:** `Debug`, `Display`, `Error`

**Example Handling:**
```rust
match operation() {
    Err(PaymentError::NetworkError(e)) => {
        log::error!("Network error: {}", e);
        // Retry logic
    }
    Err(PaymentError::DecodeError(e)) => {
        log::error!("Invalid data format: {}", e);
        // Don't retry - bad data
    }
    Err(e) => {
        log::error!("Payment error: {}", e);
    }
    Ok(result) => { /* success */ }
}
```

---

## Testing

All functions are thoroughly tested. Run tests with:

```bash
cargo test
```

See `tests/integration_tests.rs` for comprehensive test examples.

---

## Thread Safety

All functions are thread-safe and can be called concurrently.

## Async Runtime

`pay_with_esewa` requires an async runtime (e.g., tokio):

```toml
[dependencies]
tokio = { version = "1", features = ["full"] }
```

```rust
#[tokio::main]
async fn main() {
    // Use async functions
}
```
