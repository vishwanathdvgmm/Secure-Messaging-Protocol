# Crypto Module

This document defines the cryptographic module used in SMP.

The crypto module is responsible for all:

- Encryption / Decryption
- Signing / Verification
- Key derivation

---

## 1. Design Principles

- Minimal trusted code surface
- Strict separation from business logic
- Deterministic outputs
- No custom cryptographic primitives
- Use well-established algorithms only

---

## 2. Supported Primitives

| Purpose              | Algorithm                        |
| -------------------- | -------------------------------- |
| Hashing              | SHA-256                          |
| Key Derivation       | HKDF (SHA-256)                   |
| Symmetric Encryption | AES-256-GCM or ChaCha20-Poly1305 |
| Asymmetric Keys      | X25519 (DH), Ed25519 (signing)   |

---

## 3. Key Types

```text
IdentityKey (IK)
SigningKey (SK)
EncryptionKey (EK)
SessionKeys
```

---

## 4. Core Interfaces

### 4.1 Hash Function

```go
func Hash(data []byte) []byte
```

### 4.2 Key Derivation (HKDF)

```go
func HKDF(inputKey, salt, info []byte, length int) []byte
```

### 4.3 Symmetric Encryption (AEAD)

```go
func Encrypt(key, nonce, plaintext, aad []byte) (ciphertext []byte)
func Decrypt(key, nonce, ciphertext, aad []byte) (plaintext []byte, err error)
```

### 4.4 Signing

```go
func Sign(privateKey, message []byte) []byte
```

### 4.5 Verification

```go
func Verify(publicKey, message, signature []byte) bool
```

### 4.6 Diffie-Hellman (X25519)

```go
func DH(privateKey, publicKey []byte) []byte
```

---

## 5. Key Generation

```go
func GenerateIdentityKey() (priv, pub []byte)
func GenerateSigningKey() (priv, pub []byte)
func GenerateEncryptionKey() (priv, pub []byte)
```

### Requirements

- Use secure randomness
- Never reuse entropy sources improperly

---

## 6. Nonce Management

### Rules

- Must be unique per encryption key
- Must never repeat

### Recommended

```text
nonce = random(96-bit)
```

---

## 7. AEAD Usage

### Associated Data (AAD)

Must include:

- Message header
- Sender identity hash
- Recipient identity hash

### Purpose

- Prevent header tampering
- Bind metadata to ciphertext

---

## 8. Error Handling

### Decryption

```text
If authentication fails → return error
```

### Rule

- Never return partial plaintext
- Never ignore authentication failure

---

## 9. Memory Safety

Sensitive data must:

- Be cleared after use
- Not be logged
- Not be copied unnecessarily

---

## 10. Randomness Requirements

Use:

- Cryptographically secure RNG only

### Forbidden

- Math random
- Predictable seeds

---

## 11. Separation Rules

Crypto module must NOT:

- Access network
- Access storage
- Contain business logic

---

## 12. Testing Requirements

Must include:

- Known-answer tests
- Randomized tests
- Edge-case validation

---

## 13. Security Properties

Crypto module ensures:

- Confidentiality
- Integrity
- Authenticity
- Forward secrecy support

---

## 14. Summary

The crypto module provides:

- Clean, minimal interfaces
- Strong cryptographic guarantees
- Safe implementation boundaries

All higher-level modules depend on this layer.

---
