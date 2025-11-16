//! Top-level library that re-exports the `esewa` module.

pub mod esewa;

// Re-export commonly used items so existing code and docs keep working
pub use esewa::{
    pay_with_esewa,
    generate_transaction_uuid,
    generate_signature,
    validate_esewa_response,
    EsewaPaymentRequest,
    EsewaPaymentResponse,
    ValidationResult,
    EsewaEnvironment,
    PaymentError,
};
