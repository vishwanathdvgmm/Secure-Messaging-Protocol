# State Management

This document defines how the SMP client manages local state.

State management is critical for maintaining:

- Session continuity
- Trust decisions
- Identity integrity

---

## 1. Design Principles

- Secure local storage
- Separation of sensitive and non-sensitive data
- Deterministic state updates
- Minimal persistence of unnecessary data
- Strong consistency for cryptographic state

---

## 2. State Categories

Client state is divided into:

| Category       | Description             |
| -------------- | ----------------------- |
| Identity State | User identity and keys  |
| Session State  | Double Ratchet sessions |
| Trust State    | Local trust records     |
| Message State  | Cached messages         |
| Device State   | Known device list       |

---

## 3. Identity State

### Stored Data

```text
IdentityObject
IdentityPrivateKeys (IK_priv, SK_priv, EK_priv)
DeviceList
```

### Requirements

- Must be encrypted at rest
- Must never be exposed to external systems
- Prefer hardware-backed storage

---

## 4. Session State

Stored Per Device Pair

```rust
Session {
    RootKey
    DHKeys
    ChainKeys
    MessageCounters (Ns, Nr, PN)
    SkippedMessageKeys
}
```

### Properties

- Must persist across restarts
- Must be updated after each message
- Loss of state breaks forward secrecy

---

## 5. Trust State (Local)

Stored Data

```rust
LocalTrustRecord {
    FirstInteraction
    SuccessfulInteractions
    MarkedSpam
    MarkedImportant
    ManualWhitelist
    ManualBlacklist
}
```

### Purpose

- Personalize trust decisions
- Override global trust signals

---

## 6. Message State

### Stored Data

- Encrypted message cache
- Message metadata (local only)
- Message status (read/unread)

### Constraints

- Messages remain encrypted at rest
- No plaintext stored without protection

---

## 7. Device State

### Stored Data

```rust
KnownDevices {
    DeviceID
    DevicePublicKey
    TrustStatus
}
```

### Purpose

- Validate incoming device identities
- Detect unauthorized device changes

---

## 8. Storage Structure

Recommended layout:

```text
.smp/
├── identity/
├── sessions/
├── trust/
├── messages/
├── devices/
```

---

## 9. Persistence Rules

| **State** | **Persistence Requirement** |
| --------- | --------------------------- |
| Identity  | Permanent                   |
| Sessions  | Persistent                  |
| Trust     | Persistent                  |
| Messages  | Configurable                |
| Devices   | Persistent                  |

---

## 10. State Updates

State must be updated:

- Atomically
- Immediately after cryptographic operations
- Without partial writes

### Requirement

```text
No partial state updates allowed
```

---

## 11. Sync Model (Multi-Device)

Devices may sync:

- Message status (read/unread)
- Deletion events
- Non-sensitive metadata

### Must NOT Sync

- Private keys
- Session keys
- Raw cryptographic state

---

## 12. Backup & Recovery

### Backup Options

- Encrypted backup (user-controlled)
- Recovery key usage

### Requirements

- Backup must be encrypted
- Restore must preserve identity integrity

---

## 13. Failure Scenarios

### State Loss

- Sessions lost → fallback to async mode
- Identity loss → requires recovery key

### Corruption

- Detect via integrity checks
- Reset affected state only

---

## 14. Security Properties

State management ensures:

- Secure key storage
- Session continuity
- Trust persistence
- Controlled synchronization

---

## 15. Summary

The SMP client maintains structured local state to support:

- Secure communication
- Trust-aware behavior
- Multi-device operation

All sensitive state is protected and never exposed outside the client.

---
