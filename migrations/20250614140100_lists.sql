CREATE TABLE lists (
  uuid UUID PRIMARY KEY DEFAULT gen_random_uuid (),
  board_uuid UUID NOT NULL REFERENCES boards (uuid),
  name TEXT NOT NULL,
  description TEXT,
  deleted BOOLEAN NOT NULL DEFAULT FALSE,
  created_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
  updated_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP
);

CREATE EXTENSION IF NOT EXISTS pgcrypto;
