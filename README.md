# Nullith

A personal cloud system - files, notes, tools - accessible via API, built on Cloudflare Workers.

## Features

- **Notes** - Store text/notes with key-value API
- **Files** - Store blobs in R2, metadata in D1
- **API** - RESTful endpoints with Rust/WASM backend
- **Auto-deploy** - GitHub Actions on every push

## Project Structure

```
nullith/
├── worker/                 # Cloudflare Worker (Rust)
│   ├── src/
│   │   ├── lib.rs         # Main entry point
│   │   ├── routes/        # API route handlers
│   │   ├── models/        # Data structures
│   │   ├── utils/         # Helpers
│   │   └── errors.rs      # Error handling
│   ├── Cargo.toml
│   ├── wrangler.toml
│   └── schema.sql         # D1 database schema
├── cli/                    # Linux CLI client (Rust) [future]
├── android/                # Android client [future]
├── web/                    # Web dashboard [future]
├── ARCHITECTURE.md         # System design
└── README.md
```

## Quick Start

### Prerequisites

- Rust (wasm32-unknown-unknown target)
- Bun or Node.js + npm
- Cloudflare account with R2 + D1

### Local Development

```bash
cd worker

# Install dependencies (if needed)
npm install

# Create .dev.vars file (copy from example)
cp .dev.vars.example .dev.vars
# Edit .dev.vars and add your API_KEY

# Install Rust toolchain
rustup target add wasm32-unknown-unknown
cargo install worker-build

# Run locally (uses local D1/R2 if configured)
bunx wrangler dev
# or
npx wrangler dev
```

The worker will be available at `http://localhost:8787`

### Set API Key for Local Dev

```bash
echo 'API_KEY=5WhhRuT5Oyn5CC+qe1YKg5ltgoM/mzxdKYLrnH61s2Y=' > .dev.vars
```

### Deployment

Push to `main` branch - GitHub Actions auto-deploys.

## API Endpoints

### Notes

| Method | Endpoint | Description |
|--------|----------|-------------|
| GET | `/notes` | List all notes |
| GET | `/notes/:key` | Get a note |
| POST | `/notes/:key` | Create note |
| PUT | `/notes/:key` | Update note |
| DELETE | `/notes/:key` | Delete note |

**Request:**
```json
{"value": "Note content"}
```

**Response:**
```json
{
  "key": "todo",
  "value": "Note content",
  "create_at": 1778403344535,
  "update_at": 1778403344535
}
```

### Files

| Method | Endpoint | Description |
|--------|----------|-------------|
| GET | `/files` | List all files |
| GET | `/files/*path` | Download file |
| PUT | `/files/*path` | Upload file |
| DELETE | `/files/*path` | Delete file |

**Upload:**
```bash
curl -X PUT http://localhost:8787/files/test.txt \
  -H "Content-Type: text/plain" \
  --data-binary @file.txt
```

**Download:**
```bash
curl http://localhost:8787/files/test.txt
```

## Technology Stack

| Layer | Technology |
|-------|-------------|
| Runtime | Cloudflare Workers |
| Language | Rust (via workers-rs) |
| Database | Cloudflare D1 (SQLite) |
| Storage | Cloudflare R2 |
| Deploy | GitHub Actions |
| Build | worker-build |

## Environment Variables

```
CLOUDFLARE_API_TOKEN    # Required for deployment (GitHub secret)
CLOUDFLARE_ACCOUNT_ID   # Required for deployment (GitHub secret)
API_KEY                 # Set via wrangler secret put
```

## Testing

```bash
# Run unit tests
cd worker && cargo test

# Test locally with auth
curl -H "X-API-Key: your-key" http://localhost:8787/notes
```

## Security

All API endpoints require `X-API-Key` header.

```bash
curl -H "X-API-Key: your-key-here" http://localhost:8787/notes
```

Set the secret via:
```bash
bunx wrangler secret put API_KEY
```

### Planned
- Multi-key auth with permissions (read/write/admin)
- Client-side encryption
- Rate limiting per key
- User isolation

## Contributing

1. Fork the repo
2. Create a feature branch
3. Push changes
4. Auto-deploys to staging

## License

MIT