# Storage Service

This document defines the implementation of the SMP storage backend.

The storage service is responsible for:

- Persisting message packets
- Providing retrieval access
- Enforcing retention policies
- Ensuring deletion guarantees

---

## 1. Design Principles

- Opaque data storage
- High durability
- Horizontal scalability
- Strong deletion guarantees
- Minimal metadata exposure

---

## 2. Data Model

### Stored Message

```go
type StoredMessage struct {
    MessageID        string
    RecipientHash    []byte
    RawPacket        []byte
    ReceivedAt       int64
    ExpiryTimestamp  int64
    Retrieved        bool
}
```

---

## 3. Storage Interface

```go
type StorageBackend interface {
    StoreMessage(recipient []byte, packet []byte) (string, error)
    FetchMessages(recipient []byte, limit int) ([]StoredMessage, error)
    MarkRetrieved(messageID string) error
    DeleteMessage(messageID string) error
}
```

---

## 4. Write Flow

```text
Receive Packet
    ↓
Generate MessageID
    ↓
Persist StoredMessage
    ↓
Return MessageID
```

### Requirements

- Atomic write
- No partial storage
- No packet modification

---

## 5. Read Flow

```text
Query by RecipientHash
    ↓
Fetch messages (limit)
    ↓
Return RawPacket list
```

### Behavior

- Return unread messages first
- Respect limit
- Preserve approximate order (timestamp)

---

## 6. Retrieval Handling

After delivery:

```go
MarkRetrieved(messageID)
```

### Optional Behavior

- Immediate deletion (if policy enabled)
- Mark as retrieved otherwise

---

## 7. Deletion Flow

```text
DeleteMessage(messageID)
    ↓
Remove data
    ↓
Remove metadata
```

### Requirements

- No soft delete
- No recovery after deletion

---

## 8. Retention Enforcement

Background process:

```text
If current_time ≥ ExpiryTimestamp:
    DeleteMessage(messageID)
```

### Execution

- Periodic job (cron / worker)
- Must scan expired messages

---

## 9. Indexing Strategy

### Primary Index

- RecipientHash

### Secondary Index

- Timestamp
- Retrieved status

### Constraint

- No content indexing allowed

---

## 10. Storage Backends

Supported options:

- Object storage (preferred)
- Distributed KV store
- Append-only log system

---

## 11. Consistency Model

- Strong consistency for writes
- Eventual consistency acceptable for reads

---

## 12. Failure Handling

#### Write Failure

- Reject operation
- Do not return MessageID

### Read Failure

- Return error
- Client retries

### Partial Failure

- Must not occur (atomic operations required)

---

## 13. Scalability

Storage must support:

- Horizontal scaling
- Sharding by RecipientHash
- High throughput

---

### 14. Security Constraints

Storage must NOT:

- Access plaintext data
- Modify stored packets
- Build user communication graphs

---

### 15. Logging Rules

Allowed:

- Errors
- Storage metrics

Forbidden:

- Message content
- Sensitive identifiers

---

## 16. Monitoring

Track:

- Write latency
- Read latency
- Storage usage
- Deletion rate

---

## 17. Summary

The Storage Service:

- Stores encrypted message packets
- Provides efficient retrieval
- Enforces strict deletion policies

It operates as a passive, zero-knowledge data layer in SMP.

---
