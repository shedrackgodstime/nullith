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
- Node.js + npm
- Cloudflare account
- R2 + D1 enabled

### Local Development

```bash
cd worker

# Install dependencies
npm install

# Install Rust toolchain
rustup target add wasm32-unknown-unknown
cargo install worker-build

# Run locally
npx wrangler dev
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

### Files (Coming Soon)

| Method | Endpoint | Description |
|--------|----------|-------------|
| POST | `/files` | Upload file |
| GET | `/files` | List files |
| GET | `/files/:id` | Download file |
| DELETE | `/files/:id` | Delete file |

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
CLOUDFLARE_API_TOKEN    # Required for deployment
CLOUDFLARE_ACCOUNT_ID   # Required for deployment
```

## Security (Coming Soon)

- API key authentication
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