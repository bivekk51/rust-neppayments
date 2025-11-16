# Quick Start Guide

## Installation

Add to your `Cargo.toml`:

```toml
[dependencies]
rustpayment = { path = "." }
tokio = { version = "1", features = ["full"] }
```

## Basic Usage

### 1. Generate a Signature

```rust
use rustpayment::generate_signature;

let signature = generate_signature(
    "110",              // total_amount
    "id-123-abc",       // transaction_uuid
    "EPAYTEST",         // product_code
    "8gBm/:&EnhH.1/q"  // secret_key
);
println!("Signature: {}", signature);
```

### 2. Initiate Payment

```rust
use rustpayment::{pay_with_esewa, generate_transaction_uuid, EsewaPaymentRequest};

#[tokio::main]
async fn main() {
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
    
    match pay_with_esewa(request, "8gBm/:&EnhH.1/q").await {
        Ok(url) => println!("Redirect to: {}", url),
        Err(e) => eprintln!("Error: {}", e),
    }
}
```

### 3. Validate Response

```rust
use rustpayment::validate_esewa_response;

let encoded_data = "eyJ0cmFuc2FjdGlvbl9jb2RlIjoiVEVTVDEyMyJ9"; // From eSewa

match validate_esewa_response(encoded_data, "8gBm/:&EnhH.1/q") {
    Ok(result) => {
        if result.signature_valid {
            println!("✅ Payment verified!");
            println!("Transaction: {}", result.response.transaction_code);
            println!("Status: {}", result.response.status);
        } else {
            println!("⚠️ Invalid signature!");
        }
    }
    Err(e) => eprintln!("Validation error: {}", e),
}
```

## Run Examples

```bash
# Signature generation demo
cargo run --example signature_demo

# Response validation demo
cargo run --example validate_response

# Basic payment flow (requires network)
cargo run --example basic_payment

# Web server example
cargo run
# Then visit http://127.0.0.1:8080
```

## Testing

```bash
# Run all tests
cargo test

# Run with output
cargo test -- --nocapture

# Run specific test
cargo test test_signature_generation
```

## Web Integration (Actix-web)

```rust
use actix_web::{get, web, HttpResponse};
use rustpayment::{validate_esewa_response, EsewaPaymentRequest};
use serde::Deserialize;

#[derive(Deserialize)]
struct SuccessQuery {
    data: String,
}

#[get("/success")]
async fn success(query: web::Query<SuccessQuery>) -> HttpResponse {
    match validate_esewa_response(&query.data, "8gBm/:&EnhH.1/q") {
        Ok(result) if result.signature_valid => {
            HttpResponse::Ok().json(serde_json::json!({
                "status": "success",
                "transaction": result.response.transaction_code
            }))
        }
        _ => HttpResponse::BadRequest().body("Invalid payment")
    }
}
```

## Production Checklist

- [ ] Replace test credentials with production keys
- [ ] Update payment URL for production environment
- [ ] Use HTTPS for callback URLs
- [ ] Store secret keys in environment variables
- [ ] Implement transaction logging
- [ ] Add amount validation against your database
- [ ] Set up error monitoring
- [ ] Test signature validation thoroughly

## Environment Variables

```bash
# .env file
ESEWA_SECRET_KEY=your_production_secret_key
ESEWA_PRODUCT_CODE=your_product_code
ESEWA_PAYMENT_URL=https://esewa.com.np/api/epay/main/v2/form
```

```rust
use std::env;

let secret_key = env::var("ESEWA_SECRET_KEY")
    .expect("ESEWA_SECRET_KEY not set");
```

## Common Issues

### Issue: Signature mismatch
**Solution**: Ensure amounts match exactly including decimal places

### Issue: Network timeout
**Solution**: Check internet connection and eSewa service status

### Issue: Invalid base64 data
**Solution**: Ensure data parameter is properly URL-encoded

## Support

- Documentation: See [README.md](README.md)
- Examples: Check `examples/` directory
- Tests: Review `tests/` for usage patterns
- eSewa Docs: https://developer.esewa.com.np/
