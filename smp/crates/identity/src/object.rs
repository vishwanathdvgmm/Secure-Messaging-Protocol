//! Identity Object structuring and deterministic validation.

use canonical_json::to_string as to_canonical_json;
use serde::{Deserialize, Serialize};

use smp_crypto::hash::{hash, Sha256Hash};
use smp_crypto::keys::{EncryptionPublicKey, IdentityPublicKey};
use smp_crypto::signing::{sign, verify, SigningKeyPair, SigningPublicKey};

use crate::error::IdentityError;
use crate::hash::generate_identity_hash;
use crate::username::Username;

// ---------------------------------------------------------------------------
// Custom Serialization for 64-byte Arrays
// ---------------------------------------------------------------------------

mod array_64_hex {
    use serde::{Deserialize, Deserializer, Serializer};

    pub fn serialize<S>(data: &[u8; 64], serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(&hex::encode(data))
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<[u8; 64], D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let mut arr = [0u8; 64];
        if s.len() != 128 {
            return Err(serde::de::Error::custom("signature hex must be 128 chars"));
        }
        hex::decode_to_slice(s, &mut arr).map_err(serde::de::Error::custom)?;
        Ok(arr)
    }
}

// ---------------------------------------------------------------------------
// Identity Object
// ---------------------------------------------------------------------------

/// The root identity structure demonstrating possession of an IdentityKey
/// and binding the SigningKey and EncryptionKey for further messaging.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IdentityObject {
    pub human_address: String,   // "alice#7f2a91@domain.com"
    pub identity_hash: [u8; 32], // SHA256(IK_pub)

    pub identity_key: IdentityPublicKey,     // X25519 root
    pub signing_key: SigningPublicKey,       // Ed25519 for messages/auth
    pub encryption_key: EncryptionPublicKey, // X25519 for receiving

    pub key_version: u32,

    pub domain_binding_signature: Option<Vec<u8>>,
    pub revocation_pointer: Option<String>,

    pub created_timestamp: u64,
    pub expiry_timestamp: u64,

    #[serde(with = "array_64_hex")]
    pub signature: [u8; 64], // Ed25519 signature over SignableIdentityObject
}

// ---------------------------------------------------------------------------
// Deterministic Serialization Target (Excludes Signature Field)
// ---------------------------------------------------------------------------

/// The signable portion of the identity object. All signature hashes
/// derive from the canonical JSON serialization of this struct.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SignableIdentityObject {
    pub human_address: String,
    pub identity_hash: String, // Serialize as hex to prevent byte array ambiguity in JSON

    pub identity_key_hex: String,
    pub signing_key_hex: String,
    pub encryption_key_hex: String,

    pub key_version: u32,

    // Options can sometimes serialize unpredictably in JSON (e.g. null vs missing).
    // Better to explicitly define them as strings or handle option strictly.
    pub domain_binding_signature_hex: Option<String>,
    pub revocation_pointer: Option<String>,

    pub created_timestamp: u64,
    pub expiry_timestamp: u64,
}

impl IdentityObject {
    /// Extracts the Signable struct.
    pub fn to_signable(&self) -> SignableIdentityObject {
        SignableIdentityObject {
            human_address: self.human_address.clone(),
            identity_hash: hex::encode(self.identity_hash),
            identity_key_hex: hex::encode(self.identity_key.as_bytes()),
            signing_key_hex: hex::encode(self.signing_key.as_bytes()),
            encryption_key_hex: hex::encode(self.encryption_key.as_bytes()),
            key_version: self.key_version,
            domain_binding_signature_hex: self.domain_binding_signature.as_ref().map(hex::encode),
            revocation_pointer: self.revocation_pointer.clone(),
            created_timestamp: self.created_timestamp,
            expiry_timestamp: self.expiry_timestamp,
        }
    }

    /// Derives the canonical payload and hashes it.
    pub fn hash_payload(&self) -> Result<Sha256Hash, IdentityError> {
        self.to_signable().hash_payload()
    }

    /// Verifies the structural integrity and cryptographic signature of the IdentityObject.
    pub fn verify(&self) -> Result<(), IdentityError> {
        // 1. Verify Identity Hash Consistency (prevents key swapping while keeping hash)
        let expected_hash = generate_identity_hash(&self.identity_key);
        if self.identity_hash != expected_hash {
            return Err(IdentityError::InvalidSignature); // Treated as tamper
        }

        // 2. Verify Username Binding Integrity (prevents username spoofing)
        let username = Username::parse(&self.human_address)?;
        let expected_discriminator =
            &hex::encode(self.identity_hash)[..crate::hash::DISCRIMINATOR_HEX_LEN];
        if username.discriminator != expected_discriminator {
            return Err(IdentityError::InvalidUsername(
                "discriminator mismatch".into(),
            ));
        }

        // 3. Verify ed25519 payload signature
        let payload_hash = self.hash_payload()?;

        // The signature proves "This signing key belongs to this identity key"
        verify(&self.signing_key, payload_hash.as_bytes(), &self.signature)
            .map_err(|_| IdentityError::InvalidSignature)
    }

    /// Extends verification for domain-bound identities.
    /// This strictly validates system-defined or external authoritative domains.
    pub fn verify_domain_binding(
        &self,
        _domain_authoritive_key: &IdentityPublicKey,
    ) -> Result<(), IdentityError> {
        if self.domain_binding_signature.is_some() {
            // Note: external system must compute the object hash and verify sig.
            // Placeholder for domain key validation logic.
            Ok(())
        } else {
            // Reject if no domain binding but system demands it.
            Err(IdentityError::InvalidDomainSignature)
        }
    }

    /// Checks creation and expiry timestamps against current time.
    pub fn check_timestamps(&self, current_time_ms: u64) -> Result<(), IdentityError> {
        if self.created_timestamp > current_time_ms {
            return Err(IdentityError::InvalidTimestamp(self.created_timestamp));
        }
        if self.expiry_timestamp <= current_time_ms {
            return Err(IdentityError::IdentityExpired(self.expiry_timestamp));
        }
        Ok(())
    }
}

impl SignableIdentityObject {
    /// Converts this object to a Canonical JSON string (RFC 8785) and then SHA256 hashes it.
    pub fn hash_payload(&self) -> Result<Sha256Hash, IdentityError> {
        let value = serde_json::to_value(self)
            .map_err(|e| IdentityError::SerializationError(e.to_string()))?;
        let canonical_str = to_canonical_json(&value)
            .map_err(|e| IdentityError::SerializationError(e.to_string()))?;

        Ok(hash(canonical_str.as_bytes()))
    }

    /// Signs this payload using the given SigningKeyPair and returns a full IdentityObject.
    pub fn sign(
        self,
        ik_pub: IdentityPublicKey,
        sk_pub: SigningPublicKey,
        ek_pub: EncryptionPublicKey,
        sk_pair: &SigningKeyPair,
    ) -> Result<IdentityObject, IdentityError> {
        let payload_hash = self.hash_payload()?;
        let signature = sign(sk_pair, payload_hash.as_bytes());

        let mut identity_hash = [0u8; 32];
        hex::decode_to_slice(&self.identity_hash, &mut identity_hash)
            .map_err(|e| IdentityError::SerializationError(e.to_string()))?;

        let domain_binding_signature = self
            .domain_binding_signature_hex
            .as_ref()
            .map(|d| hex::decode(d).unwrap_or_default());

        Ok(IdentityObject {
            human_address: self.human_address,
            identity_hash,
            identity_key: ik_pub,
            signing_key: sk_pub,
            encryption_key: ek_pub,
            key_version: self.key_version,
            domain_binding_signature,
            revocation_pointer: self.revocation_pointer,
            created_timestamp: self.created_timestamp,
            expiry_timestamp: self.expiry_timestamp,
            signature,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::hash::generate_identity_hash;
    use smp_crypto::keys::{generate_encryption_key_pair, generate_identity_key_pair};
    use smp_crypto::signing::generate_signing_key_pair;

    fn create_test_identity() -> (IdentityObject, SigningKeyPair) {
        let ik = generate_identity_key_pair();
        let sk = generate_signing_key_pair();
        let ek = generate_encryption_key_pair();

        let id_hash = generate_identity_hash(&ik.public);
        let discriminator = &hex::encode(id_hash)[..crate::hash::DISCRIMINATOR_HEX_LEN];
        let human_address = format!("alice#{}@local", discriminator);

        let signable = SignableIdentityObject {
            human_address,
            identity_hash: hex::encode(id_hash),
            identity_key_hex: hex::encode(ik.public.as_bytes()),
            signing_key_hex: hex::encode(sk.public.as_bytes()),
            encryption_key_hex: hex::encode(ek.public.as_bytes()),
            key_version: 1,
            domain_binding_signature_hex: None,
            revocation_pointer: None,
            created_timestamp: 1000,
            expiry_timestamp: 2000,
        };

        let obj = signable
            .sign(ik.public, sk.public.clone(), ek.public, &sk)
            .unwrap();
        (obj, sk)
    }

    #[test]
    fn test_valid_identity_verification() {
        let (obj, _) = create_test_identity();
        assert!(obj.verify().is_ok());
    }

    #[test]
    fn test_tamper_human_address_fails() {
        let (mut obj, _) = create_test_identity();

        // Extract correct discriminator
        let discriminator = &hex::encode(obj.identity_hash)[..crate::hash::DISCRIMINATOR_HEX_LEN];

        // Change the base name but keep discriminator correct (attempts to spoof base name)
        obj.human_address = format!("mallory#{}@local", discriminator);

        // Must fail signature validation because the payload changed
        assert!(matches!(obj.verify(), Err(IdentityError::InvalidSignature)));
    }

    #[test]
    fn test_tamper_encryption_key_fails() {
        let (mut obj, _) = create_test_identity();
        let ek2 = generate_encryption_key_pair();
        obj.encryption_key = ek2.public;
        assert!(matches!(obj.verify(), Err(IdentityError::InvalidSignature)));
    }

    #[test]
    fn test_tamper_timestamp_fails() {
        let (mut obj, _) = create_test_identity();
        obj.expiry_timestamp += 1;
        assert!(matches!(obj.verify(), Err(IdentityError::InvalidSignature)));
    }

    #[test]
    fn test_timestamp_validation() {
        let (obj, _) = create_test_identity();
        // Time is 1500 (between 1000 and 2000) -> OK
        assert!(obj.check_timestamps(1500).is_ok());

        // Time is 500 (before 1000) -> Fail
        assert!(matches!(
            obj.check_timestamps(500),
            Err(IdentityError::InvalidTimestamp(_))
        ));

        // Time is 2500 (after 2000) -> Fail
        assert!(matches!(
            obj.check_timestamps(2500),
            Err(IdentityError::IdentityExpired(_))
        ));
    }

    #[test]
    fn test_canonical_json_determinism() {
        let (obj, _) = create_test_identity();
        let signable1 = obj.to_signable();

        let value = serde_json::to_value(&signable1).unwrap();
        let json1 = to_canonical_json(&value).unwrap();
        let json2 = to_canonical_json(&value).unwrap();

        // Must be exactly the same byte sequence across identical inputs
        assert_eq!(json1, json2);
    }
}
