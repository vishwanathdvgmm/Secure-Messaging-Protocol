# Relay Architecture

This document defines the role and behavior of relay nodes in SMP.

Relays act as **zero-knowledge transport and storage nodes**, responsible for message delivery without access to message content.

---

## 1. Relay Role

The relay is responsible for:

- Receiving SMP messages from senders
- Storing encrypted message packets
- Delivering messages to recipients upon request
- Enforcing basic network-level policies

### Key Principle

Relays must operate under a **zero-trust model**.

They must not rely on access to plaintext or cryptographic secrets.

---

## 2. Responsibilities

Relays MUST:

- Accept valid SMP packets
- Validate structural integrity
- Enforce rate limits
- Store encrypted messages
- Deliver messages to authenticated recipients
- Apply retention policies

---

## 3. Prohibited Actions

Relays MUST NOT:

- Decrypt message content
- Modify message fields
- Re-sign or re-encrypt packets
- Interpret message payload
- Compute trust scores
- Perform identity verification beyond basic validation

---

## 4. Message Storage Model

Messages are stored as opaque binary objects:

```rust
StoredMessage {
    RecipientHash
    MessageID
    RawPacket
    ReceivedTimestamp
    ExpiryTimestamp
    RetrievalStatus
}
```

### Properties

- Entire packet stored without modification
- Signature remains valid end-to-end
- No reserialization allowed

---

## 5. Metadata Visibility

Relays can access only minimal metadata:

- SenderIdentityHash
- RecipientIdentityHash
- Timestamp
- Message size
- Friction type

### Relays cannot access:

- Subject
- Body
- Attachments
- Session data

---

## 6. Retention Policy

Each recipient defines:

```rust
RetentionPolicy {
   MaxDaysStored
   DeleteAfterRetrieval
   ArchiveMode
}
```

### Behavior

- Messages expire after defined duration
- Optional deletion after retrieval
- Deletion must remove all traces

---

## 7. Message Retrieval Flow

### Step 1 — Authentication

Recipient authenticates using:

```text
Challenge-Response with Identity Key
```

### Step 2 — Message Listing

Relay returns:

```text
List of MessageIDs
```

### Step 3 — Message Fetch

Recipient fetches full packets.

### Step 4 — Post-Retrieval Handling

- Mark as retrieved
- Delete if policy requires

---

## 8. Abuse & Rate Enforcement

Relays enforce:

- Per-sender rate limits
- Per-recipient inflow limits
- Friction block validation
- Basic DoS protection

### If abuse detected:

```text
TemporarySenderThrottle
```

### Optional:

```text
RegistryFlagSubmission
```

---

## 9. Replay Handling

Relays do NOT perform full replay detection.

- Responsibility lies with recipient
- Relay may optionally detect duplicate packets

---

## 10. Availability Model

Relays may be deployed as:

- Single-node (initial phase)
- Distributed cluster
- Geo-distributed network

### Storage Backends

- Object storage (preferred)
- Distributed key-value store
- Immutable blob storage

---

## 11. Failure Handling

### Relay Failure

- Message temporarily unavailable
- Sender may retry

### Network Partition

- Messages queued until delivery possible

### Data Loss

- Mitigated via replication strategies

---

## 12. SMTP Bridge (Optional)

Legacy integration may allow:

```text
SMTP → SMP (restricted)
```

### Behavior

- Converted into SMP message
- Marked as low-trust
- Routed to lower inbox tier

### Restriction

```text
SMP → SMTP is NOT allowed
```

## 13. Legal Exposure Boundary

Relay operators can claim:

- No plaintext access
- No key custody
- Encrypted data only
- User-controlled retention

## 14. Security Properties

Relay architecture ensures:

- End-to-end confidentiality
- No server-side data exposure
- Minimal metadata leakage
- Controlled abuse surface

## 15. Summary

Relays in SMP are:

- Stateless with respect to message content
- Zero-knowledge storage nodes
- Responsible only for transport and availability

All security guarantees are enforced at the client level.

---
