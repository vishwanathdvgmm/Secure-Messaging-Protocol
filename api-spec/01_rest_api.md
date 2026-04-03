# REST API Specification

This document defines the REST-based control plane APIs for SMP.

REST APIs are used for:

- Identity lookup
- Registry access
- Authentication
- Metadata queries

High-throughput messaging is handled separately via gRPC (Relay API).

---

## 1. Design Principles

- Stateless requests
- JSON encoding
- HTTPS only
- Identity-based authentication
- No plaintext message handling

---

## 2. Base URL

```text
https://api.smp.network/v1/
```

---

## 3. Authentication Model

All authenticated requests use:

```text
Authorization: SMP-Signature <signature>
X-Identity-Hash: <identity_hash>
X-Timestamp: <unix_ms>
```

### Signature

```text
Signature = Sign(SK_priv, hash(method + path + body + timestamp))
```

### Requirements

- Timestamp must be within allowed window
- Signature must match identity
- Prevents replay attacks

---

## 4. Identity Endpoints

### 4.1 Get Identity Record

```http
GET /identity/{identity_hash}
```

### Response

```json
{
  "identity_hash": "hex",
  "identity_object": { ... },
  "version": 3,
  "timestamp": 1710000000,
  "signature": "hex"
}
```

### 4.2 Search Identity (Opt-In Only)

```http
POST /identity/search
```

### Request

```json
{
	"username": "alice#7f2a91"
}
```

### Response

```json
{
	"identity_hash": "hex",
	"exists": true
}
```

### Constraints

- Full identifier required
- No partial search allowed
- Rate-limited

---

## 5. Trust Endpoints

### 5.1 Get Trust Record

```http
GET /trust/{identity_hash}
```

### Response

```json
{
	"identity_hash": "hex",
	"identity_age": 120,
	"domain_verified": true,
	"abuse_score": 2.4,
	"stake": 100,
	"last_activity": 1710000000
}
```

---

## 6. Registry Endpoints

### 6.1 Get Latest Identity Record

```http
GET /registry/identity/{identity_hash}/latest
```

### 6.2 Get Identity History

```http
GET /registry/identity/{identity_hash}/history
```

### Response

```json
{
	"records": [
		{ "version": 1, "hash": "..." },
		{ "version": 2, "hash": "..." }
	]
}
```

---

## 7. Device Management

### 7.1 Register Device

```http
POST /device/register
```

### Request

```json
{
	"device_public_key": "hex",
	"signature": "identity_signature"
}
```

### 7.2 Revoke Device

```http
POST /device/revoke
```

### Request

```json
{
	"device_id": "hex",
	"signature": "identity_signature"
}
```

---

## 8. Pre-Key Management

### 8.1 Publish Pre-Key Bundle

```http
POST /prekey/publish
```

### Request

```json
{
	"bundle": { ... },
	"signature": "identity_signature"
}
```

### 8.2 Fetch Pre-Key Bundle

```http
GET /prekey/{identity_hash}
```

---

## 9. Rate Limiting

All endpoints enforce:

- Per-IP limits
- Per-identity limits
- Burst protection

---

## 10. Error Format

```json
{
	"error_code": "INVALID_SIGNATURE",
	"message": "Signature verification failed"
}
```

---

## 11. Security Requirements

- HTTPS mandatory
- Signature verification required
- Replay protection via timestamp
- No sensitive data in logs

---

## 12. Summary

The REST API provides:

- Identity and registry access
- Trust signal retrieval
- Device and key management

All message transport is handled outside REST (gRPC layer).

---
