# Spam Resistance

This document defines how SMP prevents unsolicited and abusive messaging.

Unlike traditional systems that rely on filtering, SMP enforces **pre-delivery resistance mechanisms** using trust and economic cost.

---

## 1. Design Philosophy

Spam prevention is enforced through:

- Trust-based access control
- Economic friction for unknown senders
- Deterministic message acceptance rules

### Key Principle

> Sending messages must have a **cost proportional to lack of trust**.

---

## 2. Friction Model Overview

Each message includes a **Friction Block** that proves sender effort.

### Friction Types

| Type          | Description          |
| ------------- | -------------------- |
| Trusted       | No cost (high trust) |
| Proof-of-Work | Computational cost   |
| Stake         | Economic cost        |

---

## 3. Trust-Based Friction

Friction required is determined by trust score:

```text
RequiredFriction = f(RecipientTrustScore, SenderTrustScore)
```

### Behavior

| **Trust Level** | **Required Friction** |
| --------------- | --------------------- |
| High            | None                  |
| Medium          | Low PoW               |
| Low             | High PoW or Stake     |
| Very Low        | Message rejected      |

---

## 4. Proof-of-Work (PoW)

### Goal

Introduce computational cost per message.

### Structure

```text
Find nonce such that:
SHA256(header + nonce) < target
```

### Difficulty

```text
target = BASE_TARGET / Difficulty
```

### Verification

- Fast for recipient
- Expensive for sender

---

## 5. Adaptive Difficulty

Difficulty is dynamically adjusted:

```text
Difficulty =
    BaseDifficulty *
    (1 - SenderTrustScore/100)
```

### Implications

- Trusted users → near-zero cost
- Unknown users → significant cost
- Abusive users → extreme cost

---

## 6. Stake-Based Messaging

### Model

Sender locks value with message:

```text
StakeAmount → attached to message
```

### Outcomes

| **Outcome** | **Effect**                 |
| ----------- | -------------------------- |
| Accepted    | Stake returned             |
| Ignored     | Partial penalty (optional) |
| Reported    | Stake slashed              |

### Purpose

- Strong deterrent for spam
- Enables economic enforcement

---

## 7. Friction Block Integration

### Included in message

```text
FrictionBlock {
    Type
    Difficulty / Stake
    Nonce / Proof
}
```

### Validation

Recipient must:

- Verify PoW or stake
- Reject invalid friction

---

## 8. Rate Limiting

Relays enforce:

- Per-identity send rate
- Burst limits
- Per-recipient inflow caps

### Purpose

- Prevent DoS
- Prevent brute-force attempts

---

## 9. Message Request Queue Interaction

Unknown senders:

- Must pass friction requirements
- Messages routed to request queue

Without friction

```text
Message → rejected at relay or client
```

---

## 10. Abuse Escalation

Repeated abuse results in:

- Increased PoW difficulty
- Mandatory stake requirement
- Temporary send restrictions
- Trust score reduction

---

## 11. Sybil Resistance

To prevent fake identity abuse:

- Identity age weighting
- Stake requirement
- Trust accumulation over time

---

## 12. Trade-offs

| **Factor**      | **Impact**       |
| --------------- | ---------------- |
| Security        | Very high        |
| User friction   | Medium           |
| Compute cost    | Increased        |
| Spam resistance | Extremely strong |

---

## 13. Security Properties

The spam resistance model ensures:

- Economic infeasibility of mass spam
- Trust-aligned communication cost
- Resistance to bot attacks
- No reliance on content filtering

---

## 14. Summary

SMP prevents spam by:

- Requiring cost for unknown communication
- Adapting difficulty based on trust
- Enforcing economic penalties for abuse

Spam is not filtered — it is **structurally discouraged**.

---
