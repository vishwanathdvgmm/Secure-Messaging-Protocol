# Registry API

This document defines REST APIs for interacting with:

- Identity Registry
- Trust Engine

These endpoints provide **verifiable public data**, not private information.

---

## 1. Design Principles

- Read-heavy APIs
- Verifiable responses
- No private data exposure
- Stateless requests

---

## 2. Base URL

```url
https://registry.smp.network/v1/
```

---

## 3. Identity Registry APIs

### 3.1 Get Identity Record (Latest)

#### HTTP

```http
GET /identity/{identity_hash}
```

#### Response

```json
{
  "identity_hash": "hex",
  "identity_object": { ... },
  "version": 5,
  "timestamp": 1710000000,
  "registry_signature": "hex"
}
```

#### Client Verification

Client must:

- Verify registry signature
- Verify identity signature (IK)
- Validate version consistency

### 3.2 Get Identity History

#### HTTP

```http
GET /identity/{identity_hash}/history
```

#### Response

```json
{
	"records": [
		{
			"version": 1,
			"record_hash": "..."
		},
		{
			"version": 2,
			"record_hash": "..."
		}
	]
}
```

#### Use Case

- Audit identity changes
- Verify key rotation chain

### 3.3 Get Identity Record by Version

```http
GET /identity/{identity_hash}/version/{version}
```

---

## 4. Trust Engine APIs

### 4.1 Get Trust Record

#### HTTP

```http
GET /trust/{identity_hash}
```

#### Response

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

#### Usage

- Input to client-side trust computation
- Determine friction level

### 4.2 Submit Abuse Report

#### HTTP

```http
POST /trust/report
```

#### Request

```json
{
	"reporter_identity": "hex",
	"target_identity": "hex",
	"message_hash": "hex",
	"reason_code": "SPAM",
	"signature": "hex"
}
```

#### Behavior

- Must be authenticated
- Signature required
- Duplicate reports rejected

---

### 5.1 Get Merkle Root

#### HTTP

```http
GET /transparency/root
```

#### Response

```json
{
	"merkle_root": "hex",
	"timestamp": 1710000000
}
```

### 5.2 Get Inclusion Proof

#### HTTP

```http
GET /transparency/proof/{identity_hash}
```

#### Response

```json
{
	"proof": ["hash1", "hash2", "hash3"]
}
```

#### Client Responsibility

- Verify proof against Merkle root
- Detect tampering

---

## 6. Rate Limiting

- Per-IP limits
- Per-identity limits
- Abuse prevention controls

---

## 7. Error Format

```json
{
	"error_code": "NOT_FOUND",
	"message": "Identity does not exist"
}
```

---

## 8. Security Requirements

- HTTPS required
- All responses must be signed
- Clients must verify all cryptographic data

---

## 9. Privacy Constraints

Registry must NOT expose:
Message content
Contact lists
Communication patterns

---

## 10. Summary

The Registry API provides:

- Access to identity records
- Trust data retrieval
- Abuse reporting
- Transparency verification

All data is public, verifiable, and privacy-preserving.

---
