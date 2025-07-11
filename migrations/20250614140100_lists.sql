CREATE EXTENSION IF NOT EXISTS pgcrypto;

CREATE TABLE lists (
  uuid UUID PRIMARY KEY DEFAULT gen_random_uuid (),
  board_uuid UUID NOT NULL REFERENCES boards (uuid) ON DELETE CASCADE,
  name TEXT NOT NULL,
  description TEXT,
  position INT NOT NULL DEFAULT 0,
  deleted BOOLEAN NOT NULL DEFAULT FALSE,
  created_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
  updated_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
  UNIQUE (board_uuid, position),
  CHECK (position >= -1)
);

-- Indexes
CREATE INDEX idx_lists_board_uuid ON lists (board_uuid);

CREATE INDEX idx_lists_position ON lists (position);
