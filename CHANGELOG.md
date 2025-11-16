# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.1.0] - 2025-11-16

### Added
- Initial release
- `generate_signature()` function for HMAC-SHA256 signature generation
- `pay_with_esewa()` async function for initiating eSewa payments
- `validate_esewa_response()` function for validating payment callbacks
- `generate_transaction_uuid()` helper function for unique transaction IDs
- Comprehensive unit tests (7 tests in lib.rs)
- Integration test suite (11 tests)
- Documentation with examples
- Example web server using Actix-web
- Three standalone examples:
  - `basic_payment.rs` - Basic payment flow
  - `signature_demo.rs` - Signature generation demonstration
  - `validate_response.rs` - Response validation demonstration
- README with complete API documentation
- MIT License

### Features
- Type-safe API with custom structs
- Async/await support using tokio and reqwest
- Automatic signature verification
- Base64 encoding/decoding
- JSON serialization/deserialization
- Comprehensive error handling with custom error types

## [Unreleased]

### Planned
- Support for production eSewa endpoints
- Configuration builder pattern
- Webhook handling utilities
- More payment status options
- Retry logic for failed requests
- Payment amount validation helpers

## [0.1.1] - 2025-11-16

### Changed
- Refactor: moved implementation into `src/esewa.rs` and made `src/lib.rs` a thin re-export module to preserve the public API.
- Packaging: removed binary entry so crate is library-first (kept example server in `examples/` if present).

### Fixed
- Ensure doctests and unit/integration tests pass after reorganization.
