# Identity Registry

This document defines the architecture and behavior of the SMP Identity Registry.

The Identity Registry is responsible for storing and serving identity records in a **tamper-evident and verifiable manner**.

---

## 1. Role of the Identity Registry

The registry acts as:

- Source of truth for identity records
- Distribution point for public keys
- Transparency system for identity changes

### Key Principle

The registry must be **verifiable, append-only, and tamper-evident**.

---

## 2. Identity Record Structure

Each identity is stored as:

```rust
IdentityRecord {
    IdentityHash
    IdentityObject
    Version
    PreviousRecordHash
    RegistrySignature
    Timestamp
}
```

### Properties

- Linked via `PreviousRecordHash`
- Forms a verifiable chain of updates
- Prevents silent overwrites

---

## 3. Append-Only Log Model

Registry maintains an append-only log:

```text
Record_1 → Record_2 → Record_3 → ...
```

### Guarantees

- No record can be deleted or altered
- All updates are publicly verifiable
- History is preserved permanently

---

## 4. Transparency Mechanism

The registry may implement a **Merkle Tree log**.

### Structure

```text
MerkleRoot = hash(all_identity_records)
```

### Benefits

- Efficient verification
- Proof of inclusion
- Detection of tampering

### Optional

- Public transparency server
- Third-party auditors

---

## 5. Identity Lookup

Clients query registry using:

```text
IdentityHash
```

### Response

```rust
IdentityRecord + InclusionProof
```

### Client Verification

Client must:

- Verify registry signature
- Verify Merkle proof
- Validate record chain integrity

---

## 6. Identity Updates

### Key Rotation

New record appended:

```rust
NewIdentityRecord {
    PreviousRecordHash
    UpdatedKeys
    Signature(IK_priv)
}
```

### Properties

- Old records remain visible
- Clients can verify transition

---

## 7. Revocation Model

If identity compromised:

```rust
RevocationRecord {
    IdentityHash
    ReasonCode
    Signature(IK_priv)
}
```

### Effects

- Identity marked as revoked
- Future communication rejected
- Historical messages remain valid

---

## 8. Device List Updates

Device changes are reflected via:

```text
New IdentityRecord with updated DeviceList
```

### Guarantees

- Device additions/removals are auditable
- No silent device injection

---

## 9. Domain Binding Integration

Domains may publish:

```rust
DomainRegistryRecord {
    Domain
    DIK_pub
    Signature
}
```

### Identity Binding

- IdentityRecord includes DomainSignature
- Clients verify domain trust separately

---

## 10. Consistency Model

Registry must provide:

- Strong consistency for writes
- Eventual consistency for reads

### Client Requirement

Clients must:

- Detect conflicting records
- Prefer highest-version valid record

---

## 11. Caching Strategy

Clients may cache:

- Identity records
- Inclusion proofs

### Cache Invalidation

Triggered by:

- Version mismatch
- Signature failure
- Periodic refresh

---

## 12. Security Properties

The identity registry ensures:

- Tamper-evident identity storage
- Verifiable key history
- Resistance to identity spoofing
- Transparent identity updates

---

## 13. Failure Scenarios

### Registry Compromise

Mitigation:

- Public transparency logs
- Multi-party verification
- Client-side validation

### Split View Attack

- Detected via Merkle root comparison
- Cross-client verification required

---

## 14. Legal & Trust Model

Registry operators:

- Do not control identity keys
- Cannot forge identities
- Provide verifiable data only

---

## 15. Summary

The SMP Identity Registry is:

- Append-only
- Cryptographically verifiable
- Transparent
- Resistant to tampering

It forms the foundation of identity trust in SMP.

---
