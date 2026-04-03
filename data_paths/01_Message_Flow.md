# Message Flow

This document defines the end-to-end flow of a message in the Secure Messaging Protocol (SMP).

It describes how messages are created, transmitted, stored, and delivered while preserving end-to-end security.

---

## 1. Flow Overview

Message delivery in SMP follows this sequence:

```
Sender Client
    ↓
Identity + Trust Evaluation
    ↓
Encryption
    ↓
Relay Submission
    ↓
Relay Storage
    ↓
Recipient Retrieval
    ↓
Decryption + Trust Evaluation
```

---

## 2. Sender-Side Flow

### Step 1 — Identity Resolution

Sender performs:

- Lookup recipient identity
- Fetch IdentityObject
- Fetch Pre-Key Bundle (if no session exists)

---

### Step 2 — Identity Verification

Sender verifies:

- Identity signature (IK)
- Domain binding (if present)
- Revocation status
- Key validity

If verification fails → **abort**

---

### Step 3 — Trust Evaluation

Sender evaluates:

- Recipient trust status
- Required friction (if unknown)

---

### Step 4 — Session Determination

```text
If session exists:
    use Double Ratchet
Else:
    use Pre-Key handshake
```

### Step 5 — Message Construction

Message payload includes:

- Subject
- Body
- Attachments
- Optional metadata

### Step 6 — Encryption

- Derive session/message key
- Encrypt payload using AEAD
- Bind header as AAD

### Step 7 — Signature

Sender signs full packet:

```text
Signature = Sign(SK_priv, hash(packet))
```

### Step 8 — Transmission

Sender sends packet to relay:

```text
POST /relay/send
```

---

## 3. Relay-Side Flow

### Step 1 — Packet Validation

Relay verifies:

- Structural validity
- Signature presence (not authenticity)
- Field lengths

### Step 2 — Policy Enforcement

Relay enforces:

- Rate limits
- Friction format validation
- Abuse thresholds

### Step 3 — Storage

Relay stores message as opaque binary:

```rust
StoredMessage {
    RecipientHash
    MessageID
    RawPacket
    Timestamp
}
```

### Step 4 — No Decryption

Relay:

- Cannot decrypt message
- Cannot modify content
- Cannot access attachments

---

## 4. Recipient-Side Flow

### Step 1 — Authentication

Recipient authenticates to relay:

- Challenge-response using Identity Key

### Step 2 — Message Retrieval

Recipient fetches:

```text
GET /relay/messages
```

### Step 3 — Packet Verification

Recipient verifies:

- Sender signature (SK)
- Identity validity
- Revocation status

### Step 4 — Decryption

- Derive message key
- Decrypt payload
- Validate AEAD integrity

### Step 5 — Replay Protection

Recipient checks:

- BundleID uniqueness
- Message counters
- Timestamp validity

### Step 6 — Trust Evaluation

Recipient determines:

```text
If sender trusted:
    deliver to inbox
Else:
    route to message request queue
```

### Step 7 — User Interaction

User may:

- Accept → establish trust
- Ignore → discard
- Block → prevent future messages

---

## 5. Message States

Messages transition through states:

| **State** | **Description**        |
| --------- | ---------------------- |
| Sent      | Created by sender      |
| Stored    | Stored in relay        |
| Retrieved | Fetched by recipient   |
| Decrypted | Successfully decrypted |
| Accepted  | Trusted by user        |
| Ignored   | Discarded              |
| Blocked   | Sender blocked         |

---

## 6. Failure Handling

### Sender-Side Failures

- Identity verification failure → abort
- Encryption failure → abort
- Relay rejection → retry or fail

### Relay Failures

- Rate limit exceeded → reject
- Invalid packet → reject

### Recipient Failures

- Signature invalid → discard
- Decryption failure → discard
- Replay detected → discard

---

## 7. Security Properties

This flow ensures:

- End-to-end confidentiality
- No relay access to plaintext
- Verified sender identity
- Controlled message acceptance
- Replay resistance

---

### 8. Summary

SMP message flow ensures that:

- Messages are encrypted before leaving sender
- Relays act as zero-knowledge transport nodes
- Recipients fully control message acceptance

The system maintains strong security guarantees while supporting asynchronous communication.

---
