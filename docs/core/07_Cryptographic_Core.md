# Cryptographic Core

This document defines the core cryptographic mechanisms used in SMP to provide forward secrecy, post-compromise security, and continuous key evolution.

The system is based on a **Double Ratchet architecture**, combined with an asynchronous pre-key handshake.

---

## 1. Architecture Overview

SMP uses a hybrid model:

```
Asynchronous Handshake (Pre-Key)
            ↓
Session Initialization
            ↓
Double Ratchet (Session Mode)
```

---

## 2. Session Model

Sessions are established between **devices**, not identities.

Device A ↔ Device B

---

### Key Principle

- Each device pair maintains an independent session
- No shared session across devices
- Enables compromise isolation

---

## 3. Session State Structure

Each session maintains the following state:

```rust
pub struct DoubleRatchet {
    root_key: [u8; 32],

    dh_self: StaticSecret,
    dh_self_pub: PublicKey,
    dh_remote: Option<PublicKey>,

    chain_key_send: Option<[u8; 32]>,
    chain_key_recv: Option<[u8; 32]>,

    ns: u32, // messages sent
    nr: u32, // messages received
    pn: u32, // previous chain length
}
```

---

### Additional State

- Skipped message keys (bounded storage)
- Session identifiers
- Last activity timestamp

---

## 4. Key Derivation Functions

Two separate KDFs must be used:

```rust
KDF_RK(root_key, dh_output)
KDF_CK(chain_key)
id="r8g3tn"
```

---

### Implementation

- HKDF-SHA256
- Distinct context labels for separation

---

## 5. Initial Session Setup (Pre-Key Handshake)

### Step 1 — Shared Secret

```text
SharedSecret = ECDH(Ephemeral_A, OneTimePreKey_B)
```

### Step 2 — Root Key Initialization

```text
RK = HKDF(SharedSecret)
```

### Step 3 — Initialize Ratchet

```text
DHs = generate new key
DHr = recipient initial key

RK, CKs = KDF_RK(RK, DH(DHs, DHr))
CKr = None
```

---

## 6. Sending Messages

For each outgoing message:

```text
CKs, MK = KDF_CK(CKs)
Ns += 1
```

- `MK` → Message Key
- Encrypt payload using `MK`

**Message Header Includes**

- Message number (`Ns`)
- DH public key (if ratchet step occurred)

## 7. Receiving Messages

### 7.1 DH Ratchet Detection

If new DH public key received:

```text
PN = Ns
Ns = 0
Nr = 0

DHr = received key
RK, CKr = KDF_RK(RK, DH(DHs, DHr))

Generate new DHs
RK, CKs = KDF_RK(RK, DH(DHs, DHr))
```

### 7.2 Message Key Derivation

```text
CKr, MK = KDF_CK(CKr)
Nr += 1
```

### 7.3 Decryption

- Decrypt ciphertext using MK
- Authenticate via AEAD

---

## 8. Skipped Message Handling

Messages may arrive out of order.

System must:

- Store skipped message keys (bounded map)
- Use stored keys to decrypt delayed messages
- Prevent unbounded memory growth

---

## 9. Session Persistence

Session state must be persisted:

```text
.smp/sessions/<device_pair_id>.json
```

### Requirement

Without persistence:

- Forward secrecy breaks
- Post-compromise security fails

---

## 10. Session Lifecycle

### Start

- Triggered by message send or session upgrade

### Active

- Continuous ratcheting
- Key evolution per message

### End

- Session destroyed when:
    - Devices go offline
    - Timeout reached
    - Explicit close signal

### After End

- Future messages revert to asynchronous mode

---

## 11. Security Guarantees

### Forward Secrecy

- Old message keys cannot be recovered from current keys

### Post-Compromise Security

- System recovers after compromise
- New keys derived from fresh DH inputs

### Break-in Recovery

- Attacker loses access after ratchet step

---

## 12. Replay Protection

Each message includes:

- Message number
- DH public key
- Unique identifiers

Recipients must reject:

- Duplicate message numbers
- Reused keys

---

## 13. Multi-Device Implications

- Each device pair has independent ratchet
- No shared session state
- Compromise isolation guaranteed

---

## 14. Implementation Requirements

To correctly implement this system:

- Persistent session state
- Strict KDF separation
- Accurate message counters (Ns / Nr)
- Secure key storage
- Bounded skipped key storage

---

## 15. Summary

The SMP cryptographic core combines:

- Pre-key based asynchronous handshake
- Double Ratchet session protocol
- Continuous key evolution

This provides:

- Forward secrecy
- Post-compromise security
- Strong protection against interception and replay

The cryptographic core is the foundation of SMP’s security guarantees.

---
