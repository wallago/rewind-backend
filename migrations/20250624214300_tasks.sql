CREATE EXTENSION IF NOT EXISTS pgcrypto;

CREATE TABLE tasks (
  uuid UUID PRIMARY KEY DEFAULT gen_random_uuid (),
  list_uuid UUID NOT NULL REFERENCES lists (uuid) ON DELETE CASCADE,
  name TEXT NOT NULL,
  description TEXT,
  position INTEGER NOT NULL DEFAULT 0,
  status status NOT NULL DEFAULT 'todo',
  priority priorities NOT NULL DEFAULT 'medium',
  deleted BOOLEAN NOT NULL DEFAULT FALSE,
  created_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
  updated_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
  deadline TIMESTAMPTZ,
  start_date TIMESTAMPTZ,
  finish_date TIMESTAMPTZ,
  UNIQUE (list_uuid, position),
  CHECK (position >= 0),
  CHECK (
    (
      start_date IS NULL
      OR finish_date IS NULL
    )
    OR (start_date <= finish_date)
  )
);

-- Trigger function for auto-updating `updated_at`
CREATE OR REPLACE FUNCTION update_updated_at_column()
RETURNS TRIGGER AS $$
BEGIN
   NEW.updated_at = NOW();
   RETURN NEW;
END;
$$ language 'plpgsql';

-- Apply the trigger
CREATE TRIGGER set_updated_at BEFORE
UPDATE ON tasks FOR EACH ROW EXECUTE PROCEDURE update_updated_at_column ();

-- Indexes
CREATE INDEX idx_tasks_list_uuid ON tasks (list_uuid);

CREATE INDEX idx_tasks_status ON tasks (status);

CREATE INDEX idx_tasks_deadline ON tasks (deadline);

CREATE INDEX idx_tasks_list_position ON tasks (list_uuid, position);
