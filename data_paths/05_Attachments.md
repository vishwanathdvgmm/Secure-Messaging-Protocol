# Attachments

This document defines how attachments are handled in SMP.

Attachments are treated as **encrypted data objects**, separate from message transport, while maintaining end-to-end security.

---

## 1. Design Principles

- Attachments must be encrypted client-side
- Relays must not access attachment content
- Large files must not inflate message packets
- Attachments must be integrity-protected
- Storage must remain zero-knowledge

---

## 2. Attachment Types

SMP supports two attachment modes:

| Type                  | Use Case    |
| --------------------- | ----------- |
| Inline Attachments    | Small files |
| Offloaded Attachments | Large files |

---

## 3. Inline Attachments

### Use Case

- Images
- Small documents
- Files below threshold (e.g., ≤ 10–50 MB)

---

### Structure

Inline attachments are embedded inside the encrypted payload:

```rust
EncryptedPayload {
    Subject
    Body
    InlineAttachments [
        { Filename, MIME, BinaryData }
    ]
}
```

### Properties

- Encrypted with message key
- No external storage required
- Simple retrieval

### Limitations

- Increases message size
- Higher relay storage usage
- Not suitable for large files

---

## 4. Offloaded Attachments

### Use Case

- Videos
- Archives
- Large documents
- Enterprise file exchange

---

## 5. Offloaded Attachment Flow

### Step 1 — Generate Attachment Key

```rust
AttachmentKey = random(256-bit)
```

### Step 2 — Encrypt File

```rust
EncryptedFile = AEAD_Encrypt(AttachmentKey, FileData)
```

### Step 3 — Upload to Storage

- Upload encrypted file to object storage node
- Receive:

```rust
AttachmentID
```

### Step 4 — Embed Reference in Message

Inside encrypted payload:

```rust
AttachmentReference {
    AttachmentID
    EncryptedAttachmentKey
    FileHash
    Size
}
```

### Step 5 — Key Wrapping

```rust
EncryptedAttachmentKey = Encrypt(SessionKey, AttachmentKey)
```

### Step 6 — Recipient Retrieval

Recipient:

1. Decrypts message
2. Extracts AttachmentKey
3. Downloads encrypted file
4. Decrypts file locally

---

## 6. Integrity Protection

Each attachment includes:

```text
FileHash = SHA-256(EncryptedFile)
```

### Verification

Recipient must:

- Compute hash after download
- Compare with expected value

### Protection Against

- Storage tampering
- Corruption
- Malicious replacement

---

## 7. Storage Node Responsibilities

### Storage node must:

- Store encrypted blobs
- Enforce size limits
- Provide retrieval APIs
- Enforce retention policies

### Storage node must NOT:

- Decrypt files
- Modify encrypted data
- Access file content

---

---

## 8. Retention Policy

Attachments follow:

- Message retention policy<br>
  or
- Attachment-specific policy

### Examples

| **Policy**          | **Behavior** |
| ------------------- | ------------ |
| Delete with message | Immediate    |
| Extended archive    | 1 year       |
| Enterprise hold     | Configurable |

---

## 9. Multi-Device Behavior

Each device receives:

```text
AttachmentKey encrypted per device
```

### Implication

- Each device can decrypt independently
- Device revocation remains effective

---

## 10. Abuse Considerations

Risks:

- Storage flooding
- Large file abuse
- Hidden malicious content

### Mitigation:

- Rate limits
- File size caps
- Increased friction for large uploads
- Metadata-based filtering (size/type only)

---

## 11. Packet Integration

Message payload includes:

```rust
InlineAttachmentCount
InlineAttachmentBlocks
OffloadedAttachmentCount
OffloadedAttachmentReferences
```

All encrypted inside payload.

---

## 12. Security Properties

The attachment model ensures:

- End-to-end confidentiality
- Storage-layer zero knowledge
- Integrity verification
- Independent access control
- Scalability for large data

## 13. Summary

SMP separates attachment handling from message transport while preserving security guarantees.

- Small files → inline encryption
- Large files → encrypted external storage

All attachments remain encrypted end-to-end and verifiable upon retrieval.

---
