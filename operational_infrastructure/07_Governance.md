# Governance

This document defines the governance model of SMP, including control boundaries, protocol evolution, and trust assumptions.

SMP is designed to minimize central control while maintaining coordinated evolution of the protocol.

---

## 1. Governance Principles

- Decentralized trust enforcement (client-side)
- Minimal central authority
- Transparent protocol evolution
- Cryptographic verification over trust
- Separation of responsibilities

---

## 2. System Components & Control

SMP consists of multiple components with distinct control boundaries:

| Component         | Controlled By         | Role                                 |
| ----------------- | --------------------- | ------------------------------------ |
| Client            | User                  | Enforces trust, encryption, identity |
| Relay             | Operator              | Message transport & storage          |
| Identity Registry | Operator / Consortium | Identity records                     |
| Trust Engine      | Operator / Consortium | Global trust signals                 |

### Key Rule

No single component can compromise the entire system.

---

## 3. Trust Boundaries

### 3.1 Client (Trusted)

- Enforces encryption
- Verifies identities
- Computes trust decisions

### 3.2 Relay (Untrusted)

- Cannot access message content
- Cannot modify messages
- Only transports data

### 3.3 Registry Systems (Partially Trusted)

- Provide verifiable data
- Must be cryptographically auditable
- Cannot forge identities

---

## 4. Protocol Versioning

Each SMP message includes:

```text
Version
```

### Version Rules

- Clients must reject unsupported versions
- Backward compatibility preferred
- Breaking changes require version increment

---

## 5. Upgrade Mechanism

### Upgrade Process

1. New version proposed
2. Specification updated
3. Clients implement support
4. Gradual network adoption

### Deployment Model

- Rolling upgrades
- Multi-version support during transition
- No forced immediate migration

## 6. Feature Evolution

New features must:

- Be optional initially
- Be backward compatible
- Not break existing security guarantees

### Examples

- New encryption algorithms
- New friction types
- Enhanced metadata protection

## 7. Identity Governance

### Identity Ownership

- Fully controlled by user
- Keys generated client-side
- No central authority over identity

### Registry Role

- Stores identity records
- Does not control identities
- Must provide verifiable history

## 8. Trust Governance

### Global Trust (Trust Engine)

- Provides signals only
- Does not enforce decisions

### Local Trust (Client)

- Final authority
- User-controlled overrides allowed

## 9. Abuse Governance

### Reporting

- Users submit abuse reports
- Reports are signed and verifiable

### Enforcement

- Trust Engine aggregates signals
- Clients apply consequences

## 10. Economic Governance

### Stake System

- Optional participation
- Provides economic accountability

### Slashing

- Triggered by abuse signals
- Managed by Trust Engine

## 11. Operator Responsibilities

### Relay Operators

- Maintain uptime
- Enforce rate limits
- Respect zero-knowledge constraints

### Registry Operators

- Maintain integrity of records
- Provide transparency mechanisms
- Prevent tampering

## 12. Transparency Requirements

Operators should provide:

- Public documentation
- Verifiable logs (where applicable)
- Clear policies

## 13. Failure & Compromise Handling

### Relay Compromise

- No message exposure (encrypted)
- Temporary availability impact only

### Registry Compromise

- Detectable via signatures / logs
- Clients must verify data

### Trust Engine Manipulation

- Limited impact (client decides final trust)
- Detectable inconsistencies

## 14. Centralization Risks

Potential risks:

- Registry centralization
- Trust Engine bias
- Relay monopolization

### Mitigation

- Multiple registry providers (future)
- Client-side verification
- Open protocol specification

## 15. Legal Considerations

Operators can claim:

- No access to message content
- No control over user identity
- Limited role in trust decisions

## 16. Summary

SMP governance ensures:

- User-controlled identity and trust
- Minimal reliance on centralized authority
- Transparent protocol evolution
- Strong separation of responsibilities

The system is designed so that security is enforced by design, not by operator trust.

---
