//! Basic payment example
//! 
//! Run with: cargo run --example basic_payment

use rustpayment::{generate_transaction_uuid, pay_with_esewa, EsewaPaymentRequest, EsewaEnvironment};

#[tokio::main]
async fn main() {
    // eSewa test credentials
    let secret_key = "8gBm/:&EnhH.1/q";//esewa recommended test secret key

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

    println!("Initiating payment for amount: {}", request.total_amount);
    println!("Transaction UUID: {}", request.transaction_uuid);

    // Initiate payment (Sandbox by default). For production use `EsewaEnvironment::Production`.
    match pay_with_esewa(request, secret_key, EsewaEnvironment::Sandbox).await {
        Ok(payment_url) => {
            println!("\n Payment initiated successfully!");
            println!("Redirect user to: {}", payment_url);
        }
        Err(e) => {
            eprintln!("\n Payment error: {}", e);
        }
    }
}
