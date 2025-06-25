CREATE EXTENSION IF NOT EXISTS pgcrypto;

CREATE TABLE boards (
  uuid UUID PRIMARY KEY DEFAULT gen_random_uuid (),
  name TEXT NOT NULL,
  description TEXT,
  position INTEGER NOT NULL DEFAULT 0 UNIQUE,
  deleted BOOLEAN NOT NULL DEFAULT FALSE,
  created_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
  updated_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
  CHECK (position >= -1)
);

-- Indexes 
CREATE INDEX idx_boards_position ON boards (position);
