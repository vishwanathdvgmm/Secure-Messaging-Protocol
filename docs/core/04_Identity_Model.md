# Identity Model

This document defines the identity architecture of the Secure Messaging Protocol (SMP).

SMP replaces address-based identity with **cryptographic identity**, where all communication is bound to verifiable public keys rather than mutable identifiers.

---

## 1. Identity Overview

In SMP, each identity is represented using a structured, human-readable format:

```
#@
```

Example:

```
alice#7f2a91@domain.com
```

**Components**

- `alice` → user-chosen base identifier.
- `#7f2a91` → system-generated discriminator (identity-derived).
- `@domain.com` → domain namespace.

**Key Property:**

- `#` is **mandatory**.
- `@domain` is **mandatory in display format**.
- User only selects the **base name**.
- Discriminator is **automatically generated and immutable**.
- Domain is **system-controlled**.

**Idey Properties:**

The username is **not the identity**.

The true identity is:

```
Identity = IdentityPublicKey (IK)
```

The username is only a **human-readable mapping**.

**Internal vs Display Representation**

| **Purpose**     | **Format**            |
| --------------- | --------------------- |
| Display         | `alice#7f2a91@domain` |
| Search / Lookup | `alice#7f2a91`        |

---

## 2. Username Generation Model

### 2.1 Base Name

- Provided by user during registration.
- Not required to be globally unique.

### 2.2 Discriminator Generation

The discriminator is derived deterministically:

```
discriminator = first_6_hex(SHA256(IdentityPublicKey))
```

Example:

```
Hash: 7f2a914e2...
→ alice#7f2a91
```

### 2.3 Properties

- Deterministic (no randomness)
- Bound to identity
- No databse lookup required
- Prevents username squatting
- Eliminates race conditions

### 2.4 Collosion Space

- 6 hex chars = 24 bits
- 16,777,216 combinations

If scaling requires:

- Future accounts may use 8 hex
- Backward compatibility preserved.

---

## 3. Username Mutability Rules

| **Component** | **Editable**   |
| ------------- | -------------- |
| Base Name     | Yes (optional) |
| Discriminator | No             |
| Domain        | No             |

**Example**

Before:

```
alice#7f2a91@domain.com
```

After rename:

```
ally#7f2a91@domain.com
```

**Note:**

- Discriminator remains same.
- Domain remains same.
- Only base name changes.

## 4. Identity Architecture

Each identity consists of a structured set of keys with distinct roles.

### 4.1 Key Hierarchy

Each identity consists of multiple keys:

| Key | Name           | Purpose                          |
| --- | -------------- | -------------------------------- |
| IK  | Identity Key   | Root identity anchor (long-term) |
| SK  | Signing Key    | Message authentication           |
| EK  | Encryption Key | Receiving encrypted messages     |
| PK  | Pre-Keys       | Forward secrecy support          |
| RK  | Recovery Key   | Account recovery                 |

---

### 4.2 Key Separation Rationale

Key separation ensures compromise containment.

- IK compromise → identity risk (rare usage).
- SK compromise → forgery risk, identity remains recoverable.
- EK compromise → limited confidentiality impact (limited by forward secrecy).
- PK compromise → limited to unused session.

**Principle:**
No single key compromise breaks the entire identity.

---

## 5. Identity Object Structure

Each identity is represented as a signed object:

```rust
IdentityObject {
   HumanAddress
   IdentityPublicKey (IK_pub)
   SigningPublicKey (SK_pub)
   EncryptionPublicKey (EK_pub)
   KeyVersion
   DomainBindingSignature
   RevocationPointer
   CreatedTimestamp
   ExpiryTimestamp
}
```

All fields are signed by the identity key:

```rust
Signature = Sign(IK_priv, hash(all_fields))
```

Ensure:

- No silent key replacement.
- Full identity integrity.

---

## 6. Domain Binding Model

SMP supports optional domain-backed identities.

### 6.1 Domain Identity Key (DIK)

Each domain maintains:

```rust
DomainIdentityKey (DIK)
```

### 6.2 Binding Process

The domain signs the identity object:

```rust
DomainSignature = Sign(DIK_priv, hash(IdentityObject))
```

This creates a verifiable link between:

```
alice74fa5e@example.com
```

and the domain authority.

### 6.3 Domain Trust Implications

- Domain identities gain higher trust
- Domain revocation removes trust, not identity
- Supports:
    - Organizational users
    - Independent users

---

## 7. Identity Discovery Model

### 7.1 Search Format

Search requires full identifier:

```
<base>#<discriminator>
```

Example:

```
alice#7f2a91
```

### 7.2 Restrictions

- No partial search (alice ❌)
- No wildcard search ❌
- No directory browsing ❌

### 7.3 Opt-In Discovery

Default:

- Not searchable

User may enable:

- Searchable via full identifier only

### 7.4 Anti-Enumeration Protection

Required protections:

- Rate limiting
- Request throtting
- Pattern detection

## 8. Contact Establishment Model

### Unknown Sender Behavior

Unknown messages go to:

```
Message Request Queue
```

**User Actions**

- Accept → trust established
- Ignore → discard
- Block → prevent future contact

**After Acceptance**

- Identity is pinned
- Future messages go to inbox
- Session may be established

## 9. Identity Verification Signals

### When receiving unknown requests:

Display:

```
vishwanath#7f2a91
Fingerprint: 91AF 7C22
```

**Fingerprint Generation**

```
fingerprint = first_4_bytes(SHA256(IdentityPublicKey))
```

**Purpose**

- Lightweight verification
- Avoid full hash exposure
- Human-friendly validation

## 10. Identity Lifecycle

### Key Generation

- Client-side only
- Secure randomness required
- IK should be hardware-protected

### Key Rotation

```rust
NewKeyAnnouncement {
   OldKeyHash
   NewPublicKey
   Signature(IK_priv)
}
```

**Key Rotation**

| Key | Rotation    |
| --- | ----------- |
| IK  | Rare        |
| SK  | 6–12 months |
| EK  | Frequent    |

### Revocation

```rust
RevocationNotice {
   RevokedKeyHash
   ReasonCode
   Timestamp
   Signature(IK_priv)
}
```

## 11. Forward Secrecy (Pre-Key Model)

```rust
PreKeyBundle {
   IK_pub
   SK_pub
   EK_pub
   OneTimePreKey_pub
   BundleID
   Expiry
   Signature(IK_priv)
}
```

## 12. Identity Verification Flow

1. Fetch identity
2. Verify:
    - IK signature
    - Domain binding
    - Revocation
    - Expiry
3. Cache fingerprint

## 13. Identity Types

| **Type**        | **Description**  |
| --------------- | ---------------- |
| Personal        | Individual       |
| Domain-Verified | Organization     |
| Service         | Automated        |
| Pseudonymous    | Not domain-bound |
| Ephemeral       | One-time         |

## 14. Security Properties

- Authencity
- Non-repudiation
- Spoofing resistance
- Compromise containment
- Auditability

## 15. Summary

SMP identity is:

- Cryptographic (not address-based)
- Deterministic (no username conflicts)
- Privacy-preserving (no open directory)
- Trust-aware (request-based communication)

This identity layer forms the foundation for all protocol behavior.

---
