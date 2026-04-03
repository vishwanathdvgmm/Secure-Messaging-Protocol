# Secure Messaging Protocol (SMP)

**A next-generation email architecture designed to replace SMTP with a secure, identity-based, and spam-resistant communication system.**

---

## 1. Core Structural Problems:

Traditional email system (SMTP-based) suffer from fundamental architecture weaknesses:

1. **No built-in end-to-end encryption:**
    - TLS provides only hop-to-hop security.
    - Mail servers can access plaintext messages..

2. **Identity is weak:**
    - Email address are not cryptographic identities.
    - Sender spoofing remains possible.

3. **Spam architecture:**
    - Any sender can contact any recipient.
    - No inherent cost or cryptographic gating.

4. **Metadata leakage:**
    - Sender, recipient, and subject are visible.
    - Enables traffic analysis and profiling.

5. **No forward secrecy:**
    - Compromise of keys exposes historical messages.

6. **No cryptographic inbox control:**
    - Recipient cannot enfore sender authenticity at protocol level.

7. **Legacy compatibility constraints:**
    - Backward compatibility limits security evolution.

---

## 2. Proposed Solutions:

SMP introduces the following core improvements:

- End-to-End Encryption by default.
- Cryptographic identity (eliminates spoofing).
- Forward secrecy and post-compromise security.
- Spam resistance via economic friction mechanisms.
- Metadata minimization.
- Trust-aware message routing.

---

## 3. Core Design Principles:

The protocol is built on the following principles:

- **Zero-trust relay infrastructure**  
  Relays are treated as untrusted transport nodes

- **Client-side cryptographic enforcement**  
  All security guarantees originate from the client

- **Identity-first communication model**  
  Communication is bound to cryptographic identity, not addresses

- **Economic resistance to abuse**  
  Unsolicited communication incurs computational or economic cost

- **Minimal metadata exposure**  
  Only essential routing metadata is visible to relays

- **End-to-end verifiability**  
  All critical operations are cryptographically verifiable

---

## 4. High-Level Architecture:

SMP is structured into the following layers:

1. **Identity Layer**  
   Cryptographic identities and key hierarchy

2. **Trust & Reputation Layer**  
   Trust scoring and sender evaluation

3. **Encryption Layer**  
   Asynchronous encryption and forward secrecy

4. **Session Layer**  
   Optional real-time communication using ratcheted sessions

5. **Relay & Storage Layer**  
   Zero-knowledge message transport and storage

---

## 5. Communication Model:

SMP operates in two modes:

- **Asynchronous Mode (default)**  
  Store-and-forward messaging using pre-key based encryption

- **Session Mode**  
  Real-time communication using a Double Ratchet mechanism

Session mode is an extension of the asynchronous protocol, not a replacement.

---

## 6. Key Innovations:

- **Cryptographic inbox control**  
  Messages are accepted based on verifiable identity and policy

- **Trust-tiered inbox architecture**  
  Messages are routed based on computed trust levels

- **Economic anti-spam model**  
  Proof-of-work or stake introduces cost to unsolicited messages

- **Device-level encryption model**  
  Messages are encrypted per recipient device

- **Transparency-backed identity registry**  
  Identity changes are publicly auditable

---

## 6. Project Scope:

This project focuses on:

- Protocol design.
- Cryptographic architecture.
- System-level design.

Not included in the intial phase:

- User interface (UI/UX).
- Production deployment infrastructure.

---

## 7. Documentation Map:

- Core/<br>
  Foundational models, threat model, and cryptographic design.
- Data_Paths/<br>
  Protocol flows, handshake mechanisms, and packet formats.
- Operational_Infrastructure/<br>
  Relay architecture, storage systems, and identity registry.

---
