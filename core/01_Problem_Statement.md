# <font color="red"> Problem Statement </font>

Traditional email systems based on SMTP are fundamentally misaligned with modern security and privacy requirements. These limitations are not incidental—they are structural and cannot be fully resolved through incremental improvements.

This section defines the core architectural deficiencies that necessitate a new protocol design.

---

## 1. <font color="cyan"> Lack of End-to-End Encryption </font>

SMTP does not provide native end-to-end encryption.

- TLS is used only for hop-to-hop transport security
- Intermediate servers can access message plaintext
- Messages are stored in readable form on servers

**Implication:**
Confidentiality depends on server trust, which violates zero-trust security principles.

---

## 2. <font color="magenta"> Absence of Cryptographic Identity </font>

Email addresses are not cryptographic identities.

- No inherent binding between sender and a cryptographic key
- Spoofing is possible without breaking cryptography
- Identity verification relies on external mechanisms (SPF, DKIM, DMARC)

**Implication:**
Authenticity is probabilistic, not cryptographically guaranteed.

---

## 3. <font color="green"> Open Sender Model (Spam by Design) </font>

SMTP allows unrestricted sender-to-recipient communication.

- Any sender can deliver messages to any recipient
- No protocol-level cost or gating mechanism
- Spam mitigation is reactive (filters), not preventive

**Implication:**
Spam is an emergent property of the protocol, not an anomaly.

---

## 4. <font color="blue"> Metadata Exposure </font>

Email systems expose critical metadata in plaintext.

- Sender and recipient addresses
- Subject lines
- Routing headers

**Implication:**
Even when content is encrypted (e.g., PGP), communication patterns remain observable, enabling traffic analysis and profiling.

---

## 5. <font color="yellow"> No Forward Secrecy </font>

SMTP-based systems do not enforce forward secrecy.

- Long-term keys (if used) can decrypt historical messages
- Message encryption schemes (e.g., PGP) lack mandatory key evolution

**Implication:**
A single key compromise can expose entire communication history.

---

## 6. <font color="grey"> Lack of Cryptographic Inbox Control </font>

Recipients cannot enforce sender authenticity at the protocol level.

- Message acceptance is based on server-side filtering
- No cryptographic policy enforcement before delivery
- No deterministic trust evaluation during message receipt

**Implication:**
Recipients are passive; control is delegated to heuristics and server logic.

---

## 7. <font color="red"> Legacy Compatibility Constraints </font>

SMTP must maintain backward compatibility with decades-old infrastructure.

- Security improvements must remain optional
- Strong guarantees cannot be enforced globally
- Innovation is constrained by interoperability requirements

**Implication:**
The protocol cannot evolve into a secure system without breaking compatibility.

---

## 8. <font color="cyan"> Summary </font>

These limitations demonstrate that SMTP is not a secure messaging protocol but a best-effort delivery system with optional security layers.

A modern communication system requires:

- Cryptographic identity as a foundation
- End-to-end encryption by default
- Forward secrecy and key evolution
- Sender accountability mechanisms
- Minimal metadata exposure
- Recipient-controlled message acceptance

SMP is designed to address these requirements at the protocol level, rather than as optional extensions.
