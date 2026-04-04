# Client Architecture Overview

This document defines the high-level architecture of an SMP client.

The client is the **trusted component** responsible for:

- Identity management
- Encryption and decryption
- Trust evaluation
- Session handling
- User interaction

---

## 1. Design Principles

- Client-side security enforcement
- Zero-trust server interaction
- Modular architecture
- Clear separation of responsibilities
- Secure key handling

---

## 2. Core Responsibilities

The client is responsible for:

- Generating and storing keys
- Verifying identities
- Encrypting outgoing messages
- Decrypting incoming messages
- Computing trust decisions
- Managing sessions
- Handling user actions

---

## 3. High-Level Architecture

```text
+--------------------------------------------------+
|                  SMP Client                      |
+--------------------------------------------------+
|                   UI Layer                       |
+--------------------------------------------------+
|               Application Layer                  |
+--------------------------------------------------+
|                  Core Modules                    |
+--------------------------------------------------+
|                 Crypto Engine                    |
+--------------------------------------------------+
|                Identity Manager                  |
+--------------------------------------------------+
```

---

## 4. Core Modules

---

### 4.1 Identity Manager

Responsible for:

- Managing IdentityObject
- Handling device registration
- Verifying other identities
- Interacting with Identity Registry

### 4.2 Key Store

Responsible for:

- Secure storage of private keys
- Hardware-backed storage (if available)
- Key retrieval and rotation

### 4.3 Crypto Engine

Responsible for:

- Encryption / Decryption
- Signature generation / verification
- Key derivation (HKDF)
- AEAD operations

### 4.4 Session Manager

Responsible for:

- Double Ratchet state
- Session lifecycle
- Key evolution
- Session persistence

### 4.5 Trust Engine (Client-Side)

Responsible for:

- Combining global + local trust
- Message routing decisions
- Trust level assignment

### 4.6 Network Client

Responsible for:

- REST API calls (control plane)
- gRPC communication (data plane)
- Retry logic
- Connection management

### 4.7 Message Processor

Responsible for:

- Parsing SMP packets
- Triggering decryption
- Validating signatures
- Routing messages

---

## 5. Data Flow (Incoming Message)

```text
Receive Packet
    ↓
Verify Signature
    ↓
Check Replay
    ↓
Decrypt
    ↓
Evaluate Trust
    ↓
Route (Inbox / Request / Block)
```

---

## 6. Data Flow (Outgoing Message)

```text
User Input
    ↓
Resolve Identity
    ↓
Evaluate Trust
    ↓
Apply Friction (if needed)
    ↓
Encrypt Message
    ↓
Send via Relay
```

---

## 7. Storage Model (Client-Side)

Client stores:

- Identity keys
- Session state
- Local trust data
- Message cache
- Device list

### Security Requirement

All sensitive data must be encrypted at rest

## 8. Multi-Device Behavior

Each device:

- Runs independent client instance
- Maintains its own sessions
- Syncs metadata (not plaintext keys)

## 9. Failure Handling

### Network Failure

- Retry with backoff
- Queue outgoing messages

### Decryption Failure

- Discard message
- Log error

### Session Failure

- Reset session
- Fallback to asynchronous mode

## 10. Security Properties

The client ensures:

- End-to-end encryption enforcement
- Identity verification
- Trust-based message control
- Secure key handling

## 11. Summary

The SMP client is the **core security boundary.**

All trust, encryption, and identity verification are enforced at the client level, ensuring that no external component can compromise message confidentiality or authenticity.

---
