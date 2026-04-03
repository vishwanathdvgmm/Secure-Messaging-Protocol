# Session Model

This document defines how communication sessions are established, maintained, and terminated in SMP.

SMP uses a **hybrid communication model**:

- Asynchronous mode (default)
- Session mode (optional, real-time)

---

## 1. Session Model Overview

```text
Default → Asynchronous Messaging
            ↓
Optional Upgrade → Session Mode
            ↓
Session Ends → Back to Asynchronous
```

---

## 2. Asynchronous Mode (Default)

This is the primary communication mode.

### Characteristics

- No persistent session state required
- Uses Pre-Key handshake per message
- Works when recipient is offline
- Stateless between messages

---

### Use Cases

- First-time communication
- Offline messaging
- Low-frequency communication

---

## 3. Session Mode (Real-Time)

Session mode is activated when both parties are online.

### Characteristics

- Persistent session state
- Double Ratchet encryption
- Continuous key evolution
- Low-latency communication

---

### Activation Condition

```text
If both clients online AND session upgrade accepted:
    enter Session Mode
```

---

## 4. Session Scope

Sessions are scoped per:

```text
Device A ↔ Device B
```

Implications

- Each device pair has its own session
- No shared session across devices
- Compromise isolation guaranteed

---

## 5. Session Lifecycle

### 5.1 Session Creation

Triggered by:

- Session upgrade request (preferred)
- Reply during active communication

### 5.2 Active Session

During session:

- Messages use Double Ratchet
- Keys evolve per message
- Session state persists locally

### 5.3 Session Expiry

Session expires when:

- Inactivity timeout reached
- Device goes offline
- Explicit session close

### 5.4 Session Termination

```text
On termination:
    destroy session state
    clear chain keys
```

### 5.5 Post-Termination

- Future messages revert to asynchronous mode
- New session must be re-established

---

## 6. Session State Management

Each session maintains:

- Root key (RK)
- DH key pair
- Sending/receiving chain keys
- Message counters (Ns, Nr, PN)

### Storage

- Stored locally per device pair
- Must persist across app restarts

### Security Requirement

- Session state must be protected (secure storage)
- Unauthorized access compromises session security

---

## 7. Session Upgrade Flow

```text
Async Message
    ↓
SessionUpgradeRequest
    ↓
SessionUpgradeAccept
    ↓
Double Ratchet Starts
```

### Key Property

Session mode is an **extension**, not a replacement, of asynchronous communication.

---

## 8. Fallback Behavior

If session fails:

- Revert to asynchronous mode
- No message loss
- No dependency on session availability

---

## 9. Multi-Device Considerations

For a recipient with multiple devices:

```text
Sender → Device A (Session 1)
Sender → Device B (Session 2)
Sender → Device C (Session 3)
```

### Implications

- Independent sessions per device
- No shared keys
- Device-level revocation supported

---

## 10. Session Resumption

Sessions may resume if:

- Both devices remain online
- Session state still valid

Otherwise:

- New session must be established

---

## 11. Failure Handling

### Session Failure Cases

- State corruption
- Key mismatch
- Ratchet desynchronization

### Recovery Strategy

```text
If session invalid:
    discard session
    fallback to async mode
```

---

## 12. Security Properties

The session model ensures:

- Forward secrecy (via ratchet)
- Post-compromise recovery
- Session isolation per device
- No dependency on persistent connections

---

## 13. Summary

SMP session model provides:

- Flexible communication (async + real-time)
- Strong security guarantees
- Clean fallback behavior
- Device-level isolation

Sessions are optional optimizations, not mandatory dependencies.

---
