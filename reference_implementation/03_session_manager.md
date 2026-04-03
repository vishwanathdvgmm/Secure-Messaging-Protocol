# Session Manager

This document defines how SMP implements secure sessions using a Double Ratchet mechanism.

The Session Manager is responsible for:

- Establishing sessions
- Evolving keys per message
- Ensuring forward secrecy
- Handling out-of-order messages

---

## 1. Design Principles

- Forward secrecy
- Post-compromise security
- Deterministic key evolution
- Independent send/receive chains

---

## 2. Session Structure

```go id="sess-struct"
type Session struct {
    RootKey        []byte
    SendChainKey   []byte
    RecvChainKey   []byte

    DHPrivateKey   []byte
    DHPublicKey    []byte
    RemoteDHPublic []byte

    Ns uint32 // sent messages
    Nr uint32 // received messages
    PN uint32 // previous chain length

    SkippedKeys map[string][]byte
}
```

---

## 3. Session Initialization

### Case 1 — Pre-Key (Async Start)

Use recipient Pre-Key Bundle

Perform DH exchanges:

```text
DH1 = DH(EK_priv, IK_pub)
DH2 = DH(IK_priv, EK_pub)
DH3 = DH(EK_priv, EK_pub)
```

### Root Key Derivation

```text
RootKey = HKDF(DH1 || DH2 || DH3)
```

---

## 4. Sending Messages

### Step 1 — Derive Message Key

```text
MessageKey = HKDF(SendChainKey)
SendChainKey = HKDF(SendChainKey)
```

### Step 2 — Encrypt

- Use AEAD with MessageKey
- Increment Ns

---

## 5. Receiving Messages

### Step 1 — Check for Skipped Keys

- If exists → use stored key

### Step 2 — If New DH Key

```text
RootKey, RecvChainKey = HKDF(DH(DHPrivateKey, RemoteDHPublic))
Generate new DH key pair
```

### Step 3 — Derive Message Key

```text
MessageKey = HKDF(RecvChainKey)
RecvChainKey = HKDF(RecvChainKey)
```

### Step 4 — Decrypt

- Use AEAD
- Increment Nr

---

## 6. Skipped Message Handling

Store skipped keys:

```go
SkippedKeys[message_id] = MessageKey
```

### Limits

- Max skipped keys must be bounded
- Old entries must be removed

---

## 7. DH Ratchet Trigger

Occurs when:

- New remote DH public key detected

### Effect

- RootKey updated
- Chain keys reset
- New forward secrecy window

---

## 8. Session Persistence

Session must be saved:

- After every message
- Atomically

### Stored Fields

- RootKey
- ChainKeys
- Counters
- DH keys

---

## 9. Session Reset

Triggered when:

- Decryption repeatedly fails
- State mismatch detected

### Behavior

```text
Delete session
Fallback to Pre-Key handshake
```

---

## 10. Multi-Device Sessions

Each device pair has:

```text
Separate Session
```

No shared session across devices

---

## 11. Security Constraints

- Keys must never be reused
- Chain keys must evolve per message
- Root key must be updated on DH ratchet

---

## 12. Error Handling

### Decryption Failure

- Attempt skipped keys
- If still fails → reject

### Out-of-Order Messages

- Store skipped keys
- Process when possible

---

## 13. Memory Safety

- Message keys deleted after use
- Skipped keys periodically cleaned

---

## 14. Security Properties

Session Manager ensures:

- Forward secrecy
- Post-compromise recovery
- Replay resistance
- Secure asynchronous messaging

---

## 15. Summary

The Session Manager implements:

- Double Ratchet key evolution
- Secure message encryption/decryption
- Robust session handling across devices

It is the core mechanism enabling secure communication in SMP.

---
