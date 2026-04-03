# SMP Development Enforcement Rules (Tool Contract)

This document defines mandatory rules that the tool MUST follow throughout the project lifecycle.

These rules are **strict, non-negotiable, and override any default behavior of the tool**.

---

# 1. SOURCE OF TRUTH

1. The documentation provided is the **only authoritative source**.
2. The tool MUST NOT:
    - Assume missing behavior
    - Infer undocumented features
    - Introduce external patterns not defined in docs
3. If ambiguity exists:
    - STOP execution immediately
    - Ask for clarification before proceeding

---

# 2. VERSION-LOCKED DEVELOPMENT

1. Development MUST strictly follow `implementation_plan.md`.
2. Only ONE version can be active at a time.
3. The tool MUST:
    - Explicitly state current version before coding
    - Implement ONLY features defined for that version
4. The tool MUST NOT:
    - Implement future-version features early
    - Skip versions
    - Merge multiple versions into one implementation

---

# 3. COMPLETION CRITERIA (PER VERSION)

Before marking a version complete, ALL must be satisfied:

- All features implemented exactly as per docs
- All mapped documentation fully covered
- No TODOs / placeholders / stubs
- No incomplete flows
- Build passes without warnings
- End-to-end flow works for that version
- Code reviewed against documentation

---

# 4. PRODUCTION CODE REQUIREMENT

The tool MUST generate `production-grade code only`.

Forbidden:

- Placeholder implementations
- Mock logic (unless explicitly instructed)
- Hardcoded shortcuts
- Temporary code
- Skeleton-only modules
- Incomplete functions

Required:

- Full error handling
- Input validation
- Proper logging (no sensitive data)
- Config-driven architecture
- Clean modular structure
- Consistent naming conventions
- Defensive programming

---

# 5. CRYPTOGRAPHY STRICTNESS

For all crypto-related code:

1. ONLY use primitives defined in docs
2. MUST NOT:
    - Modify algorithms
    - Simplify flows
    - Skip validation steps
    - Invent new crypto logic

Mandatory:

- Nonce uniqueness enforcement
- AEAD authentication validation (fail = reject)
- Cryptographically secure randomness only
- Proper key derivation (HKDF as defined)
- No key reuse across contexts
- No plaintext exposure at any stage

---

# 6. SESSION & STATE INTEGRITY

1. Session logic MUST follow Double Ratchet exactly
2. MUST NOT:
    - Skip ratchet steps
    - Ignore skipped message handling
3. State MUST:
    - Persist correctly
    - Be updated atomically
    - Never be partially written

Failure handling:

- Decryption failure → reject
- State mismatch → reset session

---

# 7. TRUST & SECURITY ENFORCEMENT

1. Trust decisions MUST follow docs strictly
2. MUST NOT:
    - Bypass trust evaluation
    - Deliver untrusted messages directly to inbox

Mandatory:

- Unknown sender → Message Request Queue
- Trust level always evaluated before display

---

# 8. TUI CONSTRAINTS

1. TUI MUST NOT:
    - Access private keys
    - Perform cryptographic operations
    - Modify core state directly

2. TUI MUST:
    - Use StateProvider (read-only)
    - Use ActionHandler (write via core)
    - Reflect actual system state

---

# 9. API & NETWORK RULES

1. All APIs MUST follow defined specs
2. MUST NOT:
    - Change request/response formats
    - Add undocumented fields

Mandatory:

- Signature verification
- Timestamp validation
- Proper error responses

---

# 10. STORAGE RULES

1. Storage MUST be:

- Opaque (no parsing message content)
- Immutable (no modification after write)

2. MUST NOT:

- Store plaintext
- Index message content
- Build communication graphs

3. Deletion MUST be:

- Permanent
- Non-recoverable

---

# 11. ERROR HANDLING POLICY

The tool MUST:

- Fail explicitly, never silently
- Return meaningful errors
- Never ignore cryptographic or validation errors

Rule:

```
If validation fails → reject immediately
```

---

# 12. LOGGING POLICY

Allowed:

- Errors
- System events
- Non-sensitive metadata

Forbidden:

- Private keys
- Message content
- Full identity exposure

---

# 13. TESTING REQUIREMENT

For each version:

- Unit tests for core modules
- Integration test for message flow
- Edge case handling (failures, invalid inputs)

The tool MUST NOT mark version complete without testing.

---

# 14. GIT & VERSION CONTROL RULES

After completing each version:

1. Tool MUST:

- Ask: “Version vX.X complete. Proceed to commit and push?”
- Wait for confirmation

2. On approval:

- Create commit with message:

```
  vX.X: <short description>
```

3. For major versions (v1.0, v2.0, v1.5, v2.5):

- Create git tag:

```
  vX.X
```

4. MUST NOT:

- Auto-push without confirmation
- Skip version commits

---

# 15. NO SHORTCUT POLICY

The tool MUST NOT:

- Optimize prematurely
- Skip steps for speed
- Collapse architecture layers

Rule:

```
Correctness > Speed
```

---

# 16. STRICT EXECUTION FLOW

For every version:

```
Read docs → Implement → Validate → Test → Confirm → Commit → Next version
```

---

# 17. ESCALATION RULE

If ANY of the following occurs:

- Missing definition
- Conflicting docs
- Unclear behavior

The tool MUST:

- STOP
- Ask user for clarification
- Resume only after confirmation
