# Trust Model

This document defines how SMP evaluates, manages, and enforces trust between identities.

Unlike traditional systems that rely on heuristic spam filters, SMP uses a **deterministic, cryptographic, and economic trust model**.

---

## 1. Trust Model Overview

In SMP, trust is **not implicit**.

Every incoming message is evaluated based on:

- Identity authenticity
- Reputation signals
- User interaction history
- Economic friction attached to the message

---

## 2. Trust Layers

Trust is computed from two independent layers:

### 2.1 Global Trust (Registry-Level)

Provided by the identity registry.

```rust
RegistryTrustRecord {
    IdentityHash
    IdentityAge
    DomainVerified
    DomainReputationScore
    AbuseReportsCount
    StakeSlashedEvents
    LastActivityTimestamp
}
```

### 2.2 Local Trust (Client-Level)

Maintained by each client.

```rust
LocalTrustRecord {
    FirstInteractionTimestamp
    SuccessfulInteractions
    MarkedImportant
    MarkedSpam
    ManualWhitelist
    ManualBlacklist
}
```

---

## 3. Trust Score Compuataion

Final trust score:

```plaintext
FinalTrustScore =
    (0.6 * GlobalTrust)
  + (0.4 * LocalTrust)
```

**Adjustable Policy**

- Enterprise: 0.8 Global + 0.2 Local
- Personal: 0.4 Global + 0.6 Local

---

## 4. Trust Levels (User-Visible)

Each identity falls into a trust category:

| **Level** | **Name**    | **Description**                   |
| --------- | ----------- | --------------------------------- |
| 🔵        | QR Verified | Verified via QR / secure exchange |
| 🟢        | Accepted    | Manually accepted request         |
| 🟡        | Unknown     | Not yet trusted                   |
| 🔴        | Suspicious  | Identity change / abuse detected  |

---

## 5. Inbox Routing Model

Messages are routed based on trust score:

| **Tier** | **Score** | **Behavior**   |
| -------- | --------- | -------------- |
| Tier 0   | 85–100    | Primary inbox  |
| Tier 1   | 70–85     | Verified inbox |
| Tier 2   | 50–70     | New / unknown  |
| Tier 3   | 30–50     | Review queue   |
| Tier 4   | <30       | Quarantine     |

**Important Rule**

- No message is sliently deleted
- All mess are visible but segregated

---

## 6. Message Acceptance Flow

### 6.1 Unknown Sender

- Message → Message Request Queue
- No automatic trust

### 6.2 User Decision

User may:

- Accept → trust established
- Ignore → discard
- Block → future messages rejected

### 6.3 After Acceptance

- Identity is pinned
- Trust level increases
- Messages move to primary inbox

---

## 7. Economic Friction Model

To prevent spam, SMP introduces cost to sending messages.

### 7.1 Friction Types

| **Type**      | **Description**    |
| ------------- | ------------------ |
| Proof-of-Work | Computational cost |
| Stake         | Economic deposit   |
| Trusted       | No friction        |

### 7.2 Adaptive Difficulty

```plaintext
Difficulty = BaseDifficulty * (100 - TrustScore)/100
```

**Behavior**

- High trust → no cost
- Medium trust → light cost
- Low trust → high cost

---

## 8. Abuse Reporting

Users can report malicious senders:

```rust
AbuseReport {
    SenderHash
    MessageHash
    Signature(SK_priv)
}
```

**Effects**

- Affects global trust score
- Weighted by reporter credibility
- Requires threshold to trigger penalties

---

## 9. Stake Slashing

If sender uses stake:

- Malicious behavior → stake reduced
- Registry updates trust score

---

## 10. Trust Persistence

Trust is identity-bound, not session-bound.

- Survives across sessions
- Survives device changes
- Stored locally and optionally synced securely

---

## 11. Trust Decay

Optional mechanism:

- Inactive identities lose trust over time
- Prevents stale trust exploitation

---

## 12. Identity Change Handling

If identity changes (key rotation anomaly):

- Mark as Suspicious
- Require re-verification
- Notify user explicitly

---

## 13. Security Properties

The trust model ensures:

- Spam resistance at protocol level
- Deterministic inbox control
- User-controlled communication boundaries
- Resistance to spoofing and impersonation
- Economic deterrence for abuse

---

## 14. Summary

SMP replaces heuristic spam filtering with a structured trust system.

This enables:

- Controlled communication
- Transparent trust decisions
- Strong resistance to abuse

Trust is computed, not assumed.

---
