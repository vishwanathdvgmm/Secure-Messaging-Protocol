# Encryption Model

This document defines the cryptographic primitives and encryption strategy used in the Secure Messaging Protocol (SMP).

SMP enforces **end-to-end encryption by default**, ensuring that only intended recipient devices can access message content.

---

## 1. Design Goals

The encryption model must provide:

- End-to-end confidentiality
- Message integrity
- Sender authenticity
- Forward secrecy
- Post-compromise security (via session layer)
- Cryptographic agility for future upgrades

---

## 2. Cryptographic Primitives

SMP uses modern, widely accepted cryptographic algorithms.

### 2.1 Key Exchange

```text
X25519
```

- Elliptic curve Diffie-Hellman (ECDH)
- Used for deriving shared secrets

### 2.2 Digital Signatures

```text
Ed25519
```

- Used for message authentication
- Ensures sender authenticity

### 2.3 Symmetric Encryption

```text
ChaCha20-Poly1305
```

- Authenticated Encryption with Associated Data (AEAD)
- Provides confidentiality + integrity

Alternative (optional):

```text
AES-GCM
```

### 2.4 Key Derivation

```text
HKDF-SHA256
```

- Used for derive:
    - Session keys
    - Chain keys
    - Message keys

---

## 3. Encryption Model Overview

SMP uses a **hybrid encryption approach**:

1. Asymmetric key exchange → derive shared secret
2. Symmetric encryption → encrypt message payload

General Flow

```text
SharedSecret = ECDH(sender_private, recipient_public)
SessionKey   = HKDF(SharedSecret)
Ciphertext   = AEAD_Encrypt(SessionKey, Payload)
```

---

## 4. What Is Encrypted

**Fully Encrypted**

- Message body
- Subject line
- Attachments
- Session metadata

**Not Encrypted (Relay-Visible)**

- Sender identity hash
- Recipient identity hash
- Timestamp
- Message size
- Friction metadata

**Rationale**

- Relays require minimal metadata for routing
- All sensitive content remains protected

---

## 5. Associated Data (AAD)

AEAD encryption includes authenticated metadata:

```text
AAD = HeaderFields (Version → FrictionBlock)
```

**Guarantees**

- Header cannot be modified without detection
- Prevents tampering with routing or metadata

---

## 6. Asynchronous Encryption (Default Mode)

Used when no prior session exists.

**Flow**

1. Sender fetches recipient Pre-Key Bundle
2. Generates ephemeral key pair
3. Derives shared secret:

```text
SharedSecret = ECDH(Eph_private, Recipient_PreKey)
```

4. Derives session key:

```text
SessionKey = HKDF(SharedSecret, context)
```

5. Encrypts message payload

**Properties**

- One-time encryption
- Forward secrecy via pre-keys
- No session state required

---

## 7. Session Encryption (Real-Time Mode)

Used after session upgrade.

- Based on Double Ratchet (defined in Cryptographic Core)
- Continuous key evolution
- Each message uses a unique key

**Guarantees**

- Forward secrecy
- Post-compromise security
- Break-in recovery

---

## 8. Multi-Device Encryption

Messages must be encrypted per device:

```text
For each recipient device:
    derive SessionKey_i
    encrypt Payload_i
```

**Implication**

- Compromise of one device does not affect others
- Enables device-level revocation

---

## 9. Attachment Encryption

**Small Attachments**

- Encrypted inline within message payload

**Large Attachments**

- Encrypted separately:

```text
AttachmentKey = random(256-bit)
EncryptedFile = AEAD_Encrypt(AttachmentKey, File)
```

- Stored externally
- Reference included in message payload

**Properties**

- Relay cannot access file content
- Attachment integrity verified via hash

---

## 10. Replay Protection

Messages include:

- Unique BundleID
- Ephemeral keys
- Timestamp

Recipients must reject:

- Reused BundleIDs
- Duplicate ephemeral keys
- Expired timestamps

---

## 11. Cryptographic Agility

SMP is designed to support multiple cipher suites.

**Example:**

```text
SupportedSuites = {
    X25519 + ChaCha20-Poly1305,
    X25519 + AES-GCM,
    Hybrid PQ (future)
}
```

**Requirements**

- Clients advertise supported suites
- Protocol allows safe upgrades
- No hardcoded algorithms

---

## 12. Post-Quantum Readiness

SMP adopts a hybrid strategy:

- Classical cryptography (default)
- Post-quantum algorithms (future integration)

**Potential additions:**

- Kyber (key exchange)
- Dilithium (signatures)

---

## 13. Security Properties

The encryption model guarantees:

- Confidentiality — only recipients can decrypt
- Integrity — tampering is detectable
- Authenticity — sender is verifiable
- Forward secrecy — past messages remain secure
- Extensibility — supports future cryptography

---

## 14. Summary

SMP uses a hybrid encryption model combining:

- Asymmetric key exchange
- Symmetric authenticated encryption
- Strong key derivation

All message content is encrypted end-to-end by default, with minimal metadata exposure required for routing.

---
