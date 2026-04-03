# Authentication Flow

This document defines how clients authenticate with SMP services.

SMP uses **cryptographic identity-based authentication**, eliminating passwords and shared secrets.

---

## 1. Design Principles

- No passwords
- No shared secrets
- Identity = public key
- Authentication via signature
- Replay protection mandatory

---

## 2. Authentication Model

Each request is authenticated using:

- Identity Key (IK) or Signing Key (SK)
- Request signature
- Timestamp

---

## 3. Authentication Headers

All authenticated requests must include:

```rust
Authorization: SMP-Signature <signature>
X-Identity-Hash: <identity_hash>
X-Timestamp: <unix_ms>
```

---

## 4. Signature Generation

Client computes:

```rust
Signature = Sign(SK_priv, hash(method + path + body + timestamp))
```

### Inputs

- HTTP method (GET, POST, etc.)
- Request path
- Request body (if present)
- Timestamp

### Purpose

- Bind request to identity
- Prevent tampering
- Prevent replay

---

## 5. Server Verification Flow

Upon receiving request:

### Step 1 — Extract Headers

- IdentityHash
- Signature
- Timestamp

### Step 2 — Fetch Identity

- Retrieve IdentityObject using IdentityHash
- Extract SigningPublicKey

### Step 3 — Validate Timestamp

```rust
|current_time - timestamp| < allowed_window
```

### Step 4 — Verify Signature

```rust
Verify(Signature, SigningPublicKey)
```

### Step 5 — Accept or Reject

- Valid → process request
- Invalid → reject

---

## 6. Replay Protection

Replay attacks are prevented using:

- Timestamp validation
- Optional nonce tracking (advanced)

### Allowed Window

```text
± 60 seconds (configurable)
```

---

## 7. Optional Session Tokens (Optimization)

SMP does NOT require sessions, but may support:

### Token-Based Optimization

After initial authentication:

```rust
SessionToken (short-lived)
```

### Flow

1. Client signs initial request
2. Server returns session token
3. Subsequent requests use token

### Properties

- Short-lived (e.g., 5–15 minutes)
- Bound to identity
- Reduces signature overhead

---

## 8. Device Authentication

Authentication is per device:

- Each device signs independently
- Device keys linked to identity

### Verification

Server must:

- Validate device is in identity’s DeviceList
- Reject unknown devices

---

## 9. Failure Conditions

Authentication fails if:

- Signature invalid
- Timestamp expired
- Identity not found
- Device not registered

---

## 10. Security Properties

This authentication model ensures:

- Strong identity binding
- No credential leakage
- Replay resistance
- No password-based attacks

---

## 11. Summary

SMP authentication is:

- Stateless
- Cryptographic
- Identity-based

Each request is independently verifiable without relying on stored secrets.

---
