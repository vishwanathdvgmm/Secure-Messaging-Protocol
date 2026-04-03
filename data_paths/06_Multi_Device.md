# Multi-Device Architecture

This document defines how SMP supports multiple devices per user while maintaining strong security guarantees.

Each device is treated as an independent cryptographic entity bound to a single identity.

---

## 1. Identity vs Device Model

SMP separates:

- **Identity (User)**
- **Devices (Endpoints)**

---

### Structure

```text
Identity (Root)
├── Device A (Phone)
├── Device B (Laptop)
├── Device C (Tablet)
```

---

## 2. Device Key Model

Each device generates its own key set:

```rust
DeviceKeyPair {
    DeviceIdentityKey
    DeviceSigningKey
    DeviceEncryptionKey
}
```

---

### Binding to Identity

Device keys are signed by the identity key:

```rust
DeviceSignature = Sign(IK_priv, DevicePublicKey)
```

### Guarantee

- Device is cryptographically linked to identity
- Unauthorized devices cannot impersonate user

---

## 3. Device Registry Model

The identity registry stores:

```rust
IdentityObject {
    IK_pub
    DeviceList [
        { DeviceID, DevicePublicKey, Signature(IK) }
    ]
}
```

### Properties

- Multiple active devices supported
- Devices can be independently managed
- Registry reflects current valid devices

---

## 4. Message Encryption (Per Device)

Messages must be encrypted separately for each device:

```text
For each Device_i:
    derive SessionKey_i
    encrypt Payload_i
```

### Packet Structure

```text
EncryptedPayloadList {
    DeviceID_1 → Ciphertext_1
    DeviceID_2 → Ciphertext_2
}
```

### Implications

- Device compromise does not expose other devices
- Fine-grained revocation possible

---

## 5. Device Registration Flow

### Step 1 — New Device Setup

- Device generates key pairs
- Requests registration

### Step 2 — Identity Authorization

- Identity signs device public key

### Step 3 — Registry Update

- Device added to DeviceList
- Published via identity registry

### Security Requirement

- Device registration must require strong authentication (existing device or recovery key)

---

## 6. Device Revocation

If a device is lost or compromised:

```rust
DeviceRevocationNotice {
    DeviceID
    Signature(IK_priv)
}
```

### Effects

- Device removed from registry
- Future messages not encrypted to that device

### Important

- Past messages remain secure (forward secrecy)
- No retroactive exposure

---

## 7. Session Model (Per Device)

Each device pair maintains independent sessions:

```text
Sender Device ↔ Recipient Device_i
```

### Implication

- Separate Double Ratchet per device pair
- No shared session state

---

## 8. Cross-Device Synchronization

Devices must synchronize state securely.

### Sync Channel

```text
Device A ↔ Device B (same identity)
```

Encrypted using device-level session.

### Sync Data

- Read/unread status
- Message deletion
- Local metadata

### Important

- Message content does NOT need re-encryption
- Only state is synchronized

---

## 9. Trust Model Interaction

Trust is:

- **Identity-based**, not device-based

### However

System may track:

- Device-level anomalies
- Suspicious device behavior

---

## 10. Security Properties

The multi-device model ensures:

- Device isolation
- Compromise containment
- Secure device onboarding
- Safe device revocation
- Independent session security

---

## 11. Failure Scenarios

### Lost Device

- Revoke device
- Continue using identity

### Compromised Device

- Limited to that device only
- No cross-device compromise

### Registry Delay

- Clients must validate latest device list
- Cache invalidation required

---

## 12. Trade-offs

| **Property** | **Impact** |
| ------------ | ---------- |
| Security     | High       |
| Message size | Increased  |
| Complexity   | Increased  |
| Privacy      | Preserved  |

---

## 13. Summary

SMP treats each device as an independent cryptographic endpoint.

This enables:

- Strong isolation
- Safe revocation
- Secure multi-device support

Per-device encryption is mandatory to preserve security guarantees.

---
