# Relay Service

This document defines the implementation of the SMP Relay Service.

The relay is responsible for:

- Receiving messages
- Storing messages
- Delivering messages

The relay operates under a **zero-knowledge model**.

---

## 1. Design Principles

- Stateless processing
- Opaque message handling
- High throughput
- Horizontal scalability
- No cryptographic logic beyond validation

---

## 2. Core Responsibilities

- Accept message packets
- Enforce rate limits
- Store messages
- Deliver messages to recipients
- Maintain availability

---

## 3. Service Structure

```go
type RelayService struct {
    Storage        StorageBackend
    RateLimiter    RateLimiter
    AuthValidator  AuthValidator
}
```

---

## 4. Incoming Request Flow (SendMessage)

```text
Receive Request
    ↓
Authenticate Sender
    ↓
Rate Limit Check
    ↓
Basic Validation
    ↓
Store Message
    ↓
Return MessageID
```

---

## 5. Authentication

Relay must verify:

- Identity hash
- Signature
- Timestamp validity

Reject if invalid

---

## 6. Basic Validation

Check:

- Packet size limits
- Required fields present
- Friction block exists (if required)

### Important

Relay does NOT:

- Decrypt messages
- Verify message signatures (end-to-end)

---

## 7. Rate Limiting

Apply:

- Per-identity limits
- Per-IP limits
- Burst limits

### On violation

```text
Reject request
```

---

## 8. Storage Interaction

### Store Message

```go
func StoreMessage(recipientHash []byte, packet []byte) (messageID string)
```

Requirements:

- Atomic write
- No modification of packet
- Assign unique MessageID

---

## 9. Fetch Messages Flow

```text
Receive Fetch Request
    ↓
Authenticate
    ↓
Query Storage
    ↓
Return Messages
```

### Behavior

- Return messages for recipient
- Respect limit
- Do not alter message order (best effort)

---

## 10. Streaming Flow

```text
Open Stream
    ↓
Authenticate
    ↓
Subscribe to recipient queue
    ↓
Push messages in real-time
```

### Requirements

- Persistent connection
- Handle reconnections gracefully

---

## 11. Message Lifecycle (Relay)

| **Stage** | **Description**         |
| --------- | ----------------------- |
| Received  | Accepted from sender    |
| Stored    | Persisted in storage    |
| Pending   | Awaiting recipient      |
| Delivered | Sent to recipient       |
| Deleted   | Removed after retention |

---

## 12. Deletion Policy

Relay must delete:

- Expired messages
- Retrieved messages (if configured)

No soft delete allowed

---

## 13. Failure Handling

### Storage Failure

- Reject message
- Do not acknowledge success

### Network Failure

- Client retries

### Stream Failure

- Client reconnect required

---

## 14. Scalability Model

Relay should support:

- Horizontal scaling
- Stateless workers
- Shared storage backend

### Load Distribution

- Hash-based routing (optional)
- Load balancers

---

## 15. Security Constraints

Relay must NOT:

- Inspect message content
- Modify message packets
- Store decrypted data
- Build communication graphs

---

## 16. Logging Rules

Allowed:

- Errors
- Request metadata (limited)

Forbidden:

- Message content
- Sensitive identifiers in plaintext

---

## 17. Monitoring

Track:

- Request rate
- Error rate
- Storage latency
- Queue depth

---

## 18. Summary

The Relay Service:

- Provides message transport and storage
- Operates without access to plaintext data
- Enforces rate limits and availability

It is a critical but untrusted component in SMP.

---
