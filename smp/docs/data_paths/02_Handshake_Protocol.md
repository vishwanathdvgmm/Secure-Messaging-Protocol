# Handshake Protocol

This document defines how two parties establish secure communication in SMP.

SMP uses a **hybrid handshake model**:

- Asynchronous handshake (default)
- Optional session upgrade (real-time)

---

## 1. Handshake Overview

```text
Asynchronous (Pre-Key)
        ↓
Secure Message Exchange
        ↓
(Optional) Session Upgrade
        ↓
Double Ratchet Session
```

---

## 2. Asynchronous Handshake (Default)

This is the primary communication mode.

It allows secure messaging without both parties being online simultaneously.

---

## 3. Pre-Key Bundle Structure

Each recipient publishes:

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

---

## 4. Sender → Recipient Flow (Alice → Bob)

### Step 1 — Identity Fetch

Alice retrieves:

- Bob’s IdentityObject
- PreKeyBundle

### Step 2 — Verification

Alice verifies:

- IK signature
- Domain binding
- Revocation status
- Pre-key validity

### Step 3 — Ephemeral Key Generation

```text
Eph_A = generate_ephemeral_keypair()
```

### Step 4 — Shared Secret Derivation

```text
SharedSecret = ECDH(Eph_A_priv, Bob_OneTimePreKey_pub)
```

### Step 5 — Session Key Derivation

```text
SessionKey = HKDF(SharedSecret, context)
```

### Step 6 — Payload Encryption

```text
Ciphertext = AEAD_Encrypt(SessionKey, Payload)
```

### Step 7 — Packet Construction

```rust
SMP_Message {
   Version
   SenderIdentityHash
   RecipientIdentityHash
   Eph_A_pub
   BundleID
   FrictionBlock
   Ciphertext
   Signature(SK_priv)
}
```

### Step 8 — Transmission

Alice sends message to relay.

---

## 5. Recipient Processing (Bob)

### Step 1 — Signature Verification

- Verify sender signature using SK_pub

### Step 2 — Replay Protection

Check:

- BundleID uniqueness
- Timestamp validity

### Step 3 — Shared Secret Reconstruction

```text
SharedSecret = ECDH(Bob_OneTimePreKey_priv, Eph_A_pub)
```

### Step 4 — Session Key Derivation

```text
SessionKey = HKDF(SharedSecret)
```

### Step 5 — Decryption

- Decrypt ciphertext
- Validate AEAD integrity

### Step 6 — Pre-Key Consumption

- Mark OneTimePreKey as used
- Prevent reuse

---

## 6. Session Upgrade (Optional)

If both users are online, they may upgrade to a ratcheted session.

### Step 1 — Update Request

```rust
SessionUpgradeRequest {
   EphemeralKey
   Nonce
   Signature(SK_priv)
}
```

### Step 2 — Acceptance

```rust
SessionUpgradeAccept {
   EphemeralKey
   Signature(SK_priv)
}
```

### Step 3 — Root Key Derivation

```text
RootKey = HKDF(ECDH(EphemeralKeys))
```

### Step 4 — Start Double Ratchet

- Initialize session state
- Begin key evolution per message

---

## 7. Session Termination

Session ends when:

- One or both clients go offline
- Timeout reached
- Explicit close signal

### Termination Packet

```rust
SessionClose {
   FinalStateHash
   Signature(SK_priv)
}
```

### After Termination

- Session state destroyed
- Future messages revert to asynchronous mode

---

## 8. Security Properties

The handshake protocol ensures:

- End-to-end confidentiality
- Forward secrecy (via pre-keys)
- Replay protection (BundleID + timestamp)
- Authenticity (signature verification)

---

## 9. Failure Conditions

Handshake must fail if:

- Identity verification fails
- Signature invalid
- Pre-key expired or reused
- Decryption fails
- Replay detected

---

## 10. Summary

SMP uses a hybrid handshake model:

- Asynchronous pre-key handshake for default messaging
- Optional session upgrade for real-time communication

This design enables:

- Offline communication
- Strong cryptographic guarantees
- Seamless transition to real-time secure sessions

---
