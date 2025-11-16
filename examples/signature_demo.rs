//! Signature generation example
//! 
//! Run with: cargo run --example signature_demo

use rustpayment::generate_signature;

fn main() {
    let secret_key = "8gBm/:&EnhH.1/q";
    
    println!("eSewa Signature Generation Demo\n");
    println!("================================\n");

    // Example 1: Basic signature
    let sig1 = generate_signature("110", "id-123-abc", "EPAYTEST", secret_key);
    println!("Example 1:");
    println!("  Total Amount: 110");
    println!("  Transaction UUID: id-123-abc");
    println!("  Product Code: EPAYTEST");
    println!("  Signature: {}\n", sig1);

    // Example 2: Different amount
    let sig2 = generate_signature("250", "id-456-def", "EPAYTEST", secret_key);
    println!("Example 2:");
    println!("  Total Amount: 250");
    println!("  Transaction UUID: id-456-def");
    println!("  Product Code: EPAYTEST");
    println!("  Signature: {}\n", sig2);

    // Example 3: Verify consistency
    let sig3a = generate_signature("100", "id-999-xyz", "EPAYTEST", secret_key);
    let sig3b = generate_signature("100", "id-999-xyz", "EPAYTEST", secret_key);
    println!("Example 3 (Consistency Check):");
    println!("  First signature:  {}", sig3a);
    println!("  Second signature: {}", sig3b);
    println!("  Are they equal? {}", sig3a == sig3b);
}
