//! HKDF key derivation using SHA-256.
//!
//! Implements the HMAC-based Key Derivation Function (RFC 5869)
//! using SHA-256 as the underlying hash function.
//!
//! Used for deriving session keys, chain keys, and message keys
//! throughout the protocol.

use hkdf::Hkdf;
use sha2::Sha256;
use zeroize::Zeroize;

use crate::error::CryptoError;

/// Maximum output length for HKDF-SHA256.
/// Per RFC 5869: L <= 255 * HashLen = 255 * 32 = 8160 bytes.
pub const HKDF_MAX_OUTPUT_LEN: usize = 255 * 32;

/// Derives key material using HKDF-SHA256.
///
/// This follows the Extract-then-Expand paradigm from RFC 5869:
/// 1. Extract: HMAC-SHA256(salt, input_key_material) → PRK
/// 2. Expand: PRK + info → output key material
///
/// # Arguments
/// * `input_key_material` - The input keying material (IKM).
/// * `salt` - Optional salt value. If `None`, a zero-filled salt is used.
/// * `info` - Context and application specific information.
/// * `length` - Desired output length in bytes. Must be <= 8160.
///
/// # Returns
/// Derived key material of the requested length, or an error.
///
/// # Errors
/// Returns `CryptoError::KeyDerivationFailed` if:
/// - `length` is 0
/// - `length` exceeds maximum (8160 bytes)
/// - HKDF expansion fails internally
pub fn derive_key(
    input_key_material: &[u8],
    salt: Option<&[u8]>,
    info: &[u8],
    length: usize,
) -> Result<Vec<u8>, CryptoError> {
    if length == 0 {
        return Err(CryptoError::KeyDerivationFailed(
            "output length must be greater than 0".into(),
        ));
    }
    if length > HKDF_MAX_OUTPUT_LEN {
        return Err(CryptoError::KeyDerivationFailed(format!(
            "output length {length} exceeds maximum {HKDF_MAX_OUTPUT_LEN}"
        )));
    }

    let hk = Hkdf::<Sha256>::new(salt, input_key_material);

    let mut output = vec![0u8; length];
    hk.expand(info, &mut output).map_err(|e| {
        output.zeroize();
        CryptoError::KeyDerivationFailed(format!("HKDF expansion failed: {e}"))
    })?;

    Ok(output)
}

#[cfg(test)]
mod tests {
    use super::*;

    /// RFC 5869 Test Case 1: Basic test with SHA-256.
    #[test]
    fn test_hkdf_rfc5869_case1() {
        let ikm = hex_decode("0b0b0b0b0b0b0b0b0b0b0b0b0b0b0b0b0b0b0b0b0b0b");
        let salt = hex_decode("000102030405060708090a0b0c");
        let info = hex_decode("f0f1f2f3f4f5f6f7f8f9");
        let expected_okm = hex_decode(
            "3cb25f25faacd57a90434f64d0362f2a2d2d0a90cf1a5a4c5db02d56ecc4c5bf34007208d5b887185865",
        );

        let result = derive_key(&ikm, Some(&salt), &info, 42);
        assert!(result.is_ok());
        assert_eq!(result.as_ref().map(|v| v.len()), Ok(42));
        assert_eq!(result.unwrap(), expected_okm);
    }

    /// RFC 5869 Test Case 2: Longer inputs/outputs.
    #[test]
    fn test_hkdf_rfc5869_case2() {
        let ikm = hex_decode(
            "000102030405060708090a0b0c0d0e0f101112131415161718191a1b1c1d1e1f\
             202122232425262728292a2b2c2d2e2f303132333435363738393a3b3c3d3e3f\
             404142434445464748494a4b4c4d4e4f",
        );
        let salt = hex_decode(
            "606162636465666768696a6b6c6d6e6f707172737475767778797a7b7c7d7e7f\
             808182838485868788898a8b8c8d8e8f909192939495969798999a9b9c9d9e9f\
             a0a1a2a3a4a5a6a7a8a9aaabacadaeaf",
        );
        let info = hex_decode(
            "b0b1b2b3b4b5b6b7b8b9babbbcbdbebfc0c1c2c3c4c5c6c7c8c9cacbcccdcecf\
             d0d1d2d3d4d5d6d7d8d9dadbdcdddedfe0e1e2e3e4e5e6e7e8e9eaebecedeeef\
             f0f1f2f3f4f5f6f7f8f9fafbfcfdfeff",
        );
        let expected_okm = hex_decode(
            "b11e398dc80327a1c8e7f78c596a49344f012eda2d4efad8a050cc4c19afa97c\
             59045a99cac7827271cb41c65e590e09da3275600c2f09b8367793a9aca3db71\
             cc30c58179ec3e87c14c01d5c1f3434f1d87",
        );

        let result = derive_key(&ikm, Some(&salt), &info, 82);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), expected_okm);
    }

    /// RFC 5869 Test Case 3: Zero-length salt (uses default).
    #[test]
    fn test_hkdf_rfc5869_case3() {
        let ikm = hex_decode("0b0b0b0b0b0b0b0b0b0b0b0b0b0b0b0b0b0b0b0b0b0b");
        let info: Vec<u8> = vec![];
        let expected_okm = hex_decode(
            "8da4e775a563c18f715f802a063c5a31b8a11f5c5ee1879ec3454e5f3c738d2d\
             9d201395faa4b61a96c8",
        );

        let result = derive_key(&ikm, None, &info, 42);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), expected_okm);
    }

    /// Zero length output must fail.
    #[test]
    fn test_hkdf_zero_length_fails() {
        let result = derive_key(b"key", Some(b"salt"), b"info", 0);
        assert!(result.is_err());
    }

    /// Output length exceeding maximum must fail.
    #[test]
    fn test_hkdf_exceeds_max_length_fails() {
        let result = derive_key(b"key", Some(b"salt"), b"info", HKDF_MAX_OUTPUT_LEN + 1);
        assert!(result.is_err());
    }

    /// Same inputs must always produce the same output (deterministic).
    #[test]
    fn test_hkdf_deterministic() {
        let r1 = derive_key(b"ikm", Some(b"salt"), b"info", 32).unwrap();
        let r2 = derive_key(b"ikm", Some(b"salt"), b"info", 32).unwrap();
        assert_eq!(r1, r2);
    }

    /// Different info values must produce different outputs.
    #[test]
    fn test_hkdf_different_info() {
        let r1 = derive_key(b"ikm", Some(b"salt"), b"info_a", 32).unwrap();
        let r2 = derive_key(b"ikm", Some(b"salt"), b"info_b", 32).unwrap();
        assert_ne!(r1, r2);
    }

    /// Helper: decode hex string to bytes.
    fn hex_decode(hex: &str) -> Vec<u8> {
        (0..hex.len())
            .step_by(2)
            .map(|i| u8::from_str_radix(&hex[i..i + 2], 16).unwrap())
            .collect()
    }
}
