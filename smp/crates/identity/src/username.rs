//! Username structural representation and validation.

use crate::error::IdentityError;

/// Strong typing for the human-readable username mapping
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Username {
    pub base: String,
    pub discriminator: String,
    pub domain: String,
}

impl Username {
    /// Parses a username string in the format `base#dicriminator@domain`.
    pub fn parse(address: &str) -> Result<Self, IdentityError> {
        let at_parts: Vec<&str> = address.split('@').collect();
        if at_parts.len() != 2 {
            return Err(IdentityError::InvalidUsername(
                "must contain exactly one @ symbol".into(),
            ));
        }

        let main_part = at_parts[0];
        let domain_part = at_parts[1];

        if domain_part.is_empty() {
            return Err(IdentityError::InvalidUsername(
                "domain part cannot be empty".into(),
            ));
        }

        let hash_parts: Vec<&str> = main_part.split('#').collect();
        if hash_parts.len() != 2 {
            return Err(IdentityError::InvalidUsername(
                "must contain exactly one # symbol before the @ domain".into(),
            ));
        }

        let base = hash_parts[0];
        let discriminator = hash_parts[1];

        if base.is_empty() {
            return Err(IdentityError::InvalidUsername(
                "base name cannot be empty".into(),
            ));
        }

        if discriminator.len() != crate::hash::DISCRIMINATOR_HEX_LEN {
            return Err(IdentityError::InvalidUsername(format!(
                "discriminator must be exactly {} characters",
                crate::hash::DISCRIMINATOR_HEX_LEN
            )));
        }

        Ok(Username {
            base: base.to_string(),
            discriminator: discriminator.to_string(),
            domain: domain_part.to_string(),
        })
    }

    /// Returns the full display format: `base#discriminator@domain`
    pub fn to_display(&self) -> String {
        format!("{}#{}@{}", self.base, self.discriminator, self.domain)
    }

    /// Returns the search/lookup format: `base#discriminator`
    pub fn to_search(&self) -> String {
        format!("{}#{}", self.base, self.discriminator)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_valid_username_parse() {
        let u = Username::parse("alice#7f2a91@domain.com").unwrap();
        assert_eq!(u.base, "alice");
        assert_eq!(u.discriminator, "7f2a91");
        assert_eq!(u.domain, "domain.com");

        assert_eq!(u.to_display(), "alice#7f2a91@domain.com");
        assert_eq!(u.to_search(), "alice#7f2a91");
    }

    #[test]
    fn test_missing_domain() {
        assert!(Username::parse("alice#7f2a91").is_err());
    }

    #[test]
    fn test_missing_discriminator_sep() {
        assert!(Username::parse("alice7f2a91@domain.com").is_err());
    }

    #[test]
    fn test_wrong_discriminator_length() {
        assert!(Username::parse("alice#7f2a9@domain.com").is_err()); // 5
        assert!(Username::parse("alice#7f2a91a@domain.com").is_err()); // 7
    }

    #[test]
    fn test_empty_parts() {
        assert!(Username::parse("#7f2a91@domain.com").is_err()); // no base
        assert!(Username::parse("alice#@domain.com").is_err()); // no discriminator
        assert!(Username::parse("alice#7f2a91@").is_err()); // no domain
    }
}
