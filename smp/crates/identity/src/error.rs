//! Identity error mapping.

use thiserror::Error;

#[derive(Debug, Error)]
pub enum IdentityError {
    #[error("invalid username format: {0}")]
    InvalidUsername(String),

    #[error("signature verification failed")]
    InvalidSignature,

    #[error("identity expired at {0}")]
    IdentityExpired(u64),

    #[error("identity creation timestamp is in the future: {0}")]
    InvalidTimestamp(u64),

    #[error("domain signature verification failed")]
    InvalidDomainSignature,

    #[error("serialization error: {0}")]
    SerializationError(String),

    #[error("crypto error: {0}")]
    Crypto(#[from] smp_crypto::error::CryptoError),
}
