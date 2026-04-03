# Relay API (gRPC)

This document defines the gRPC-based relay communication layer for SMP.

The Relay API handles:

- Message submission
- Message retrieval
- Streaming delivery

All messages are treated as **opaque binary packets**.

---

## 1. Design Principles

- Binary-safe transport
- High throughput
- Low latency
- Stateless relay behavior
- Zero-knowledge message handling

---

## 2. Service Definition

```rust
service RelayService {
    rpc SendMessage (SendRequest) returns (SendResponse);
    rpc FetchMessages (FetchRequest) returns (FetchResponse);
    rpc StreamMessages (StreamRequest) returns (stream MessagePacket);
}
```

---

## 3. Message Packet

```rust
message MessagePacket {
    bytes raw_packet = 1; // Full SMP binary message
}
```

### Properties

- Relay does NOT parse packet content
- Relay treats packet as opaque binary

---

## 4. Send Message

### RPC

```rust
rpc SendMessage (SendRequest) returns (SendResponse);
```

### Request

```rust
message SendRequest {
    bytes raw_packet = 1;
}
```

### Response

```rust
message SendResponse {
    string message_id = 1;
    int64 timestamp = 2;
}
```

### Behavior

Relay must:

- Validate basic structure (size, format)
- Enforce rate limits
- Store message without modification

---

## 5. Fetch Messages (Pull Model)

### RPC

```rust
rpc FetchMessages (FetchRequest) returns (FetchResponse);
```

### Request

```rust
message FetchRequest {
    bytes recipient_hash = 1;
    int32 limit = 2;
}
```

### Response

```rust
message FetchResponse {
    repeated MessagePacket messages = 1;
}
```

### Behavior

- Returns pending messages
- Does not decrypt or inspect content

---

## 6. Stream Messages (Push Model)

### RPC

```rust
rpc StreamMessages (StreamRequest) returns (stream MessagePacket);
```

### Request

```rust
message StreamRequest {
    bytes recipient_hash = 1;
}
```

### Behavior

- Persistent connection
- Real-time delivery
- Sends messages as they arrive

### Use Case

- Real-time messaging
- Session mode optimization

---

## 7. Authentication (gRPC)

Each request must include metadata:

```text
x-identity-hash
x-signature
x-timestamp
```

### Verification

Same as REST:

- Signature validation
- Timestamp validation
- Identity lookup

---

## 8. Rate Limiting

Relay enforces:

- Per-identity send limits
- Per-IP limits
- Burst limits

### Failure Response

```rust
message ErrorResponse {
    string error_code = 1;
    string message = 2;
}
```

---

## 9. Message Storage Integration

### On SendMessage:

```text
Store → Assign MessageID → Return Response
```

### On Fetch/Stream:

```text
Retrieve → Send Packet → Mark Retrieved (optional)
```

---

## 10. Ordering Guarantees

Messages are delivered in timestamp order
Strict ordering not guaranteed across distributed nodes

---

## 11. Failure Handling

### Send Failure

- Reject invalid packet
- Reject rate-limited sender

### Stream Failure

- Client reconnect required
- No message loss (stored)

---

## 12. Security Properties

Relay API ensures:

- No message inspection
- No plaintext exposure
- End-to-end encryption preserved
- Transport-level efficiency

---

## 13. Summary

The Relay API provides:

- Efficient message transport
- Real-time streaming capability
- Zero-knowledge message handling

It forms the backbone of SMP’s communication layer.

---
