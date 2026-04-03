# Trust Engine

This document defines how SMP computes, maintains, and distributes trust signals across the network.

The Trust Engine provides **global trust inputs** that are combined with local client trust to determine message acceptance and routing.

---

## 1. Role of the Trust Engine

The Trust Engine is responsible for:

- Aggregating global behavior signals
- Maintaining trust-related data for identities
- Processing abuse reports
- Tracking stake and penalties
- Providing trust inputs to clients

### Key Principle

The Trust Engine does NOT make final decisions.

- It provides **signals**
- Clients compute final trust locally

---

## 2. Trust Data Model

Each identity has an associated trust record:

```rust
TrustRecord {
    IdentityHash
    IdentityAge
    DomainVerified
    DomainReputation
    AbuseReportsCount
    WeightedAbuseScore
    StakeAmount
    SlashingHistory
    LastActivityTimestamp
}
```

### Notes:

- No message content is stored.
- Only aggregated behavioral signals are tracked.

---

## 3. Data Sources

Trust signals are derived from:

### 3.1 Identity Registry

- Identity age
- Key stability
- Device consistency

### 3.2 User Reports

```rust
AbuseReport {
    ReporterIdentityHash
    TargetIdentityHash
    MessageHash
    ReasonCode
    Signature
}
```

### 3.3 Economic Signals

- Stake attached to identity
- Slashing events
- Stake history

### 3.4 Activity Signals

- Message frequency
- Interaction success rate
- Long-term usage patterns

---

## 4. Abuse Report Processing

### Validation

Each report must:

- Be signed by reporter
- Reference a valid message
- Not be duplicate

### Weighting

Reports are weighted by:

```text
ReportWeight = f(ReporterTrustScore, InteractionHistory)
```

### Aggregation

```text
WeightedAbuseScore += ReportWeights
```

---

## 5. Threshold-Based Effects

When abuse crosses hresholds:

| **Level** | **Effect**                     |
| --------- | ------------------------------ |
| Low       | Minor trust reduction          |
| Medium    | Increased friction requirement |
| High      | Temporary restrictions         |
| Critical  | Identity flagged               |

---

## 6. Stake Model

### Purpose

- Introduce economic accountability
- Increase baseline trust

### Structure

```text
StakeAmount → locked value
```

### Behavior

- Higher stake → higher baseline trust
- Stake is optional

---

## 7. Slashing Mechanism

When abuse is confirmed:

```text
StakeAmount -= Penalty
```

### Effects

- Immediate trust reduction
- Economic loss for sender
- Visible slashing history

---

## 8. Domain Reputation

Domains maintain their own reputation:

```rust
DomainTrustRecord {
    Domain
    ReputationScore
    VerifiedUsers
    AbuseIncidents
}
```

### Impact

- Domain-verified identities inherit trust
- Poor domain reputation reduces baseline trust

---

## 9. Trust Decay

Inactive identities lose trust over time:

```text
TrustScore -= decay_factor(time_inactive)
```

### Purpose

- Prevent domant identity abuse
- Keep trust dynamic

---

## 10. Data Distribution

Clients fetch:

```text
TrustRecord
```

### Usage

Client combines:

- Global trust (this engine)
- Local trust (user behavior)

---

## 12. Privacy Constraints

Trust Engine must NOT:

- Store message content
- Build communication graphs
- Expose user relationships

### Allowed Data

- Aggregated metrics only
- No direct identity graph exposure

---

## 12. Consistncy Model

- Strong consistency for updates
- Eventual consistency for reads

### Client Responsibility

- Handle temporary inconsistencies
- Prefer latest valid data

---

## 13. Failure Scenarios

### False Reports

Mitigation:

- Weighted reporting
- Threshold requirements
- Reputation credibility

### Sybil Attacks

Mitigation:

- Identity age weighting
- Stake requirments
- Rate limits

---

## 14. Security Properties

The Trust Engine ensures:

- Network-wide abuse resistance
- Economic deterrence
- Adaptive trust signals
- Resistance to manipulation

---

## 15. Summary

The SMP Trust Registry provides:

- Global reputation signals
- Abuse tracking
- Economic enforcement mechanisms

It supports the Trust Model without centralizing decision-making.

---
