//! SHA-256 hashing.
//!
//! Provides a single, clean interface for hashing arbitrary data.
//! Uses the `sha2` crate — no custom implementations.

use sha2::{Digest, Sha256};

/// SHA-256 output length in bytes.
pub const SHA256_OUTPUT_LEN: usize = 32;

/// Fixed-size SHA-256 hash output.
///
/// Wraps a 32-byte array to provide strong typing instead of raw `Vec<u8>`.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Sha256Hash([u8; SHA256_OUTPUT_LEN]);

impl Sha256Hash {
    /// Returns the hash as a byte slice.
    pub fn as_bytes(&self) -> &[u8; SHA256_OUTPUT_LEN] {
        &self.0
    }

    /// Consumes the hash and returns the inner byte array.
    pub fn into_bytes(self) -> [u8; SHA256_OUTPUT_LEN] {
        self.0
    }

    /// Creates a `Sha256Hash` from a 32-byte array.
    pub fn from_bytes(bytes: [u8; SHA256_OUTPUT_LEN]) -> Self {
        Self(bytes)
    }

    /// Returns the hash as a lowercase hex string.
    pub fn to_hex(&self) -> String {
        self.0.iter().map(|b| format!("{b:02x}")).collect()
    }
}

impl AsRef<[u8]> for Sha256Hash {
    fn as_ref(&self) -> &[u8] {
        &self.0
    }
}

/// Computes the SHA-256 hash of the given data.
///
/// # Arguments
/// * `data` - The input bytes to hash.
///
/// # Returns
/// A `Sha256Hash` containing the 32-byte digest.
pub fn hash(data: &[u8]) -> Sha256Hash {
    let mut hasher = Sha256::new();
    hasher.update(data);
    let result = hasher.finalize();
    let mut output = [0u8; SHA256_OUTPUT_LEN];
    output.copy_from_slice(&result);
    Sha256Hash(output)
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Known-answer test: SHA-256 of empty input.
    /// Reference: https://en.wikipedia.org/wiki/SHA-2
    #[test]
    fn test_hash_empty() {
        let result = hash(b"");
        assert_eq!(
            result.to_hex(),
            "e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855"
        );
    }

    /// Known-answer test: SHA-256 of "abc".
    #[test]
    fn test_hash_abc() {
        let result = hash(b"abc");
        assert_eq!(
            result.to_hex(),
            "ba7816bf8f01cfea414140de5dae2223b00361a396177a9cb410ff61f20015ad"
        );
    }

    /// Same input must always produce the same output (deterministic).
    #[test]
    fn test_hash_deterministic() {
        let data = b"SMP protocol test data";
        let h1 = hash(data);
        let h2 = hash(data);
        assert_eq!(h1, h2);
    }

    /// Different inputs must produce different outputs.
    #[test]
    fn test_hash_different_inputs() {
        let h1 = hash(b"input_a");
        let h2 = hash(b"input_b");
        assert_ne!(h1, h2);
    }

    /// Output must always be exactly 32 bytes.
    #[test]
    fn test_hash_output_length() {
        let result = hash(b"arbitrary data of any length can go here");
        assert_eq!(result.as_bytes().len(), 32);
    }

    /// `from_bytes` and `into_bytes` round-trip.
    #[test]
    fn test_hash_round_trip() {
        let original = hash(b"round trip");
        let bytes = original.clone().into_bytes();
        let restored = Sha256Hash::from_bytes(bytes);
        assert_eq!(original, restored);
    }
}
