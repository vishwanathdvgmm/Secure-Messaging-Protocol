//! Key types and Diffie-Hellman key exchange.
//!
//! Implements:
//! - Strong key types (IdentityKey, EncryptionKey, SessionKeys)
//! - X25519 Diffie-Hellman key exchange
//! - Key generation using OsRng (cryptographically secure)
//!
//! Key types use the newtype pattern to prevent accidental misuse
//! of raw byte arrays.

use rand_core::OsRng;
use serde::{Deserialize, Serialize};
use x25519_dalek::{PublicKey as X25519PublicKey, StaticSecret as X25519StaticSecret};
use zeroize::{Zeroize, ZeroizeOnDrop};

use crate::error::CryptoError;

/// X25519 key size in bytes.
pub const DH_KEY_LEN: usize = 32;

/// Shared secret size from X25519 DH exchange.
pub const DH_SHARED_SECRET_LEN: usize = 32;

// ---------------------------------------------------------------------------
// Identity Key Pair (X25519 for DH-based identity)
// ---------------------------------------------------------------------------

/// Private component of an identity key pair.
///
/// This key MUST be kept secret and MUST be zeroized when dropped.
/// It is used for Diffie-Hellman exchanges during session establishment.
#[derive(ZeroizeOnDrop)]
pub struct IdentityPrivateKey {
    inner: X25519StaticSecret,
}

/// Public component of an identity key pair.
///
/// This key is shared with other participants and published to the registry.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct IdentityPublicKey {
    bytes: [u8; DH_KEY_LEN],
}

/// A complete identity key pair (private + public).
pub struct IdentityKeyPair {
    pub private: IdentityPrivateKey,
    pub public: IdentityPublicKey,
}

impl IdentityPrivateKey {
    /// Returns the raw bytes of the private key.
    ///
    /// # Safety
    /// Caller is responsible for handling secret material securely.
    pub fn to_bytes(&self) -> [u8; DH_KEY_LEN] {
        self.inner.to_bytes()
    }

    /// Performs X25519 Diffie-Hellman with the given public key.
    pub fn diffie_hellman(&self, their_public: &IdentityPublicKey) -> SharedSecret {
        let their_pk = X25519PublicKey::from(their_public.bytes);
        let shared = self.inner.diffie_hellman(&their_pk);
        SharedSecret {
            bytes: shared.to_bytes(),
        }
    }
}

impl IdentityPublicKey {
    /// Returns the public key as a byte slice.
    pub fn as_bytes(&self) -> &[u8; DH_KEY_LEN] {
        &self.bytes
    }

    /// Creates a public key from a 32-byte array.
    pub fn from_bytes(bytes: [u8; DH_KEY_LEN]) -> Self {
        Self { bytes }
    }
}

/// Generates a new identity key pair using secure randomness (OsRng).
pub fn generate_identity_key_pair() -> IdentityKeyPair {
    let secret = X25519StaticSecret::random_from_rng(OsRng);
    let public = X25519PublicKey::from(&secret);

    IdentityKeyPair {
        private: IdentityPrivateKey { inner: secret },
        public: IdentityPublicKey {
            bytes: public.to_bytes(),
        },
    }
}

// ---------------------------------------------------------------------------
// Encryption Key Pair (X25519 for ephemeral / pre-key DH)
// ---------------------------------------------------------------------------

/// Private component of an encryption key pair.
///
/// Used for ephemeral and pre-key Diffie-Hellman exchanges.
/// Zeroized on drop.
#[derive(ZeroizeOnDrop)]
pub struct EncryptionPrivateKey {
    inner: X25519StaticSecret,
}

/// Public component of an encryption key pair.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct EncryptionPublicKey {
    bytes: [u8; DH_KEY_LEN],
}

/// A complete encryption key pair (private + public).
pub struct EncryptionKeyPair {
    pub private: EncryptionPrivateKey,
    pub public: EncryptionPublicKey,
}

impl EncryptionPrivateKey {
    /// Returns the raw bytes of the private key.
    pub fn to_bytes(&self) -> [u8; DH_KEY_LEN] {
        self.inner.to_bytes()
    }

    /// Performs X25519 Diffie-Hellman with a given public key.
    pub fn diffie_hellman(&self, their_public: &EncryptionPublicKey) -> SharedSecret {
        let their_pk = X25519PublicKey::from(their_public.bytes);
        let shared = self.inner.diffie_hellman(&their_pk);
        SharedSecret {
            bytes: shared.to_bytes(),
        }
    }

    /// Performs X25519 Diffie-Hellman with an identity public key.
    pub fn diffie_hellman_identity(&self, their_public: &IdentityPublicKey) -> SharedSecret {
        let their_pk = X25519PublicKey::from(*their_public.as_bytes());
        let shared = self.inner.diffie_hellman(&their_pk);
        SharedSecret {
            bytes: shared.to_bytes(),
        }
    }
}

impl EncryptionPublicKey {
    /// Returns the public key as a byte slice.
    pub fn as_bytes(&self) -> &[u8; DH_KEY_LEN] {
        &self.bytes
    }

    /// Creates a public key from a 32-byte array.
    pub fn from_bytes(bytes: [u8; DH_KEY_LEN]) -> Self {
        Self { bytes }
    }
}

impl IdentityPrivateKey {
    /// Performs X25519 Diffie-Hellman with an encryption public key.
    pub fn diffie_hellman_encryption(&self, their_public: &EncryptionPublicKey) -> SharedSecret {
        let their_pk = X25519PublicKey::from(*their_public.as_bytes());
        let shared = self.inner.diffie_hellman(&their_pk);
        SharedSecret {
            bytes: shared.to_bytes(),
        }
    }
}

/// Generates a new encryption key pair using secure randomness (OsRng).
pub fn generate_encryption_key_pair() -> EncryptionKeyPair {
    let secret = X25519StaticSecret::random_from_rng(OsRng);
    let public = X25519PublicKey::from(&secret);

    EncryptionKeyPair {
        private: EncryptionPrivateKey { inner: secret },
        public: EncryptionPublicKey {
            bytes: public.to_bytes(),
        },
    }
}

// ---------------------------------------------------------------------------
// Shared Secret
// ---------------------------------------------------------------------------

/// The output of a Diffie-Hellman key exchange.
///
/// Contains the raw 32-byte shared secret that should be fed into HKDF
/// for further key derivation. Never used directly as an encryption key.
#[derive(Zeroize, ZeroizeOnDrop)]
pub struct SharedSecret {
    bytes: [u8; DH_SHARED_SECRET_LEN],
}

impl SharedSecret {
    /// Returns the shared secret as a byte slice.
    ///
    /// # Security
    /// This value MUST be processed through HKDF before use as a key.
    pub fn as_bytes(&self) -> &[u8; DH_SHARED_SECRET_LEN] {
        &self.bytes
    }
}

// ---------------------------------------------------------------------------
// Generic DH function (matches doc interface)
// ---------------------------------------------------------------------------

/// Performs a raw X25519 Diffie-Hellman exchange.
///
/// This is the low-level DH function defined in the crypto module spec.
/// For typed key exchanges, prefer the methods on key pair types.
///
/// # Arguments
/// * `private_key` - 32-byte X25519 private key
/// * `public_key` - 32-byte X25519 public key
///
/// # Returns
/// 32-byte shared secret, or an error if key lengths are invalid.
///
/// # Errors
/// Returns `CryptoError::InvalidKey` if key lengths are not exactly 32 bytes.
pub fn dh(
    private_key: &[u8],
    public_key: &[u8],
) -> Result<[u8; DH_SHARED_SECRET_LEN], CryptoError> {
    if private_key.len() != DH_KEY_LEN {
        return Err(CryptoError::InvalidKey(format!(
            "private key must be {DH_KEY_LEN} bytes, got {}",
            private_key.len()
        )));
    }
    if public_key.len() != DH_KEY_LEN {
        return Err(CryptoError::InvalidKey(format!(
            "public key must be {DH_KEY_LEN} bytes, got {}",
            public_key.len()
        )));
    }

    let mut priv_bytes = [0u8; DH_KEY_LEN];
    priv_bytes.copy_from_slice(private_key);

    let mut pub_bytes = [0u8; DH_KEY_LEN];
    pub_bytes.copy_from_slice(public_key);

    let secret = X25519StaticSecret::from(priv_bytes);
    let their_public = X25519PublicKey::from(pub_bytes);
    let shared = secret.diffie_hellman(&their_public);

    // Zeroize local copies
    priv_bytes.zeroize();
    pub_bytes.zeroize();

    Ok(shared.to_bytes())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_identity_key_generation() {
        let kp = generate_identity_key_pair();
        // Public key must be 32 bytes
        assert_eq!(kp.public.as_bytes().len(), DH_KEY_LEN);
        // Private key must be 32 bytes
        assert_eq!(kp.private.to_bytes().len(), DH_KEY_LEN);
    }

    #[test]
    fn test_encryption_key_generation() {
        let kp = generate_encryption_key_pair();
        assert_eq!(kp.public.as_bytes().len(), DH_KEY_LEN);
        assert_eq!(kp.private.to_bytes().len(), DH_KEY_LEN);
    }

    #[test]
    fn test_unique_key_pairs() {
        let kp1 = generate_identity_key_pair();
        let kp2 = generate_identity_key_pair();
        // Two random key pairs must be different
        assert_ne!(kp1.public.as_bytes(), kp2.public.as_bytes());
    }

    #[test]
    fn test_dh_shared_secret_agreement() {
        // Alice and Bob generate key pairs
        let alice = generate_identity_key_pair();
        let bob = generate_identity_key_pair();

        // Both compute shared secret — must match
        let shared_ab = alice.private.diffie_hellman(&bob.public);
        let shared_ba = bob.private.diffie_hellman(&alice.public);

        assert_eq!(shared_ab.as_bytes(), shared_ba.as_bytes());
    }

    #[test]
    fn test_dh_different_peers_different_secrets() {
        let alice = generate_identity_key_pair();
        let bob = generate_identity_key_pair();
        let carol = generate_identity_key_pair();

        let shared_ab = alice.private.diffie_hellman(&bob.public);
        let shared_ac = alice.private.diffie_hellman(&carol.public);

        assert_ne!(shared_ab.as_bytes(), shared_ac.as_bytes());
    }

    #[test]
    fn test_raw_dh_function() {
        let alice = generate_identity_key_pair();
        let bob = generate_identity_key_pair();

        let shared1 = dh(&alice.private.to_bytes(), bob.public.as_bytes());
        let shared2 = dh(&bob.private.to_bytes(), alice.public.as_bytes());

        assert!(shared1.is_ok());
        assert!(shared2.is_ok());
        assert_eq!(shared1.unwrap(), shared2.unwrap());
    }

    #[test]
    fn test_raw_dh_invalid_key_length() {
        let result = dh(&[0u8; 16], &[0u8; 32]);
        assert!(result.is_err());

        let result = dh(&[0u8; 32], &[0u8; 16]);
        assert!(result.is_err());
    }

    #[test]
    fn test_cross_key_type_dh() {
        let identity = generate_identity_key_pair();
        let encryption = generate_encryption_key_pair();

        let shared1 = identity
            .private
            .diffie_hellman_encryption(&encryption.public);
        let shared2 = encryption.private.diffie_hellman_identity(&identity.public);

        assert_eq!(shared1.as_bytes(), shared2.as_bytes());
    }

    #[test]
    fn test_public_key_serialization() {
        let kp = generate_identity_key_pair();
        let bytes = *kp.public.as_bytes();
        let restored = IdentityPublicKey::from_bytes(bytes);
        assert_eq!(kp.public, restored);
    }
}
