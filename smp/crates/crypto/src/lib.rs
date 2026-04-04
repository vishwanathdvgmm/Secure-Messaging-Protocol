//! # SMP Crypto Module
//!
//! Cryptographic primitives for the Secure Messaging Protocol.
//!
//! This module is responsible for all:
//! - Encryption / Decryption (ChaCha20-Poly1305 AEAD)
//! - Signing / Verification (Ed25519)
//! - Key Exchange (X25519 Diffie-Hellman)
//! - Key Derivation (HKDF-SHA256)
//! - Hashing (SHA-256)
//!
//! ## Design Principles
//! - Minimal trusted code surface
//! - Strict separation from business logic
//! - No custom cryptographic primitives
//! - Uses only well-established algorithms
//!
//! ## Separation Rules
//! This module MUST NOT:
//! - Access network
//! - Access storage
//! - Contain business logic

pub mod aead;
pub mod error;
pub mod hash;
pub mod hkdf;
pub mod keys;
pub mod signing;

pub use error::CryptoError;
