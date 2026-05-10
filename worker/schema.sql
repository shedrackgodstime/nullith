-- Nullith D1 Schema

-- Files metadata table
CREATE TABLE IF NOT EXISTS files (
  id TEXT PRIMARY KEY,
  name TEXT NOT NULL,
  size INTEGER NOT NULL,
  mime_type TEXT,
  uploaded_at INTEGER NOT NULL,
  status TEXT DEFAULT 'active'
);

-- Notes table
CREATE TABLE IF NOT EXISTS notes (
  key TEXT PRIMARY KEY,
  value TEXT NOT NULL,
  create_at INTEGER NOT NULL,
  update_at INTEGER NOT NULL
);

-- Indexes
CREATE INDEX IF NOT EXISTS idx_files_uploaded_at ON files(uploaded_at DESC);
CREATE INDEX IF NOT EXISTS idx_notes_updated_at ON notes(update_at DESC);