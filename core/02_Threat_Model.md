# <font color="red"> Threat Model </font>

This document defines the adversarial environment in which SMP operates and the security guarantees the protocol is designed to provide.

The threat model directly informs all architectural and cryptographic decisions.

---

## 1. <font color="cyan"> System Assumptions </font>

### 1.1 Relay Trust Model

Relays are considered **untrusted transport nodes**.

- Relays may be compromised
- Relays may attempt to inspect, modify, or drop messages
- Relays must not have access to plaintext message content

**Requirement:**
All confidentiality and integrity guarantees must be enforced end-to-end at the client level.

---

### 1.2 Client Device Trust Model

Client devices are considered **potentially compromised**.

- Malware or physical compromise is possible
- Private keys may be exposed temporarily

**Requirement:**

The system must provide:

- Forward secrecy
- Post-compromise security
- Key evolution
- Compromise containment

This necessitates the use of a **Double Ratchet mechanism**.

---

### 1.3 Network Adversary

The network is considered **fully observable**.

- Attackers can monitor traffic between clients and relays
- Packet timing, size, and routing metadata may be visible

**Requirement:**

- Transport encryption (TLS or equivalent)
- Minimize exposed metadata
- Prevent passive content inspection

---

## 2. <font color="magenta"> Metadata Protection Scope </font>

Full metadata anonymity (e.g., Tor-level) is **out of scope for initial design**.

### Supported:

- Encryption of message content (subject + body)
- Protection against passive network observers
- Limited exposure of identity hashes instead of raw identities

### Not Fully Addressed:

- Global traffic correlation attacks
- Full social graph obfuscation
- Mix-net or onion-routing level anonymity

**Rationale:**
Full metadata privacy introduces significant complexity and latency. SMP adopts a **privacy-centric but practical model**.

---

## 3. <font color="green"> Federation and Domain Isolation </font>

SMP supports a federated deployment model.

### Assumption:

- Individual domains may be compromised independently

### Requirement:

- No shared global secrets across domains
- Each domain maintains:
    - Independent trust anchors
    - Separate identity roots
    - Independent relay authentication

**Security Goal:**
A compromise in one domain must not affect the security of another domain.

---

## 4. <font color="yellow"> Adversary Classes </font>

### 4.1 Low-Skill Attackers

Capabilities:

- Basic spoofing attempts
- Phishing
- Credential guessing
- Simple spam tools

Mitigation:

- Mandatory cryptographic identity
- Signed messages only
- No password-only authentication

---

### 4.2 Large-Scale Spam Networks

Capabilities:

- Botnet-based message distribution
- Identity churn
- Automated content generation

Mitigation:

- Economic friction (proof-of-work / stake)
- Reputation-based trust scoring
- Rate limiting
- Abuse reporting mechanisms

---

### 4.3 Targeted Attackers (Corporate Espionage)

Capabilities:

- Targeted interception attempts
- Insider threats
- Key compromise attempts
- Metadata analysis

Mitigation:

- End-to-end encryption
- Forward secrecy
- Frequent key rotation
- Secure client-side key storage

---

### 4.4 State-Level Adversaries

Capabilities:

- ISP-level monitoring
- Legal coercion
- Infrastructure disruption
- Traffic analysis

Mitigation:

- Metadata minimization
- Encrypted message content (including subject)
- Federated deployment options
- Transparency-backed identity registry

Limitations:

- SMP does not guarantee full resistance to global passive adversaries
- Advanced anonymity systems (e.g., mixnets) are future extensions

---

## 5. <font color="grey"> Cryptographic Security Goals </font>

Based on the above threat model, SMP must guarantee:

### 5.1 Confidentiality

- Message content must remain inaccessible to relays and network observers
- Only intended recipient devices can decrypt messages

---

### 5.2 Integrity

- Messages must not be modifiable without detection
- All message components must be cryptographically authenticated

---

### 5.3 Authenticity

- Sender identity must be verifiable using cryptographic signatures
- Spoofing must be cryptographically infeasible

---

### 5.4 Forward Secrecy

- Compromise of long-term keys must not expose past messages

---

### 5.5 Post-Compromise Security

- After a compromise, future communications must recover security automatically

---

### 5.6 Replay Protection

- Replayed messages must be detectable and rejected

---

## 6. <font color="red"> Post-Quantum Considerations </font>

SMP adopts a **hybrid cryptographic strategy**.

- Classical cryptography (X25519, Ed25519) is used initially
- Protocol is designed to support post-quantum algorithms in future

**Requirement:**

- Cryptographic agility must be built into the protocol
- Transition to PQ algorithms must not require protocol redesign

---

## 7. <font color="cyan"> Out-of-Scope Threats </font>

The following are explicitly not addressed in the initial version:

- Full anonymity against global passive adversaries
- Endpoint compromise prevention
- Physical device security
- Side-channel attacks on client devices

---

## 8. <font color="blue"> Summary </font>

SMP assumes an adversarial environment where:

- Relays are untrusted
- Networks are observable
- Clients may be compromised
- Attackers range from low-skill to state-level

The protocol is designed to provide strong cryptographic guarantees under these conditions, while maintaining practical deployability.
