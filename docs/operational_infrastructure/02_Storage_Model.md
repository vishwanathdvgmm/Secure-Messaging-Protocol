# Storage Model

This document defines how SMP handles storage of messages and associated data.

All stored data is treated as **opaque encrypted blobs**, with strict lifecycle and retention policies.

---

## 1. Design Principles

- Zero-knowledge storage
- Immutable message objects
- Policy-driven retention
- Deterministic deletion
- Minimal metadata storage

---

## 2. Stored Data Types

SMP stores the following data:

| Data Type        | Description                        |
| ---------------- | ---------------------------------- |
| Message Packets  | Full encrypted SMP messages        |
| Metadata Index   | Minimal routing metadata           |
| Attachment Blobs | Encrypted files (external storage) |

---

## 3. Message Storage Structure

Each message is stored as:

```rust
StoredMessage {
    RecipientHash
    MessageID
    RawBinaryPacket
    ReceivedTimestamp
    ExpiryTimestamp
    RetrievalStatus
}
```

### Properties

- Entire packet stored without modification
- No field-level parsing required
- Signature remains valid end-to-end

---

## 4. Metadata Index

To enable efficient retrieval:

```rust
MetadataIndex {
    RecipientHash
    MessageID
    Timestamp
    Size
    RetrievalStatus
}
```

### Constraints

- No plaintext content stored
- No subject/body indexing
- No full identity exposure

---

## 5. Attachment Storage

Large attachments are stored separately:

```text
EncryptedBlob (opaque binary)
```

### Properties

- Stored as encrypted data only
- Identified via AttachmentID
- Linked to message via reference

## 6. Retention Policy Model

Each user defines:

```rust
RetentionPolicy {
    MaxDaysStored
    DeleteAfterRetrieval
    ArchiveMode
}
```

### Behavior

- Messages expire after MaxDaysStored
- Optional deletion immediately after retrieval
- Archive mode extends retention duration

---

## 7. Retention Execution

Storage system must enforce:

```text
If current_time ≥ ExpiryTimestamp:
    delete(message)
```

### Deletion Requirements

Deletion must:

- Remove encrypted blob
- Remove metadata index entry
- Remove attachment references
- Log deletion event

---

## 8. Deletion Guarantees

SMP requires **strong deletion semantics**:

- No soft-delete
- No hidden backups accessible to operators
- No content recovery after deletion

### Optional (Advanced)

- Secure overwrite (where applicable)
- Cryptographic erasure via key destruction

---

## 9. Retrieval Status Tracking

Messages track:

| **Status** | **Meaning**          |
| ---------- | -------------------- |
| Unread     | Not yet retrieved    |
| Retrieved  | Delivered to client  |
| Deleted    | Removed from storage |

### Behavior

- Retrieval updates status
- May trigger deletion based on policy

---

## 10. Storage Backends

SMP supports multiple storage systems:

- Object storage (preferred)
- Distributed key-value store
- Immutable blob storage

### Requirements

- High availability
- Durability
- Horizontal scalability

---

## 11. Data Integrity

Integrity is ensured via:

- End-to-end signatures
- AEAD authentication
- Hash-based attachment verification

### Storage Layer Role

- Does NOT verify cryptographic integrity
- Only ensures data persistence

---

## 12. Failure Handling

### Data Loss

- Mitigated via replication

### Partial Writes

- Must be rejected
- Atomic storage required

### Corruption

- Detected at client during decryption

---

## 13. Privacy Guarantees

Storage system must not:

- Access plaintext content
- Index message content
- Build user communication graphs

### Allowed Metadata

- Recipient hash
- Message size
- Timestamp

---

## 14. Legal Considerations

Storage operators can claim:

- No access to message content
- No ability to decrypt data
- Data controlled by user-defined retention

---

## 15. Summary

The SMP storage model ensures:

- Secure handling of encrypted data
- Strict retention enforcement
- Strong deletion guarantees
- Minimal metadata exposure

All stored data remains opaque to the storage layer.

---
