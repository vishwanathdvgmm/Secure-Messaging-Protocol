# TUI Overview

This document defines the Terminal User Interface (TUI) for the SMP client.

The TUI provides a minimal, functional interface for interacting with the system.

It is intended for:

- Development
- Testing
- Debugging
- Early-stage usage

---

## 1. Design Principles

- Minimal and functional
- Keyboard-driven navigation
- Clear separation from core logic
- Real-time feedback for system state
- Explicit visibility of security/trust status

---

## 2. Role of TUI

The TUI acts as:

- Input layer (user actions)
- Output layer (system state visualization)

### It does NOT:

- Perform cryptographic operations
- Make trust decisions
- Handle network logic directly

---

## 3. Architecture Position

```text
User
↓
TUI Layer
↓
Application Layer
↓
Core Modules (Crypto / Session / Trust)
```

---

## 4. Core Responsibilities

- Display messages
- Show identity information
- Show trust levels
- Accept user input
- Trigger client operations

---

## 5. Key Features

### 5.1 Messaging Interface

- View inbox
- View message requests
- Send messages

### 5.2 Identity View

Display:

- Username (`alice#7f2a91`)
- Fingerprint
- Trust level

### 5.3 Trust Indicators

Display levels:

- QR Verified
- Accepted
- Unknown
- Suspicious

### 5.4 Message Requests

- Show unknown senders
- Accept / Ignore / Block

### 5.5 Debug Visibility

Show:

- Session state (optional debug mode)
- Encryption status
- Errors

---

## 6. Interaction Model

- Fully keyboard-driven
- No mouse dependency

---

### Example

| Key | Action      |
| --- | ----------- |
| `i` | Inbox       |
| `r` | Requests    |
| `n` | New message |
| `q` | Quit        |

---

## 7. State Integration

TUI interacts with:

- Message State
- Trust State
- Identity State

### It does NOT store state independently

---

## 8. Error Handling

Errors must be:

- Clearly displayed
- Non-blocking where possible

---

## 9. Security Considerations

- Never display private keys
- Mask sensitive values
- Show warnings for identity changes

---

## 10. Summary

The TUI provides:

- A functional interface for SMP
- Visibility into system behavior
- A testing and debugging tool

It is a thin layer over the core client logic.

---
