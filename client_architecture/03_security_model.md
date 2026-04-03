# Client Security Model

This document defines the security model enforced by the SMP client.

The client is the **primary trust boundary** responsible for enforcing all security guarantees.

---

## 1. Security Principles

- Zero trust in external systems
- Client-side enforcement of all security checks
- Protection of private keys at all times
- Deterministic verification of all inputs
- Fail-safe behavior on any inconsistency

---

## 2. Trust Boundary

### Trusted Component

- Client (local device)

### Untrusted Components

- Relay servers
- Network infrastructure
- Storage systems
- Other clients (until verified)

---

## 3. Threat Model Alignment

The client must defend against:

- Identity spoofing
- Message tampering
- Replay attacks
- Key compromise (partial)
- Malicious relays
- Metadata inference (limited)

---

## 4. Key Protection Strategy

### Keys to Protect

- Identity Key (IK_priv)
- Signing Key (SK_priv)
- Encryption Key (EK_priv)
- Session keys

### Requirements

- Encrypted at rest
- Never transmitted externally
- Loaded only when required
- Cleared from memory when possible

### Recommended

- Hardware-backed storage (Secure Enclave / TPM)

---

## 5. Identity Verification

For every interaction, client must:

- Verify identity signature (IK)
- Verify key consistency
- Check revocation status
- Detect identity changes

### On Mismatch

```text
Mark as suspicious
Require re-verification
```

---

## 6. Message Verification

### Before accepting a message

1. Verify sender signature (SK)
2. Validate identity
3. Check replay conditions
4. Decrypt using correct session

### Failure Handling

- Reject message
- Do not display to user

---

## 7. Session Security

### Requirements

- Double Ratchet must be enforced
- Session state must be protected
- Keys must evolve per message

### On Desynchronization

```text
Reset session
Fallback to async mode
```

---

## 8. Replay Protection

### Client must track

- Message counters
- BundleIDs
- Recent timestamps

### Reject if

- Duplicate message detected
- Timestamp invalid
- Key reuse detected

---

## 9. Trust Enforcement

### Client must

- Evaluate trust before displaying messages
- Route messages based on trust level
- Allow user override

### Rule

```text
Untrusted → Message Request Queue
```

---

## 10. Device Security

### Client must

- Verify device list from registry
- Detect unknown devices
- Warn user on device changes

### On Unauthorized Device

```text
Flag identity as suspicious
```

---

## 11. Network Security

- All communication over TLS
- Certificate validation required
- No fallback to insecure channels

---

## 12. Storage Security

- All sensitive data encrypted at rest
- Integrity checks for stored data
- Secure deletion when required

---

## 13. UI Security Signals

Client must clearly indicate

- Trust level (QR verified, accepted, unknown)
- Identity changes
- Suspicious activity

---

## 14. Failure Handling

### General Rule

```text
On any failure → reject and alert
```

### Examples

- Invalid signature → reject
- Decryption failure → discard
- Identity mismatch → warn user

---

## 15. Security Properties

The client enforces:

- End-to-end confidentiality
- Authenticity of messages
- Forward secrecy
- Post-compromise recovery
  Trust-controlled communication

---

## 16. Summary

The SMP client is the **core security enforcement layer**.

All cryptographic guarantees and trust decisions are implemented at the client level, ensuring that no external system can compromise security.

---
