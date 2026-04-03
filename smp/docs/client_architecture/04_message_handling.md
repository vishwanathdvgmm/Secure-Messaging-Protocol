# Message Handling

This document defines how the SMP client processes messages internally.

It covers both:

- Incoming message handling
- Outgoing message handling

All processing must follow strict validation and security rules.

---

## 1. Design Principles

- Deterministic processing pipeline
- Strict validation before decryption
- Fail-safe rejection on errors
- Separation of crypto, trust, and UI layers

---

## 2. Incoming Message Flow

### Pipeline

```text
Receive Packet
    ↓
Basic Validation
    ↓
Signature Verification
    ↓
Replay Check
    ↓
Session Resolution
    ↓
Decryption
    ↓
Trust Evaluation
    ↓
Routing (Inbox / Request / Block)
    ↓
UI Update
```

---

## 3. Step-by-Step Processing

### 3.1 Receive Packet

- From relay (gRPC)
- Raw binary SMP packet

### 3.2 Basic Validation

Check:

- Packet size
- Version support
- Field structure

### 3.3 Signature Verification

```text
Verify(Signature, SenderSigningKey)
```

Reject if invalid

### 3.4 Replay Check

Verify:

- BundleID uniqueness
- Message counters (Ns/Nr)
- Timestamp validity

Reject if duplicate or invalid

### 3.5 Session Resolution

```text
If session exists:
    use Double Ratchet
Else:
    use Pre-Key decryption
```

### 3.6 Decryption

- Derive message key
- Decrypt payload (AEAD)
- Validate integrity

Reject if decryption fails

### 3.7 Trust Evaluation

Client determines:

- Trusted → Inbox
- Unknown → Message Request Queue
- Blocked → Discard

### 3.8 State Update

Update:

- Session state
- Message cache
- Trust records (if interaction occurs)

### 3.9 UI Update

Display:

- Message content
- Trust indicators
- Identity fingerprint (if needed)

---

## 4. Outgoing Message Flow

### Pipeline

```text
User Input
    ↓
Recipient Resolution
    ↓
Trust Evaluation
    ↓
Friction Application (if needed)
    ↓
Session Determination
    ↓
Encryption
    ↓
Packet Construction
    ↓
Send via Relay
```

---

## 5. Step-by-Step Processing

### 5.1 User Input

- Message content
- Attachments
- Target identity

### 5.2 Recipient Resolution

- Fetch identity
- Verify identity record

### 5.3 Trust Evaluation

Determine:

- Trust level
- Required friction

### 5.4 Friction Application

If required:

- Generate PoW
- Attach stake

### 5.5 Session Determination

```text
If active session:
    use Double Ratchet
Else:
    use Pre-Key handshake
```

### 5.6 Encryption

- Derive message/session key
- Encrypt payload
- Bind header as AAD

### 5.7 Packet Construction

Build SMP message:

- Header
- Friction block
- Ciphertext
- Signature

### 5.8 Transmission

- Send via Relay API (gRPC)
- Handle response

---

## 6. Message State Transitions

### State

| **State** | **Description**        |
| --------- | ---------------------- |
| Created   | User composed message  |
| Encrypted | Ready for sending      |
| Sent      | Delivered to relay     |
| Delivered | Retrieved by recipient |
| Read      | Opened by recipient    |

---

## 7. Error Handling

### Incoming Errors

| **Error**          | **Action**    |
| ------------------ | ------------- |
| Invalid signature  | Reject        |
| Replay detected    | Reject        |
| Decryption failure | Discard       |
| Session mismatch   | Reset session |

### Outgoing Errors

| **Error**              | **Action**    |
| ---------------------- | ------------- |
| Identity fetch failure | Abort         |
| Encryption failure     | Abort         |
| Relay rejection        | Retry or fail |

---

## 8. Retry Logic

- Exponential backoff
- Max retry limit
- Queue unsent messages

---

## 9. Security Properties

Message handling ensures:

- Only valid messages are processed
- No unverified data reaches UI
- Trust rules enforced before display
- Cryptographic guarantees maintained

---

## 10. Summary

The message handling pipeline ensures:

- Secure processing of all messages
- Strict validation and verification
- Proper integration of crypto, trust, and UI

All message operations are deterministic and security-first.

---
