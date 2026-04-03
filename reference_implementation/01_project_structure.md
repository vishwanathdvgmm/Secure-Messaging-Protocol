# Project Structure

This document defines the recommended project structure for implementing SMP.

The structure is designed to:

- Map directly to protocol components
- Maintain separation of concerns
- Support scalability and testing

---

## 1. High-Level Layout

```text
smp/
├── cmd/
├── internal/
├── pkg/
├── api/
├── configs/
├── scripts/
├── tests/
└── docs/
.gitignore
README.md
```

---

## 2. Directory Breakdown

### 2.1 cmd/

Entry points for binaries.

```text
cmd/
├── client/
    ├── main.go
    ├── tui/
        ├── screens/
        ├── controller/
        ├── components/
├── relay/
├── registry/
```

### Purpose

- `client/` → CLI or app entry
- `relay/` → relay server
- `registry/` → identity + trust services

### 2.2 internal/

Core implementation (private modules).

```text
internal/
├── crypto/
├── identity/
├── session/
├── trust/
├── message/
├── transport/
├── storage/
```

### Modules

| **Module** | **Maps To**           |
| ---------- | --------------------- |
| crypto     | Encryption Model      |
| identity   | Identity Model        |
| session    | Cryptographic Core    |
| trust      | Trust Model           |
| message    | Message Format + Flow |
| transport  | API + Relay           |
| storage    | Storage Model         |

### 2.3 pkg/

Reusable public libraries (optional).

```text
pkg/
├── smp_protocol/
├── smp_client/
```

### Purpose

- Shared logic across services
- External integrations

### 2.4 api/

API definitions.

```text
api/
├── rest/
├── grpc/
```

### Contents

- REST schemas
- gRPC proto files

### 2.5 configs/

Configuration files.

```text
configs/
├── relay.yaml
├── registry.yaml
├── client.yaml
```

### 2.6 scripts/

Automation scripts.

```text
scripts/
├── build.sh
├── run.sh
```

### 2.7 tests/

Test suite.

```text
tests/
├── unit/
├── integration/
├── e2e/
```

### 2.8 docs/

```text
docs/
├── api_spec
├── client_architecture
├── core
├── data_paths
├── operational_infrastructure
├── reference_implementation
```

---

## 3. Module Dependency Rules

Strict rules:

```text
crypto ← no dependencies
identity ← depends on crypto
session ← depends on crypto
trust ← depends on identity
message ← depends on crypto + session
transport ← depends on message
storage ← independent
```

### Key Rule

```text
No circular dependencies allowed
```

---

## 4. Service Separation

### Client

- Uses all modules except relay/storage backend

### Relay

- Uses transport + storage
- Does NOT use crypto (except validation)

### Registry

- Uses identity + trust modules
- Does NOT handle messages

---

## 5. Language Mapping

### Go

- `internal/` → private packages
- `cmd/` → main packages
- `pkg/` → reusable modules

### Rust

- `crate` structure per module
- `bin/` for executables
- `lib/` for shared logic

---

## 6. Build Targets

| **Target** | **Description**          |
| ---------- | ------------------------ |
| client     | SMP client               |
| relay      | Relay server             |
| registry   | Identity + trust service |

---

## 7. Scalability Considerations

- Modules must be independently testable
- Services must be horizontally scalable
- Clear API boundaries required

---

## 8. Summary

The SMP project structure:

- Maps directly to protocol layers
- Separates concerns cleanly
- Supports both Go and Rust implementations

This structure ensures maintainable and scalable development.

---
