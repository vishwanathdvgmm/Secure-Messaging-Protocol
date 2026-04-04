//! Identity hash and discriminator derivation.

use smp_crypto::hash::hash;
use smp_crypto::keys::IdentityPublicKey;

/// Number of hex characters for discriminator
pub const DISCRIMINATOR_HEX_LEN: usize = 6;

/// Generates the canonical Identity Hash array directly from the IK_pub.
pub fn generate_identity_hash(ik_pub: &IdentityPublicKey) -> [u8; 32] {
    let sha_hash = hash(ik_pub.as_bytes());
    sha_hash.into_bytes()
}

/// Derives the 6-character hex discriminator from the identity root key.
pub fn generate_discriminator(ik_pub: &IdentityPublicKey) -> String {
    let hash_obj = hash(ik_pub.as_bytes());
    let hex_full = hash_obj.to_hex();
    // take the first 6 characters
    hex_full[..DISCRIMINATOR_HEX_LEN].to_string()
}

#[cfg(test)]
mod tests {
    use super::*;
    use smp_crypto::keys::generate_identity_key_pair;

    #[test]
    fn test_discriminator_length() {
        let kp = generate_identity_key_pair();
        let discrim = generate_discriminator(&kp.public);
        assert_eq!(discrim.len(), DISCRIMINATOR_HEX_LEN);
    }

    #[test]
    fn test_hash_consistency() {
        let kp = generate_identity_key_pair();
        let h1 = generate_identity_hash(&kp.public);
        let h2 = generate_identity_hash(&kp.public);
        assert_eq!(h1, h2);

        let d1 = generate_discriminator(&kp.public);
        let d2 = generate_discriminator(&kp.public);
        assert_eq!(d1, d2);

        // Discriminator must match start of hash converted to hex
        let manual_hex = hex::encode(h1);
        assert_eq!(d1, &manual_hex[..6]);
    }
}
