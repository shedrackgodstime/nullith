# Nullith - Personal Cloud System

## How It Works

```
                    ┌─────────────────────────────────────────┐
                    │           CLOUDFLARE                   │
                    │                                         │
                    │   ┌─────────────────────────────────┐   │
                    │   │         WORKER (API)              │   │
                    │   │                                 │   │
                    │   │   GET /files       POST /files   │   │
                    │   │   GET /files/:id  DELETE /files/:id   │
                    │   │   GET /notes      POST /notes   │   │
                    │   │   PUT /notes/:key DELETE /notes/:key   │
                    │   │                                 │   │
                    │   └───────────────┬─────────────────┘   │
                    │                 │                     │
                    │      ┌──────────┴──────────┐       │
                    │      ↓                     ↓       │
                    │   ┌──────┐           ┌───────┐   │
                    │   │  R2  │           │  D1   │   │
                    │   │(files)           │(SQLite)│   │
                    │   └──────┘           └──┬────┘   │
                    │                        │        │
                    │              ┌────────┴────────┐ │
                    │              ↓                 ↓ │
                    │         files table     notes table  │
                    │         + api_keys         + meta  │
                    └─────────────────────────────────┘
```

### Data Flow

#### Upload File
```
Client ──(POST /files)────→ Worker ──→ R2 (blob)
                              └─→ D1 (metadata)
                           ←─── Response (file_id)
```

#### Download File
```
Client ──(GET /files/:id)──→ Worker ──→ D1 (lookup)
                              └─→ R2 (fetch blob)
                           ←─── File
```

#### Store Note
```
Client ──(PUT /notes/:key)──→ Worker ──→ D1 (upsert)
                           ←─── Response
```

#### Get Note
```
Client ──(GET /notes/:key)──→ Worker ──→ D1 (fetch)
                           ←─── { key, value, updated_at }
```

------------------------------------------------------------------------

## API Endpoints

### Files
| Method | Endpoint | Description |
|--------|----------|-------------|
| POST | /files | Upload file |
| GET | /files | List files |
| GET | /files/:id | Download file |
| DELETE | /files/:id | Delete file |

### Notes
| Method | Endpoint | Description |
|--------|----------|-------------|
| GET | /notes | List notes |
| GET | /notes/:key | Get note |
| PUT | /notes/:key | Create/update note |
| DELETE | /notes/:key | Delete note |

### Query Params (both)
| Param | Example | Description |
|-------|---------|-------------|
| limit | ?limit=50 | Max results |
| offset | ?offset=0 | Pagination |
| search | ?search=foo | Search name/key |

------------------------------------------------------------------------

## Core Stack

-   **Compute:** Cloudflare Workers
-   **Storage:** Cloudflare R2
-   **Database:** Cloudflare D1 (SQLite-based)
-   **Auth:** Simple API Keys or Magic Links
-   **Backup (Future):** MEGA / Storj
-   **Automation (Future):** GitHub Actions

------------------------------------------------------------------------

## Tech Stack

### Cloudflare (Backend)
| Component | Technology | Rationale |
|-----------|------------|-----------|
| Compute | Cloudflare Workers (TypeScript) | Native bindings for R2/D1 |
| Runtime | Node.js runtime | Stable, well-documented |
| Encryption | XChaCha20-Poly1305 (tweetnacl) | Modern, fast, small output |
| API | REST (Fetch API) | Simple, universally compatible |
| Auth | API Keys (blake2b hashed) | Stateless, no session management |

### Linux Client
| Component | Technology | Rationale |
|-----------|------------|-----------|
| Language | Rust | Memory safe, small binaries, great CLI crates |
| HTTP | reqwest | Battle-tested |
| Encryption | x25519-dalek + chacha20poly1305 | Modern crypto |
| CLI | clap | Best CLI argument parsing |
| Storage | tokio + serde | Async + serialization |

### Android Client
| Component | Technology | Rationale |
|-----------|------------|-----------|
| Language | Kotlin | Native Android |
| HTTP | Retrofit + OkHttp | Robust HTTP client |
| Encryption | Tink (Android) | Google's encryption library |
| Storage | Room (SQLite) | Local caching |
| UI | Jetpack Compose | Modern Android UI |

### General
| Component | Technology | Rationale |
|-----------|------------|-----------|
| Encryption | Client-side encryption | Server never sees plaintext |
| Key Derivation | Argon2id | Memory-hard KDF |
| File Integrity | BLAKE3 | Fast, verified hashing |

------------------------------------------------------------------------

## Architecture Overview

```
Clients (Linux/Android/Web/API)
        ↓
   Cloudflare Worker API
        ↓
   ┌─────────┬──────────┐
   │Encrypt  │ Encrypt  │
   ↓         ↓          ↓
   R2      D1 (files)  D1 (notes/text)
 (blobs)   (metadata)  (key-value JSON)
```

------------------------------------------------------------------------

## Data Types

### Files (R2)
- Binary blobs: documents, images, archives
- Encrypted before upload
- D1 tracks metadata (hash, size, uploaded_at)

### Notes/Text (D1)
- Key-value store for quick text
- Full markdown documents
- API-accessible snippets
- JSON structure: `{ key, value, updated_at }`

------------------------------------------------------------------------

## Clients

### Priority 1: Linux + Android
- Linux: CLI tool + optional GUI
- Android: Native app (Kotlin)

### Priority 2: Web Dashboard
- Fallback for any platform
- No Apple requirement

### Priority 3: Windows (optional)
- Later if needed

------------------------------------------------------------------------

## Features

- File upload/download (encrypted)
- Text/notes storage (quick API access)
- Quick share (temporary links)
- Cross-device sync
- CLI for Linux

------------------------------------------------------------------------

## Database Schema

### Files Metadata (D1)
```sql
CREATE TABLE files (
  id TEXT PRIMARY KEY,
  name TEXT,
  hash TEXT,
  size INTEGER,
  mime_type TEXT,
  uploaded_at INTEGER,
  status TEXT
);
```

### Notes/Text (D1)
```sql
CREATE TABLE notes (
  id TEXT PRIMARY KEY,
  key TEXT UNIQUE,
  value TEXT,
  created_at INTEGER,
  updated_at INTEGER
);
```

### API Keys (D1)
```sql
CREATE TABLE api_keys (
  id TEXT PRIMARY KEY,
  key_hash TEXT,
  name TEXT,
  created_at INTEGER,
  last_used INTEGER
);
```

------------------------------------------------------------------------

## Upload Flow (Files)

1.  Client sends file to Worker API
2.  Worker encrypts file (XChaCha20-Poly1305)
3.  File stored in R2
4.  Metadata stored in D1 files table

## Download Flow (Files)

1.  Client requests file
2.  Worker checks D1
3.  Fetch from R2
4.  Return file

## Notes/Text Flow

1.  Client sends key/value to Worker API
2.  Worker validates API key
3.  Upsert to D1 notes table
4.  Return updated record

## Quick Share Flow

1.  Create share link with expiry (views or time)
2.  Share link opens worker endpoint
3.  Worker serves file/note directly
4.  Expires after condition met

------------------------------------------------------------------------

## Future Enhancements

-   Multi-cloud backup (MEGA, Storj)
-   Failover logic
-   Deduplication (store once, reference many)
-   File versioning
-   Collaboration (share with others)
-   Password manager (encrypted notes)
-   Calendar/tasks (future)
-   Periodic sync using GitHub Actions

------------------------------------------------------------------------

## Key Principles

-   Encrypt before upload
-   Never rely on single provider
-   Keep system stateless
-   Design for failure
-   Offline-first for clients
-   Privacy-first (no tracking)

------------------------------------------------------------------------

## Implementation Phases

### Phase 1: Core API (MVP - No Auth/Encryption)
- Worker setup with D1 + R2
- File upload/download (POST/GET /files)
- Notes CRUD (CRUD /notes)
- **Goal**: Ship fast, get something working

### Phase 2: Linux Client
- CLI tool (Rust)
- File sync commands
- Notes access commands

### Phase 3: Android App
- Native Android app (Kotlin)
- File manager UI
- Notes editor

### Phase 4: Web Dashboard
- Simple web UI
- File browser
- Notes editor

### Phase 5: Security
- API key auth
- Client-side encryption
- Key derivation

### Phase 6: Polish
- Quick share links
- File versioning
- Deduplication

## Project Structure

```
nullith/
├── worker/     ← Cloudflare Worker (TypeScript)
├── cli/        ← Linux CLI (Rust)
├── android/    ← Android App (Kotlin)
└── web/        ← Web Dashboard
```

## Deployment

| Component | Deploy To |
|-----------|-----------|
| worker | Cloudflare Workers |
| R2 | Cloudflare R2 |
| D1 | Cloudflare D1 |
| cli | GitHub Releases / Direct download |
| android | F-Droid / Direct APK |
| web | Cloudflare Pages (optional) |

A zero-cost, API-driven personal cloud replacing self-hosted
services - files, notes, tools - accessible from Linux, Android,
and web. Privacy-focused, encrypted, and scalable.
