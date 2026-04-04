//! Structured error types for the crypto module.
//!
//! All cryptographic errors are represented as typed variants.
//! No silent failures — every error propagates via `Result`.

use thiserror::Error;

/// All errors that can occur within the crypto module.
#[derive(Debug, Error, PartialEq)]
pub enum CryptoError {
    /// AEAD encryption failed.
    #[error("encryption failed: {0}")]
    EncryptionFailed(String),

    /// AEAD decryption failed — authentication tag mismatch or corrupted data.
    /// This MUST be treated as a hard failure. No partial plaintext is ever returned.
    #[error("decryption failed: authentication or integrity check failed")]
    DecryptionFailed,

    /// Invalid key length or format.
    #[error("invalid key: {0}")]
    InvalidKey(String),

    /// Invalid nonce length.
    #[error("invalid nonce: expected {expected} bytes, got {actual}")]
    InvalidNonce { expected: usize, actual: usize },

    /// HKDF expansion failed.
    #[error("key derivation failed: {0}")]
    KeyDerivationFailed(String),

    /// Signature verification failed.
    #[error("signature verification failed")]
    SignatureVerificationFailed,

    /// Invalid signature format.
    #[error("invalid signature: {0}")]
    InvalidSignature(String),

    /// Random number generation failed.
    #[error("secure random generation failed: {0}")]
    RngFailed(String),
}
