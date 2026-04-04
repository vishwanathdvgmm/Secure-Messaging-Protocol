//! # SMP Identity Module
//!
//! Cryptographic identity generation, structuring, and validation.
//! Identifiers in SMP are fundamentally public keys, not usernames.
//! Usernames are deterministically mapped display structures.

pub mod error;
pub mod hash;
pub mod object;
pub mod username;

pub use error::IdentityError;
pub use object::{IdentityObject, SignableIdentityObject};
pub use username::Username;
