# Metadata Protection

This document defines how SMP minimizes and protects metadata to preserve user privacy.

While message content is fully encrypted, metadata must be carefully controlled to prevent indirect information leakage.

---

## 1. Design Principles

- Minimize metadata exposure
- Avoid unnecessary data retention
- Prevent communication graph reconstruction
- Ensure metadata is not linkable across contexts
- Keep relay visibility minimal

---

## 2. Metadata Categories

Metadata in SMP is divided into:

| Category          | Description             |
| ----------------- | ----------------------- |
| Routing Metadata  | Required for delivery   |
| Protocol Metadata | Required for validation |
| Optional Metadata | Used for optimization   |

---

## 3. Exposed Metadata (Relay-Visible)

Relays can access only:

- SenderIdentityHash
- RecipientIdentityHash
- Timestamp
- Message size
- Friction type

### Properties

- Fixed-size identifiers (hashes)
- No plaintext identity (username not exposed)
- No message content

---

## 4. Hidden Metadata (Encrypted)

The following is always encrypted:

- Subject
- Message body
- Attachments
- Session data
- Contact context

---

## 5. Identity Protection

### Hash-Based Identity

```text
IdentityHash = SHA-256(IdentityPublicKey)
```

### Benefit

- Prevents direct identity exposure
- Hides human-readable identifiers from relay

## 6. Communication Graph Protection

SMP prevents building communication graphs by:

- Avoiding public directories
- Using opt-in discovery only
- Requiring full identifier for lookup
- Limiting relay metadata visibility

### Result

Relays cannot easily determine:

- Social connections
- Frequency patterns at scale
- Contact relationships

## 7. Traffic Analysis Mitigation

While full resistance is not possible, SMP reduces exposure:

### Techniques

- Uniform message format
- Fixed-size header fields
- Optional message batching
- Controlled timing variance (client-side)

### Limitation

- Network-level observers may still infer patterns

## 8. Message Size Leakage

Message size can reveal information.

Mitigation options:

- Padding (optional)
- Size buckets (future enhancement)

### Trade-off

| **Approach** | **Impact**                |
| ------------ | ------------------------- |
| No padding   | Efficient, less private   |
| Padding      | More private, higher cost |

---

## 9. Timestamp Exposure

Relays require timestamps for:

- Ordering
- Expiry handling

### Risk

- Timing correlation attacks

### Mitigation

- Client-side delay (optional)
- Batching (optional)

## 10. Multi-Device Metadata

Each device receives:

- Separate encrypted payload

### Implication

- Device count may be inferred indirectly
- Exact device identity remains hidden

## 11. Storage Metadata

Stored metadata includes:

- RecipientHash
- MessageID
- Timestamp
- Size

### Restrictions

Storage must NOT:

- Index content
- Track communication relationships
- Build user graphs

## 12. Relay Constraints

Relays must NOT:

- Correlate sender-recipient pairs beyond delivery
- Persist communication patterns long-term
- Expose metadata externally

## 13. Optional Enhancements (Future)

- Mix networks
- Onion routing
- Metadata padding policies
- Private information retrieval (PIR)

## 14. Security Properties

The metadata protection model ensures:

- Minimal identity exposure
- Reduced communication traceability
- Resistance to large-scale metadata harvesting

## 15. Summary

SMP minimizes metadata exposure by:

- Using identity hashes instead of usernames
- Encrypting all sensitive content
- Restricting relay visibility
- Avoiding global directories

While not eliminating all metadata leakage, SMP significantly reduces the attack surface compared to traditional systems.

---
