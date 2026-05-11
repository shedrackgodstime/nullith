# Nullith Roadmap

## Completed ✅
- [x] Cloudflare Worker with D1 database (notes API)
- [x] R2 bucket for file storage
- [x] GitHub Actions auto-deploy
- [x] Logging and error handling
- [x] API Key authentication (single key)
- [x] File upload/download/delete/list

## In Progress 🔄
- [ ] Set API_KEY secret in Cloudflare

## Planned 📋

### Phase 1: Multi-Key Auth (High Priority)
- [ ] Multiple API keys with different privileges
- [ ] Key management endpoints (create, revoke, list)
- [ ] Permission levels: Read-only, Write, Delete, Admin
- [ ] Store keys in D1 database (hashed)

### Phase 2: User Isolation
- [ ] Add user_id to notes table
- [ ] Add user_id to R2 object metadata
- [ ] Per-user data separation
- [ ] API key -> user mapping

### Phase 3: Client Apps
- [ ] CLI tool (Rust)
- [ ] Android app (Kotlin)
- [ ] Desktop app (maybe Tauri?)

### Phase 4: Enhanced Features
- [ ] Notes categories/folders
- [ ] File folders in R2
- [ ] Share links (time-limited)
- [ ] Webhooks on changes

### Phase 5: Observability
- [ ] Request metrics dashboard
- [ ] Usage alerts
- [ ] Cost tracking (free tier monitoring)

## Tech Stack
- Backend: Rust + workers-rs + D1 + R2
- CI/CD: GitHub Actions
- Hosting: Cloudflare Workers (free tier)

## Free Tier Limits
- 100K requests/day
- 1GB R2 storage
- 5GB D1 storage