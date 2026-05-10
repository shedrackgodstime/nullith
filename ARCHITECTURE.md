# Nullith - Foundation Architecture

## Current Status (v1.0)

- ✓ Cloudflare Worker (Rust)
- ✓ D1 Database (Notes)
- ✓ R2 Bucket (created, not integrated)
- ✓ GitHub Actions auto-deploy

---

## Foundation for Scalability

### 1. Authentication Layer

```
Current: No auth (open API)
         ┌─────────────┐
         │   Request   │
         └──────┬──────┘
                ↓
         ┌──────────────┐
         │ API Key Auth │ (add now)
         └──────┬───────┘
                ↓
         ┌──────────────┐
         │   Validate   │
         └──────┬───────┘
                ↓
         ┌──────────────┐
         │   Business   │
         │    Logic     │
         └──────────────┘
```

**Required:**
- API key validation on every request
- Keys stored in D1 (hashed)
- Rate limiting per key

### 2. Encryption Layer

```
Client (encrypt) ──→ Server (storage) ──→ Client (decrypt)
                         ↓
                   Server NEVER
                   sees plaintext
```

**Required:**
- Client-side encryption before upload
- User provides encryption key locally
- Server stores encrypted blobs

### 3. Data Organization

```
user_id/
├── files/
│   ├── metadata (D1)
│   └── blobs (R2)
├── notes/
│   └── data (D1)
├── shares/        # Quick share links
│   └── metadata
└── keys/          # API keys
    └── metadata
```

### 4. Client Architecture (For Future)

```
┌────────────────────────────────────────────┐
│                  CLI (Rust)                 │
│  ┌────────┐  ┌────────┐  ┌────────────┐  │
│  │ Upload │  │  Notes │  │  Encrypt   │  │
│  └────────┘  └────────┘  └────────────┘  │
└────────────────────────────────────────────┘

┌────────────────────────────────────────────┐
│             Android (Kotlin)               │
│  ┌────────┐  ┌────────┐  ┌────────────┐  │
│  │ Sync   │  │  UI    │  │  Crypto    │  │
│  └────────┘  └────────┘  └────────────┘  │
└────────────────────────────────────────────┘

┌────────────────────────────────────────────┐
│                Web (WASM)                   │
│  ┌────────┐  ┌────────┐  ┌────────────┐  │
│  │ SPA    │  │  API   │  │  Encrypted │  │
│  └────────┘  └────────┘  └────────────┘  │
└────────────────────────────────────────────┘
```

---

## Phase 2: Foundation (Next)

### Priority Order:

1. **API Key Authentication**
   - Generate keys for clients
   - Validate on every request
   - Rate limiting

2. **Client-Side Encryption**
   - Key derivation (Argon2id)
   - Encryption (XChaCha20-Poly1305)
   - Encrypt before upload

3. **User Isolation**
   - Add user_id to all data
   - Multi-user support

4. **File Upload Integration**
   - Fix R2 integration
   - Add metadata storage

---

## Architecture for Any Scale

### If Personal Use Only:
- Single user, simple key management
- No multi-tenancy

### If Collaboration (Later):
- Add user management
- Share links with permissions
- Team workspaces

### If Public API (Later):
- Add usage tracking
- Add billing (usage based)
- Add rate limits per user

---

## Implementation Path

```
v1.0 (now) → v1.1 (auth) → v1.2 (encryption) → v2.0 (clients)
   ↓              ↓             ↓                ↓
  Notes      API Keys      Encrypt all    CLI + Android
  R2 ready   Rate limit    User isolation  Web UI
```

---

## What's Ready Now

| Component | Status | Notes |
|-----------|--------|-------|
| Cloudflare Worker | ✓ | Rust-based |
| D1 Database | ✓ | Notes CRUD |
| R2 Storage | ✓ | Bucket created |
| GitHub Deploy | ✓ | Auto-deploy |
| Auth | ✗ | Next priority |
| Encryption | ✗ | After auth |
| Clients | ✗ | After foundation |

---

## Next Decision

We can add:
1. **API Key Auth** - Secure the endpoints
2. **Fix R2 Integration** - Full file support
3. **User Isolation** - Multi-user ready

What's your priority? Fix R2 files or add auth first?