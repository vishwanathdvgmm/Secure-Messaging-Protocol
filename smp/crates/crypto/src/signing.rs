//! Ed25519 digital signatures.
//!
//! Implements:
//! - Signing key generation
//! - Message signing
//! - Signature verification
//!
//! Uses the `ed25519-dalek` crate. No custom signature schemes.

use ed25519_dalek::{
    Signature, Signer, SigningKey as Ed25519SigningKey, Verifier,
    VerifyingKey as Ed25519VerifyingKey,
};
use rand_core::OsRng;
use serde::{Deserialize, Serialize};
use zeroize::ZeroizeOnDrop;

use crate::error::CryptoError;

/// Ed25519 private key size in bytes.
pub const SIGNING_KEY_LEN: usize = 32;

/// Ed25519 public key size in bytes.
pub const VERIFYING_KEY_LEN: usize = 32;

/// Ed25519 signature size in bytes.
pub const SIGNATURE_LEN: usize = 64;

// ---------------------------------------------------------------------------
// Signing Key Pair
// ---------------------------------------------------------------------------

/// Private signing key.
///
/// Used to create digital signatures. MUST be kept secret.
/// Zeroized on drop.
#[derive(ZeroizeOnDrop)]
pub struct SigningPrivateKey {
    #[zeroize(skip)] // Ed25519SigningKey handles its own zeroization
    inner: Ed25519SigningKey,
}

/// Public verifying key.
///
/// Used to verify digital signatures. Can be shared freely.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SigningPublicKey {
    bytes: [u8; VERIFYING_KEY_LEN],
}

/// A complete signing key pair (private + public).
pub struct SigningKeyPair {
    pub private: SigningPrivateKey,
    pub public: SigningPublicKey,
}

impl SigningPrivateKey {
    /// Returns the raw bytes of the private key.
    ///
    /// # Security
    /// Handle with care — this is secret material.
    pub fn to_bytes(&self) -> [u8; SIGNING_KEY_LEN] {
        self.inner.to_bytes()
    }
}

impl SigningPublicKey {
    /// Returns the public key as a byte slice.
    pub fn as_bytes(&self) -> &[u8; VERIFYING_KEY_LEN] {
        &self.bytes
    }

    /// Creates a verifying key from a 32-byte array.
    pub fn from_bytes(bytes: [u8; VERIFYING_KEY_LEN]) -> Self {
        Self { bytes }
    }
}

/// Generates a new Ed25519 signing key pair using OsRng.
pub fn generate_signing_key_pair() -> SigningKeyPair {
    let signing_key = Ed25519SigningKey::generate(&mut OsRng);
    let verifying_key = signing_key.verifying_key();

    SigningKeyPair {
        private: SigningPrivateKey { inner: signing_key },
        public: SigningPublicKey {
            bytes: verifying_key.to_bytes(),
        },
    }
}

// ---------------------------------------------------------------------------
// Sign / Verify operations
// ---------------------------------------------------------------------------

/// Signs a message using the provided signing key pair.
///
/// # Arguments
/// * `key_pair` - The signing key pair (private key is used).
/// * `message` - The message bytes to sign.
///
/// # Returns
/// A 64-byte Ed25519 signature.
pub fn sign(key_pair: &SigningKeyPair, message: &[u8]) -> [u8; SIGNATURE_LEN] {
    let signature = key_pair.private.inner.sign(message);
    signature.to_bytes()
}

/// Verifies an Ed25519 signature against a message and public key.
///
/// # Arguments
/// * `public_key` - The signer's public key.
/// * `message` - The original message bytes.
/// * `signature_bytes` - The 64-byte signature to verify.
///
/// # Returns
/// `Ok(())` if the signature is valid.
///
/// # Errors
/// - `CryptoError::InvalidSignature` if the signature bytes are malformed.
/// - `CryptoError::SignatureVerificationFailed` if the signature does not match.
pub fn verify(
    public_key: &SigningPublicKey,
    message: &[u8],
    signature_bytes: &[u8],
) -> Result<(), CryptoError> {
    if signature_bytes.len() != SIGNATURE_LEN {
        return Err(CryptoError::InvalidSignature(format!(
            "expected {SIGNATURE_LEN} bytes, got {}",
            signature_bytes.len()
        )));
    }

    let mut sig_array = [0u8; SIGNATURE_LEN];
    sig_array.copy_from_slice(signature_bytes);

    let signature = Signature::from_bytes(&sig_array);

    let verifying_key = Ed25519VerifyingKey::from_bytes(&public_key.bytes)
        .map_err(|e| CryptoError::InvalidKey(format!("invalid public key: {e}")))?;

    verifying_key
        .verify(message, &signature)
        .map_err(|_| CryptoError::SignatureVerificationFailed)
}

/// Signs a message using raw private key bytes.
///
/// This is the low-level function matching the doc spec interface.
/// For typed usage, prefer [sign] with key pair types.
///
/// # Arguments
/// * `private_key` - 32-byte Ed25519 private key.
/// * `message` - The message bytes to sign.
///
/// # Returns
/// 64-byte signature, or an error if the key is invalid.
pub fn sign_raw(private_key: &[u8], message: &[u8]) -> Result<[u8; SIGNATURE_LEN], CryptoError> {
    if private_key.len() != SIGNING_KEY_LEN {
        return Err(CryptoError::InvalidKey(format!(
            "signing key must be {SIGNING_KEY_LEN} bytes, got {}",
            private_key.len()
        )));
    }

    let mut key_bytes = [0u8; SIGNING_KEY_LEN];
    key_bytes.copy_from_slice(private_key);

    let signing_key = Ed25519SigningKey::from_bytes(&key_bytes);
    let signature = signing_key.sign(message);

    Ok(signature.to_bytes())
}

/// Verifies a signature using raw public key bytes.
///
/// This is the low-level function matching the doc spec interface.
/// For typed usage, prefer [verify] with key types.
///
/// # Arguments
/// * `public_key` - 32-byte Ed25519 public key.
/// * `message` - The original message bytes.
/// * `signature` - 64-byte signature.
///
/// # Returns
/// `Ok(())` if valid, error otherwise.
pub fn verify_raw(public_key: &[u8], message: &[u8], signature: &[u8]) -> Result<(), CryptoError> {
    if public_key.len() != VERIFYING_KEY_LEN {
        return Err(CryptoError::InvalidKey(format!(
            "verifying key must be {VERIFYING_KEY_LEN} bytes, got {}",
            public_key.len()
        )));
    }

    let pk = SigningPublicKey::from_bytes({
        let mut arr = [0u8; VERIFYING_KEY_LEN];
        arr.copy_from_slice(public_key);
        arr
    });

    verify(&pk, message, signature)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_signing_key_generation() {
        let kp = generate_signing_key_pair();
        assert_eq!(kp.private.to_bytes().len(), SIGNING_KEY_LEN);
        assert_eq!(kp.public.as_bytes().len(), VERIFYING_KEY_LEN);
    }

    #[test]
    fn test_unique_signing_keys() {
        let kp1 = generate_signing_key_pair();
        let kp2 = generate_signing_key_pair();
        assert_ne!(kp1.public.as_bytes(), kp2.public.as_bytes());
    }

    #[test]
    fn test_sign_and_verify() {
        let kp = generate_signing_key_pair();
        let message = b"SMP protocol message";

        let sig = sign(&kp, message);
        assert_eq!(sig.len(), SIGNATURE_LEN);

        let result = verify(&kp.public, message, &sig);
        assert!(result.is_ok());
    }

    #[test]
    fn test_verify_wrong_message_fails() {
        let kp = generate_signing_key_pair();
        let sig = sign(&kp, b"original message");

        let result = verify(&kp.public, b"tampered message", &sig);
        assert!(result.is_err());
    }

    #[test]
    fn test_verify_wrong_key_fails() {
        let kp1 = generate_signing_key_pair();
        let kp2 = generate_signing_key_pair();

        let sig = sign(&kp1, b"message");
        let result = verify(&kp2.public, b"message", &sig);
        assert!(result.is_err());
    }

    #[test]
    fn test_verify_corrupted_signature_fails() {
        let kp = generate_signing_key_pair();
        let mut sig = sign(&kp, b"message");
        // Corrupt one byte
        sig[0] ^= 0xFF;

        let result = verify(&kp.public, b"message", &sig);
        assert!(result.is_err());
    }

    #[test]
    fn test_verify_invalid_signature_length() {
        let kp = generate_signing_key_pair();
        let result = verify(&kp.public, b"message", &[0u8; 32]);
        assert!(matches!(result, Err(CryptoError::InvalidSignature(_))));
    }

    #[test]
    fn test_sign_deterministic() {
        // Ed25519 signing is deterministic for the same key and message
        let kp = generate_signing_key_pair();
        let msg = b"deterministic test";

        let sig1 = sign(&kp, msg);
        let sig2 = sign(&kp, msg);
        assert_eq!(sig1, sig2);
    }

    #[test]
    fn test_raw_sign_and_verify() {
        let kp = generate_signing_key_pair();
        let message = b"raw interface test";

        let sig = sign_raw(&kp.private.to_bytes(), message);
        assert!(sig.is_ok());

        let result = verify_raw(kp.public.as_bytes(), message, &sig.unwrap());
        assert!(result.is_ok());
    }

    #[test]
    fn test_raw_sign_invalid_key_length() {
        let result = sign_raw(&[0u8; 16], b"message");
        assert!(result.is_err());
    }

    #[test]
    fn test_raw_verify_invalid_key_length() {
        let result = verify_raw(&[0u8; 16], b"message", &[0u8; 64]);
        assert!(result.is_err());
    }

    #[test]
    fn test_public_key_round_trip() {
        let kp = generate_signing_key_pair();
        let bytes = *kp.public.as_bytes();
        let restored = SigningPublicKey::from_bytes(bytes);
        assert_eq!(kp.public, restored);
    }
}
