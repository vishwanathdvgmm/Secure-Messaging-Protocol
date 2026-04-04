# Project Structure

This document defines the project structure for implementing SMP.

The structure is designed to:

- Map directly to protocol components
- Maintain separation of concerns
- Support scalability and testing
- Use Rust workspace architecture exclusively

---

## 1. High-Level Layout

```text
smp/
├── Cargo.toml              ← workspace root
├── crates/
│   ├── crypto/
│   ├── identity/
│   ├── session/
│   ├── trust/
│   ├── message/
│   ├── storage/
│   ├── registry_client/
│   └── relay_client/
├── bin/
│   ├── client/
│   ├── relay/
│   └── registry/
├── api/
│   └── proto/
├── configs/
├── tests/
└── docs/
```

---

## 2. Directory Breakdown

### 2.1 crates/ (Library Crates)

Core implementation. Each module is its own independent crate.

```text
crates/
├── crypto/                 ← Cargo.toml + src/lib.rs
├── identity/               ← Cargo.toml + src/lib.rs
├── session/                ← Cargo.toml + src/lib.rs
├── trust/                  ← Cargo.toml + src/lib.rs
├── message/                ← Cargo.toml + src/lib.rs
├── storage/                ← Cargo.toml + src/lib.rs
├── registry_client/        ← Cargo.toml + src/lib.rs
└── relay_client/           ← Cargo.toml + src/lib.rs
```

### Modules

| **Crate**         | **Maps To**           | **Type**    |
| ----------------- | --------------------- | ----------- |
| crypto            | Encryption Model      | Library     |
| identity          | Identity Model        | Library     |
| session           | Cryptographic Core    | Library     |
| trust             | Trust Model           | Library     |
| message           | Message Format + Flow | Library     |
| storage           | Storage Model         | Library     |
| registry_client   | Registry API (client) | Library     |
| relay_client      | Relay API (client)    | Library     |

### Rules

- NO monolithic crate
- NO mixing unrelated modules in one crate
- Clear boundaries between crates
- `registry_client` and `relay_client` are client-side ONLY
- Server logic MUST stay in `bin/relay` and `bin/registry`

### 2.2 bin/ (Executables)

Entry points for binaries.

```text
bin/
├── client/                 ← Cargo.toml + src/main.rs
│   └── src/
│       ├── main.rs
│       ├── tui/
│       │   ├── screens/
│       │   ├── controller/
│       │   └── components/
│       ├── state_provider.rs
│       └── action_handler.rs
├── relay/                  ← Cargo.toml + src/main.rs
│   └── src/
│       └── main.rs
└── registry/               ← Cargo.toml + src/main.rs
    └── src/
        └── main.rs
```

### Purpose

- `client/` → TUI client (ratatui + crossterm)
- `relay/` → Relay server (tonic gRPC)
- `registry/` → Identity registry server (tonic gRPC)

### 2.3 api/

API definitions.

```text
api/
└── proto/
    ├── relay.proto
    └── registry.proto
```

### Contents

- gRPC protobuf definitions
- Compiled via `tonic-build` in build scripts

### 2.4 configs/

Configuration files.

```text
configs/
├── relay.yaml
├── registry.yaml
└── client.yaml
```

### 2.5 tests/

Test suite.

```text
tests/
├── integration/
└── e2e/
```

Note: Unit tests live inside each crate (`#[cfg(test)]` modules).

### 2.6 docs/

```text
docs/
├── api_spec/
├── client_architecture/
├── core/
├── data_paths/
├── operational_infrastructure/
├── reference_implementation/
└── tui/
```

---

## 3. Module Dependency Rules

Strict rules:

```text
crypto            ← no dependencies (leaf crate)
identity          ← depends on crypto
session           ← depends on crypto
trust             ← depends on identity
message           ← depends on crypto + session
storage           ← independent (leaf crate)
relay_client      ← depends on message
registry_client   ← depends on identity
```

### Key Rule

```text
No circular dependencies allowed
```

---

## 4. Service Separation

### Client (bin/client)

- Uses: crypto, identity, session, trust, message, relay_client, registry_client
- Does NOT contain server-side logic

### Relay (bin/relay)

- Uses: storage
- Does NOT use crypto (except signature validation)
- Server-side relay logic ONLY

### Registry (bin/registry)

- Uses: identity, trust, storage
- Does NOT handle messages
- Server-side registry logic ONLY

---

## 5. Language & Runtime

### Rust (ONLY)

- `crates/` → library crates (`lib.rs`)
- `bin/` → binary crates (`main.rs`)
- Workspace managed via root `Cargo.toml`

### Async Runtime

- `tokio` for all async operations
- All network services MUST be async
- No blocking calls in async context

### Networking

- `tonic` for gRPC services
- `serde` for serialization

---

## 6. Build Targets

| **Target**        | **Crate Path**    | **Description**          |
| ----------------- | ----------------- | ------------------------ |
| smp-client        | bin/client        | TUI client               |
| smp-relay         | bin/relay         | Relay server             |
| smp-registry      | bin/registry      | Identity + trust service |

### Build Commands

```text
cargo build                     ← build all
cargo build -p smp-client       ← build client only
cargo build -p smp-relay        ← build relay only
cargo build -p smp-registry     ← build registry only
cargo test                      ← test all crates
cargo test -p smp-crypto        ← test crypto only
```

---

## 7. Scalability Considerations

- Each crate must be independently testable
- Services must be horizontally scalable
- Clear API boundaries required between crates
- No shared mutable state between services

---

## 8. Summary

The SMP project structure:

- Uses Rust workspace architecture exclusively
- Maps directly to protocol layers
- Separates concerns cleanly via independent crates
- Enforces strict dependency ordering
- Supports independent testing and building per crate

This structure ensures maintainable and scalable development.

---
