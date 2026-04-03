# SMP Implementation Plan

This document defines a phased implementation roadmap for SMP.

Each version:

- Introduces specific capabilities
- Maps directly to documentation files
- Builds incrementally toward full system

---

# VERSIONING STRATEGY

- v0.x → Foundation + core crypto + minimal messaging
- v1.x → Stable messaging system
- v2.x → Trust, spam resistance, and production features

---

# v0.1 — Project Bootstrap

## Goal
Basic project skeleton + crypto primitives

## Implement

- Project structure
- Crypto module (basic primitives)

## Docs to Follow

- Reference_Implementation/01_Project_Structure.md
- Reference_Implementation/02_Crypto_Module.md

## Output

- Buildable project
- Working crypto functions

---

# v0.2 — Identity Core

## Goal
Implement identity system

## Implement

- Identity key generation
- IdentityObject creation
- Identity hash generation
- Username format `<base>#<6hex>`

## Docs to Follow

- Core/Identity_Model.md
- Reference_Implementation/02_Crypto_Module.md

## Output

- Identity creation CLI/API
- Identity serialization

---

# v0.3 — Registry (Basic)

## Goal
Store and retrieve identities

## Implement

- Identity registry service
- Identity fetch API

## Docs to Follow

- Operational_Infrastructure/Identity_Registry.md
- API_Spec/04_Registry_API.md

## Output

- GET /identity/{hash} working
- Identity persistence

---

# v0.4 — Pre-Key System

## Goal
Enable async communication

## Implement

- Pre-key bundle generation
- Publish/fetch pre-keys

## Docs to Follow

- Core/Identity_Model.md (PreKeyBundle)
- API_Spec/01_REST_API.md

## Output

- Pre-key publish + fetch working

---

# v0.5 — Session Manager (Initial)

## Goal
Basic session establishment

## Implement

- Pre-key handshake
- Root key derivation
- Initial session struct

## Docs to Follow

- Data_Paths/Session_Model.md
- Reference_Implementation/03_Session_Manager.md

## Output

- Session creation working

---

# v0.6 — Message Encryption

## Goal
Send encrypted messages

## Implement

- Message encryption
- AEAD integration
- Packet construction (basic)

## Docs to Follow

- Core/Encryption_Model.md
- Data_Paths/Message_Format.md
- Reference_Implementation/02_Crypto_Module.md

## Output

- Encrypted message generation

---

# v0.7 — Relay Service (Basic)

## Goal
Transport messages

## Implement

- gRPC RelayService
- SendMessage
- FetchMessages

## Docs to Follow

- API_Spec/03_Relay_API.md
- Reference_Implementation/04_Relay_Service.md

## Output

- Messages stored and retrieved

---

# v0.8 — End-to-End Messaging + Basic TUI

## Goal
First usable system

## Implement

### Messaging
- Send → Relay → Receive
- Decryption pipeline

### TUI (Minimal)
- Inbox screen
- Message view
- Send message input

## Docs

### Messaging
- Data_Paths/Message_Flow.md
- Client_Architecture/04_Message_Handling.md

### TUI
- TUI/01_Overview.md
- TUI/02_Screens_and_Flows.md
- TUI/04_Implementation.md

---

# v0.9 — Double Ratchet + TUI Stability

## Implement

### Crypto
- Full Double Ratchet
- Skipped messages

### TUI Improvements
- Real-time updates (event-driven)
- Error display
- Basic navigation stability

## Docs

### Crypto
- Core/Cryptographic_Core.md
- Reference_Implementation/03_Session_Manager.md

### TUI
- TUI/03_State_Integration.md
- TUI/04_Implementation.md

---

# v1.0 — Stable Messaging Release (WITH TUI)

## Includes

- Full messaging system
- Working TUI client
- Stable sessions

## Docs

- ALL Core/
- ALL Data_Paths/
- Client_Architecture/*
- TUI/*

---

# v1.1 — Storage & Retention

## Docs

- Operational_Infrastructure/Storage_Model.md
- Reference_Implementation/05_Storage_Service.md

---

# v1.2 — Multi-Device + TUI Sync

## Implement

### Core
- Multi-device sessions

### TUI
- Device-aware messaging
- Sync updates across sessions

## Docs

- Data_Paths/Multi_Device.md
- Client_Architecture/02_State_Management.md
- TUI/03_State_Integration.md

---

# v1.3 — Authentication Layer

## Docs

- API_Spec/02_Auth_Flow.md

---

# v1.4 — Client Trust Logic + TUI Trust UI

## Implement

### Core
- Trust tiers
- Request queue

### TUI
- Trust indicators (QR / Accepted / Unknown)
- Request screen (Accept / Ignore / Block)

## Docs

- Core/Trust_Model.md
- Client_Architecture/03_Security_Model.md
- TUI/02_Screens_and_Flows.md

---

# v1.5 — Identity UX Enhancements

## Implement

### Core
- Username search (opt-in)

### TUI
- Fingerprint display
- Profile screen
- Identity verification UI

## Docs

- Core/Identity_Model.md
- TUI/02_Screens_and_Flows.md

---

# v2.0 — Trust Engine Integration

## Docs

- Operational_Infrastructure/Trust_Engine.md
- API_Spec/04_Registry_API.md

---

# v2.1 — Spam Resistance (Core)

## Docs

- Operational_Infrastructure/Spam_Resistance.md
- Data_Paths/Message_Format.md

---

# v2.2 — Stake & Slashing

## Docs

- Trust Engine doc
- Spam Resistance doc

---

# v2.3 — Metadata Protection

## Docs

- Operational_Infrastructure/Metadata_Protection.md

---

# v2.4 — Governance & Versioning

## Docs

- Operational_Infrastructure/Governance.md

---

# v2.5 — Performance & Scaling + TUI Optimization

## Implement

### Backend
- Scaling
- Load balancing

### TUI
- Efficient rendering
- Reduced redraw
- Performance tuning

## Docs

- Relay + Storage docs
- TUI/04_Implementation.md

---

# FINAL STATE

At v2.5:

- Full secure messaging system
- Functional TUI client
- Trust + spam resistance
- Privacy-preserving infrastructure

---

# DEVELOPMENT RULES

1. Never skip versions
2. Validate TUI alongside backend (v0.8 onwards)
3. Crypto must stabilize before UI complexity
4. Do not implement trust/spam before messaging is stable
5. Always follow document mappings strictly

---

# SUMMARY

This plan now includes:

- Backend system
- Client architecture
- **Usable TUI from v0.8 onwards**

This ensures you can:

- Test early
- Debug easily
- Iterate safely