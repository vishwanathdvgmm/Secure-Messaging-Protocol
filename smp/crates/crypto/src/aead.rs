//! ChaCha20-Poly1305 AEAD encryption and decryption.
//!
//! Implements authenticated encryption with associated data (AEAD)
//! using ChaCha20-Poly1305 as specified in the crypto module docs.
//!
//! ## AEAD Rules (from spec)
//! - Associated Data (AAD) MUST include: message header, sender identity hash,
//!   recipient identity hash
//! - Nonce MUST be unique per key (never reused)
//! - Message keys MUST be single-use
//! - Authentication failure → hard error (no partial plaintext)
//!
//! ## Nonce Management
//! - 96-bit (12-byte) random nonce generated via OsRng
//! - Nonce is prepended to ciphertext for transport

use chacha20poly1305::{
    aead::{Aead, KeyInit, Payload},
    ChaCha20Poly1305, Nonce as ChaCha20Nonce,
};
use rand_core::{OsRng, RngCore};

use crate::error::CryptoError;

/// ChaCha20-Poly1305 key length in bytes.
pub const AEAD_KEY_LEN: usize = 32;

/// ChaCha20-Poly1305 nonce length in bytes (96 bits).
pub const AEAD_NONCE_LEN: usize = 12;

/// Poly1305 authentication tag length in bytes.
pub const AEAD_TAG_LEN: usize = 16;

/// Generates a cryptographically secure random nonce (96-bit).
///
/// Uses `OsRng` — the only approved randomness source.
///
/// # Returns
/// A 12-byte random nonce, or error if RNG fails.
pub fn generate_nonce() -> Result<[u8; AEAD_NONCE_LEN], CryptoError> {
    let mut nonce = [0u8; AEAD_NONCE_LEN];
    OsRng
        .try_fill_bytes(&mut nonce)
        .map_err(|e| CryptoError::RngFailed(format!("nonce generation failed: {e}")))?;
    Ok(nonce)
}

/// Encrypts plaintext using ChaCha20-Poly1305 AEAD.
///
/// The nonce is prepended to the output: `[nonce (12 bytes) || ciphertext || tag (16 bytes)]`.
/// This ensures the nonce is always available for decryption.
///
/// # Arguments
/// * `key` - 32-byte encryption key. MUST be single-use.
/// * `plaintext` - The data to encrypt.
/// * `aad` - Associated Authenticated Data. Authenticated but not encrypted.
///           MUST include message header, sender hash, recipient hash.
///
/// # Returns
/// `nonce || ciphertext || tag` as a single `Vec<u8>`.
///
/// # Errors
/// - `CryptoError::InvalidKey` if key length is not 32 bytes.
/// - `CryptoError::EncryptionFailed` if AEAD encryption fails.
/// - `CryptoError::RngFailed` if nonce generation fails.
pub fn encrypt(key: &[u8], plaintext: &[u8], aad: &[u8]) -> Result<Vec<u8>, CryptoError> {
    validate_key_length(key)?;

    let nonce_bytes = generate_nonce()?;
    encrypt_with_nonce(key, &nonce_bytes, plaintext, aad)
}

/// Encrypts plaintext using a caller-provided nonce.
///
/// **WARNING**: The caller is responsible for ensuring nonce uniqueness per key.
/// Prefer [encrypt] which auto-generates nonces.
///
/// # Arguments
/// * `key` - 32-byte encryption key.
/// * `nonce` - 12-byte nonce. MUST be unique for this key.
/// * `plaintext` - The data to encrypt.
/// * `aad` - Associated Authenticated Data.
///
/// # Returns
/// `nonce || ciphertext || tag` as a single `Vec<u8>`.
pub fn encrypt_with_nonce(
    key: &[u8],
    nonce: &[u8],
    plaintext: &[u8],
    aad: &[u8],
) -> Result<Vec<u8>, CryptoError> {
    validate_key_length(key)?;
    validate_nonce_length(nonce)?;

    let cipher = ChaCha20Poly1305::new_from_slice(key)
        .map_err(|e| CryptoError::InvalidKey(format!("key initialization failed: {e}")))?;

    let chacha_nonce = ChaCha20Nonce::from_slice(nonce);

    let payload = Payload {
        msg: plaintext,
        aad,
    };

    let ciphertext = cipher
        .encrypt(chacha_nonce, payload)
        .map_err(|e| CryptoError::EncryptionFailed(format!("{e}")))?;

    // Prepend nonce to ciphertext: [nonce || ciphertext || tag]
    let mut output = Vec::with_capacity(AEAD_NONCE_LEN + ciphertext.len());
    output.extend_from_slice(nonce);
    output.extend_from_slice(&ciphertext);

    Ok(output)
}

/// Decrypts ciphertext using ChaCha20-Poly1305 AEAD.
///
/// Expects input format: `[nonce (12 bytes) || ciphertext || tag (16 bytes)]`.
/// The nonce is extracted from the first 12 bytes.
///
/// # Arguments
/// * `key` - 32-byte decryption key. Must match the encryption key.
/// * `ciphertext_with_nonce` - The encrypted data with prepended nonce.
/// * `aad` - Associated Authenticated Data. Must match what was used during encryption.
///
/// # Returns
/// The decrypted plaintext.
///
/// # Errors
/// - `CryptoError::InvalidKey` if key length is not 32 bytes.
/// - `CryptoError::DecryptionFailed` if authentication fails or data is corrupted.
///   **No partial plaintext is ever returned.**
pub fn decrypt(
    key: &[u8],
    ciphertext_with_nonce: &[u8],
    aad: &[u8],
) -> Result<Vec<u8>, CryptoError> {
    validate_key_length(key)?;

    // Must have at least nonce + tag
    let min_len = AEAD_NONCE_LEN + AEAD_TAG_LEN;
    if ciphertext_with_nonce.len() < min_len {
        return Err(CryptoError::DecryptionFailed);
    }

    let (nonce, ciphertext) = ciphertext_with_nonce.split_at(AEAD_NONCE_LEN);
    decrypt_with_nonce(key, nonce, ciphertext, aad)
}

/// Decrypts ciphertext using a caller-provided nonce.
///
/// # Arguments
/// * `key` - 32-byte decryption key.
/// * `nonce` - 12-byte nonce used during encryption.
/// * `ciphertext` - The encrypted data (without nonce prefix, includes tag).
/// * `aad` - Associated Authenticated Data.
///
/// # Returns
/// The decrypted plaintext.
///
/// # Errors
/// - `CryptoError::DecryptionFailed` if authentication fails.
///   **No partial plaintext is ever returned.**
pub fn decrypt_with_nonce(
    key: &[u8],
    nonce: &[u8],
    ciphertext: &[u8],
    aad: &[u8],
) -> Result<Vec<u8>, CryptoError> {
    validate_key_length(key)?;
    validate_nonce_length(nonce)?;

    let cipher = ChaCha20Poly1305::new_from_slice(key)
        .map_err(|e| CryptoError::InvalidKey(format!("key initialization failed: {e}")))?;

    let chacha_nonce = ChaCha20Nonce::from_slice(nonce);

    let payload = Payload {
        msg: ciphertext,
        aad,
    };

    let plaintext = cipher
        .decrypt(chacha_nonce, payload)
        .map_err(|_| CryptoError::DecryptionFailed)?;

    // Return plaintext — caller is responsible for zeroizing after use
    Ok(plaintext)
}

/// Validates that the key is exactly 32 bytes.
fn validate_key_length(key: &[u8]) -> Result<(), CryptoError> {
    if key.len() != AEAD_KEY_LEN {
        return Err(CryptoError::InvalidKey(format!(
            "AEAD key must be {AEAD_KEY_LEN} bytes, got {}",
            key.len()
        )));
    }
    Ok(())
}

/// Validates that the nonce is exactly 12 bytes.
fn validate_nonce_length(nonce: &[u8]) -> Result<(), CryptoError> {
    if nonce.len() != AEAD_NONCE_LEN {
        return Err(CryptoError::InvalidNonce {
            expected: AEAD_NONCE_LEN,
            actual: nonce.len(),
        });
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Helper: generate a valid 32-byte key for testing.
    fn test_key() -> [u8; AEAD_KEY_LEN] {
        let mut key = [0u8; AEAD_KEY_LEN];
        OsRng.fill_bytes(&mut key);
        key
    }

    #[test]
    fn test_encrypt_decrypt_round_trip() {
        let key = test_key();
        let plaintext = b"Hello, SMP secure messaging!";
        let aad = b"header|sender_hash|recipient_hash";

        let encrypted = encrypt(&key, plaintext, aad);
        assert!(encrypted.is_ok());

        let encrypted = encrypted.unwrap();
        // Output must be nonce + plaintext + tag
        assert_eq!(
            encrypted.len(),
            AEAD_NONCE_LEN + plaintext.len() + AEAD_TAG_LEN
        );

        let decrypted = decrypt(&key, &encrypted, aad);
        assert!(decrypted.is_ok());
        assert_eq!(decrypted.unwrap(), plaintext);
    }

    #[test]
    fn test_encrypt_empty_plaintext() {
        let key = test_key();
        let aad = b"metadata";

        let encrypted = encrypt(&key, b"", aad);
        assert!(encrypted.is_ok());

        let decrypted = decrypt(&key, &encrypted.unwrap(), aad);
        assert!(decrypted.is_ok());
        assert_eq!(decrypted.unwrap(), b"");
    }

    #[test]
    fn test_decrypt_wrong_key_fails() {
        let key1 = test_key();
        let key2 = test_key();
        let plaintext = b"secret data";
        let aad = b"aad";

        let encrypted = encrypt(&key1, plaintext, aad).unwrap();
        let result = decrypt(&key2, &encrypted, aad);
        assert!(matches!(result, Err(CryptoError::DecryptionFailed)));
    }

    #[test]
    fn test_decrypt_wrong_aad_fails() {
        let key = test_key();
        let plaintext = b"secret data";

        let encrypted = encrypt(&key, plaintext, b"correct_aad").unwrap();
        let result = decrypt(&key, &encrypted, b"wrong_aad");
        assert!(matches!(result, Err(CryptoError::DecryptionFailed)));
    }

    #[test]
    fn test_decrypt_tampered_ciphertext_fails() {
        let key = test_key();
        let plaintext = b"secret data";
        let aad = b"aad";

        let mut encrypted = encrypt(&key, plaintext, aad).unwrap();
        // Tamper with a byte in the ciphertext (after the nonce)
        if encrypted.len() > AEAD_NONCE_LEN + 1 {
            encrypted[AEAD_NONCE_LEN + 1] ^= 0xFF;
        }

        let result = decrypt(&key, &encrypted, aad);
        assert!(matches!(result, Err(CryptoError::DecryptionFailed)));
    }

    #[test]
    fn test_decrypt_truncated_data_fails() {
        let key = test_key();
        // Too short to contain nonce + tag
        let result = decrypt(&key, &[0u8; 10], b"aad");
        assert!(matches!(result, Err(CryptoError::DecryptionFailed)));
    }

    #[test]
    fn test_invalid_key_length() {
        let result = encrypt(&[0u8; 16], b"data", b"aad");
        assert!(matches!(result, Err(CryptoError::InvalidKey(_))));
    }

    #[test]
    fn test_invalid_nonce_length() {
        let key = test_key();
        let result = encrypt_with_nonce(&key, &[0u8; 8], b"data", b"aad");
        assert!(matches!(result, Err(CryptoError::InvalidNonce { .. })));
    }

    #[test]
    fn test_nonce_uniqueness() {
        // Two encryptions with auto-generated nonces must produce different ciphertexts
        let key = test_key();
        let plaintext = b"same message";
        let aad = b"aad";

        let enc1 = encrypt(&key, plaintext, aad).unwrap();
        let enc2 = encrypt(&key, plaintext, aad).unwrap();

        // Ciphertexts must differ because nonces differ
        assert_ne!(enc1, enc2);

        // But both must decrypt to the same plaintext
        assert_eq!(decrypt(&key, &enc1, aad).unwrap(), plaintext);
        assert_eq!(decrypt(&key, &enc2, aad).unwrap(), plaintext);
    }

    #[test]
    fn test_explicit_nonce_encrypt_decrypt() {
        let key = test_key();
        let nonce = [1u8; AEAD_NONCE_LEN];
        let plaintext = b"explicit nonce test";
        let aad = b"metadata";

        let encrypted = encrypt_with_nonce(&key, &nonce, plaintext, aad).unwrap();
        let decrypted = decrypt(&key, &encrypted, aad).unwrap();
        assert_eq!(decrypted, plaintext);
    }

    #[test]
    fn test_large_plaintext() {
        let key = test_key();
        let plaintext = vec![0xABu8; 65536]; // 64 KB
        let aad = b"large payload test";

        let encrypted = encrypt(&key, &plaintext, aad).unwrap();
        let decrypted = decrypt(&key, &encrypted, aad).unwrap();
        assert_eq!(decrypted, plaintext);
    }

    #[test]
    fn test_empty_aad() {
        let key = test_key();
        let plaintext = b"message with no AAD";

        let encrypted = encrypt(&key, plaintext, b"").unwrap();
        let decrypted = decrypt(&key, &encrypted, b"").unwrap();
        assert_eq!(decrypted, plaintext);
    }

    #[test]
    fn test_nonce_is_prepended() {
        let key = test_key();
        let nonce = [0x42u8; AEAD_NONCE_LEN];
        let plaintext = b"check nonce position";
        let aad = b"aad";

        let encrypted = encrypt_with_nonce(&key, &nonce, plaintext, aad).unwrap();

        // First 12 bytes should be the nonce
        assert_eq!(&encrypted[..AEAD_NONCE_LEN], &nonce);
    }
}
