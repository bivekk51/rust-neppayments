# RustPayment - eSewa Payment Integration Library

A Rust library for integrating eSewa payment gateway into your applications. This library provides a simple, type-safe API for initiating payments, generating HMAC signatures, and validating payment responses.

## Features

-  **HMAC-SHA256 Signature Generation** - Secure payment authentication
-  **Payment Initialization** - Easy integration with eSewa payment gateway
-  **Response Validation** - Automatic decoding and signature verification
-  **Well-Tested** - Comprehensive unit and integration tests
-  **Zero Configuration** - Works out of the box with sensible defaults
-  **Async Support** - Built with `tokio` and `reqwest` for async operations

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
rustpayment = { path = "." }  # Or version number when published
tokio = { version = "1", features = ["full"] }
```

## Quick Start

### 1. Basic Usage

```rust
use rustpayment::{
    pay_with_esewa, 
    generate_signature, 
    validate_esewa_response,
    generate_transaction_uuid,
    EsewaPaymentRequest,
    EsewaEnvironment,
};

#[tokio::main]
async fn main() {
    // Your eSewa merchant secret key
    let secret_key = "8gBm/:&EnhH.1/q";
    
    // Create a payment request
    let request = EsewaPaymentRequest {
        amount: "100".to_string(),
        tax_amount: "10".to_string(),
        total_amount: "110".to_string(),
        transaction_uuid: generate_transaction_uuid(),
        product_code: "EPAYTEST".to_string(),
        product_service_charge: "0".to_string(),
        product_delivery_charge: "0".to_string(),
        success_url: "http://yoursite.com/success".to_string(),
        failure_url: "http://yoursite.com/failure".to_string(),
        signed_field_names: "total_amount,transaction_uuid,product_code".to_string(),
    };
    
    // Initiate payment (use `EsewaEnvironment::Production` for real integration)
    match pay_with_esewa(request, secret_key, EsewaEnvironment::Sandbox).await {
        Ok(payment_url) => {
            println!("Redirect user to: {}", payment_url);
        }
        Err(e) => {
            eprintln!("Payment error: {}", e);
        }
    }
}
```

### 2. Web Server Integration (Actix-web)

```rust
use actix_web::{get, web, App, HttpResponse, HttpServer, Responder};
use rustpayment::{
    generate_transaction_uuid, pay_with_esewa, validate_esewa_response, EsewaPaymentRequest,
};
use serde::Deserialize;

const SECRET_KEY: &str = "8gBm/:&EnhH.1/q";

#[derive(Deserialize)]
struct SuccessQuery {
    data: String,
}

#[get("/pay")]
async fn initiate_payment() -> impl Responder {
    let request = EsewaPaymentRequest {
        amount: "100".to_string(),
        tax_amount: "10".to_string(),
        total_amount: "110".to_string(),
        transaction_uuid: generate_transaction_uuid(),
        product_code: "EPAYTEST".to_string(),
        product_service_charge: "0".to_string(),
        product_delivery_charge: "0".to_string(),
        success_url: "http://127.0.0.1:8080/success".to_string(),
        failure_url: "http://127.0.0.1:8080/failure".to_string(),
        signed_field_names: "total_amount,transaction_uuid,product_code".to_string(),
    };

    match pay_with_esewa(request, SECRET_KEY, EsewaEnvironment::Sandbox).await {
        Ok(payment_url) => HttpResponse::Found()
            .append_header(("Location", payment_url))
            .finish(),
        Err(e) => HttpResponse::InternalServerError()
            .body(format!("Payment error: {}", e)),
    }
}

#[get("/success")]
async fn payment_success(query: web::Query<SuccessQuery>) -> impl Responder {
    match validate_esewa_response(&query.data, SECRET_KEY) {
        Ok(result) => {
            if result.signature_valid {
                HttpResponse::Ok().json(serde_json::json!({
                    "status": "success",
                    "transaction_code": result.response.transaction_code,
                    "amount": result.response.total_amount,
                }))
            } else {
                HttpResponse::BadRequest().json(serde_json::json!({
                    "status": "error",
                    "message": "Invalid signature"
                }))
            }
        }
        Err(e) => HttpResponse::BadRequest()
            .body(format!("Validation error: {}", e)),
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(initiate_payment)
            .service(payment_success)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
```

## API Reference

### Functions

#### `generate_signature`

Generates an HMAC-SHA256 signature for eSewa payment authentication.

```rust
pub fn generate_signature(
    total_amount: &str,
    transaction_uuid: &str,
    product_code: &str,
    secret_key: &str,
) -> String
```

**Parameters:**
- `total_amount` - Total payment amount as string
- `transaction_uuid` - Unique transaction identifier
- `product_code` - eSewa product code (e.g., "EPAYTEST")
- `secret_key` - Merchant secret key provided by eSewa

**Returns:** Base64-encoded HMAC-SHA256 signature

**Example:**
```rust
let signature = generate_signature("110", "id-123-abc", "EPAYTEST", "your_secret_key");
```

---

#### `pay_with_esewa`

Initiates a payment with eSewa and returns the redirect URL.

```rust
pub async fn pay_with_esewa(
    request: EsewaPaymentRequest,
    secret_key: &str,
) -> Result<String, PaymentError>
```

**Parameters:**
- `request` - Payment request details
- `secret_key` - Merchant secret key

**Returns:** `Result<String, PaymentError>` - Payment URL on success

**Example:**
```rust
let request = EsewaPaymentRequest { /* ... */ };
let url = pay_with_esewa(request, "secret").await?;
```

---

#### `validate_esewa_response`

Validates and decodes eSewa payment response.

```rust
pub fn validate_esewa_response(
    encoded_data: &str,
    secret_key: &str,
) -> Result<ValidationResult, PaymentError>
```

**Parameters:**
- `encoded_data` - Base64-encoded JSON data from eSewa callback
- `secret_key` - Merchant secret key

**Returns:** `Result<ValidationResult, PaymentError>` containing decoded data and signature validity

**Example:**
```rust
let result = validate_esewa_response(encoded_data, "secret")?;
if result.signature_valid {
    println!("Payment verified: {}", result.response.transaction_code);
}
```

---

#### `generate_transaction_uuid`

Generates a unique transaction identifier.

```rust
pub fn generate_transaction_uuid() -> String
```

**Returns:** UUID in format `id-<milliseconds>-<random>`

**Example:**
```rust
let uuid = generate_transaction_uuid();
// Example: "id-1763263100223-b26yhc0gy"
```

---

### Types

#### `EsewaPaymentRequest`

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

#### `EsewaPaymentResponse`

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

#### `ValidationResult`

```rust
pub struct ValidationResult {
    pub signature_valid: bool,
    pub response: EsewaPaymentResponse,
}
```

#### `PaymentError`

```rust
pub enum PaymentError {
    NetworkError(String),
    InvalidResponse(String),
    SignatureError(String),
    DecodeError(String),
}
```

## Testing

Run the test suite:

```bash
# Run all tests
cargo test

# Run with output
cargo test -- --nocapture

# Run specific test
cargo test test_signature_generation

# Run integration tests only
cargo test --test integration_tests
```

### Test Coverage

The library includes:
-  Unit tests for signature generation
-  UUID generation and uniqueness tests
-  Response validation tests
-  Serialization/deserialization tests
-  Error handling tests

## Example Project

A complete web server example is included in `src/main.rs`. Run it with:

```bash
cargo run
```

Then visit `http://127.0.0.1:8080` to test the payment flow.

## Environment Variables

For production use, consider storing sensitive data in environment variables:

```rust
use std::env;

let secret_key = env::var("ESEWA_SECRET_KEY")
    .expect("ESEWA_SECRET_KEY must be set");
```

## eSewa Integration Guide

### 1. Get Merchant Credentials
- Register as a merchant on [eSewa](https://esewa.com.np/)
- Obtain your merchant ID and secret key
- Use sandbox credentials for testing

### 2. Test Credentials
```
Product Code: EPAYTEST
Secret Key: 8gBm/:&EnhH.1/q
Environment: https://rc-epay.esewa.com.np/
```

### 3. Production Credentials
- Replace with production credentials before deployment
- Update the payment URL endpoint
- Ensure HTTPS for callback URLs

## Security Best Practices

1. **Never expose secret keys** - Store in environment variables or secure vaults
2. **Always validate signatures** - Check `signature_valid` before processing payments
3. **Use HTTPS** - Ensure callback URLs use HTTPS in production
4. **Validate amounts** - Cross-check amounts with your database
5. **Log transactions** - Keep audit logs of all payment attempts

## Error Handling

```rust
match pay_with_esewa(request, secret_key).await {
    Ok(url) => {
        // Success - redirect user
    }
    Err(PaymentError::NetworkError(e)) => {
        // Handle network errors (retry logic)
    }
    Err(PaymentError::InvalidResponse(e)) => {
        // Handle invalid API responses
    }
    Err(e) => {
        // Handle other errors
    }
}
```

## Dependencies

- `reqwest` - HTTP client for API requests
- `hmac` & `sha2` - HMAC-SHA256 signature generation
- `base64` - Base64 encoding/decoding
- `serde` & `serde_json` - JSON serialization
- `actix-web` - Web framework (example only)
- `rand` - Random UUID generation

## Contributing

Contributions are welcome! Please:
1. Fork the repository
2. Create a feature branch
3. Add tests for new functionality
4. Ensure all tests pass (`cargo test`)
5. Submit a pull request

## License

MIT License - See LICENSE file for details

## Support

For issues and questions:
- GitHub Issues: https://github.com/bivekk51/rust-neppayments
- eSewa Documentation: https://developer.esewa.com.np/

## Changelog

### Version 0.1.0
- Initial release
- Core payment functions
- Signature generation and validation
- Comprehensive test suite
- Documentation and examples

---

Made with  for the Rust community
