# Message Format

This document defines the binary structure of SMP messages.

All SMP messages are encoded as **compact binary frames** with strict field ordering and length definitions.

---

## 1. Design Principles

- Fixed-length fields where possible
- Length-prefixed variable fields
- Network byte order (big-endian)
- No self-describing tags
- Version-first structure

---

## 2. Message Frame Layout

```text
+--------------------------------------------------+
| Version (1 byte)                                 |
| Flags (1 byte)                                   |
| HeaderLength (2 bytes)                           |
+--------------------------------------------------+
| SenderIdentityHash (32 bytes)                    |
| RecipientIdentityHash (32 bytes)                 |
| EphemeralPublicKey (32 bytes)                    |
| BundleID (8 bytes)                               |
| Timestamp (8 bytes)                              |
+--------------------------------------------------+
| FrictionLength (2 bytes)                         |
| FrictionBlock (variable)                         |
+--------------------------------------------------+
| CiphertextLength (4 bytes)                       |
| Ciphertext (variable)                            |
+--------------------------------------------------+
| Signature (64 bytes)                             |
+--------------------------------------------------+
```

---

## 3. Field Definitions

### 3.1 Version (1 byte)

```text
0x01 → SMP v1
```

### 3.2 Flags (1 byte)

Bitmask:

```text
0x01 → Asynchronous mode
0x02 → Session upgrade
0x04 → Legacy (SMTP-origin)
0x08 → High priority
```

### 3.3 HeaderLength (2 bytes)

- Total size of header fields
- Allows forward compatibility

### 3.4 Identity Hashes (32 bytes each)

```text
SenderIdentityHash   = SHA-256(IK_pub)
RecipientIdentityHash = SHA-256(IK_pub)
```

### Purpose

- Avoid exposing full identity.
- Fixed-size routing identifier.

### 3.5 EphemeralPublicKey (32 bytes)

- X25519 public key
- Used for key agreement
- Mandatory in async mode

### 3.6 BundleID (8 bytes)

- Identifier for pre-key usage
- Prevents replay attacks

### 3.7 Timestamp (8 bytes)

- Unix time (milliseconds)
- Used for:
    - Replay detection
    - Expiry validation

---

## 4. Friction Block

### Structure

```
+-----------------------------------+
| FrictionType (1 byte)             |
| DifficultyOrStake (4 bytes)       |
| NonceLength (1 byte)              |
| Nonce (variable)                  |
+-----------------------------------+
```

### Types

```text
0x01 → Proof-of-Work
0x02 → Stake
0x03 → Trusted (no friction)
```

---

## 5. Ciphertext Block

### Encryption

```text
AEAD: ChaCha20-Poly1305
```

### Associated Data (AAD)

```text
All header fields (Version → FrictionBlock)
```

### Payload Contains

```rust
EncryptedPayload {
   Subject
   Body
   Attachments
   Optional session metadata
}
```

---

## 6. Signature

```text
Signature = Ed25519(SigningKey, hash(all previous bytes))
```

### Guarantees

- Full message integrity
- Sender authenticity
- Prevents tampering

---

## 7. Encoding Rules

- Big-endian byte order
- No padding between fields
- Length-prefixed variable sections
- Strict field order must be maintained

---

## 8. Replay Protection Requirements

Recipient must track:

- Used BundleIDs
- Recent ephemeral keys
- Timestamp windows

Reject if:

- BundleID reused
- Timestamp outside allowed window
- Duplicate ephemeral key detected

---

## 9. Validation Rules

A message must be rejected if:

- Version unsupported
- Field lengths invalid
- Signature invalid
- Ciphertext decryption fails
- Replay detected

---

## 10. Extensibility

Future versions may:

- Add new flags
- Extend header fields
- Introduce new friction types

Backward compatibility ensured via:

```text
Version + HeaderLength
```

---

## 11. Summary

The SMP message format is:

- Compact and deterministic
- Cryptographically secure
- Forward-compatible
- Efficient for network transmission

This structure enables reliable and secure message exchange across all SMP implementations.

---
