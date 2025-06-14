CREATE TYPE status AS ENUM ('todo', 'in_progress', 'done');

CREATE TABLE tasks (
  uuid UUID PRIMARY KEY DEFAULT gen_random_uuid (),
  -- list_uuid UUID NOT NULL REFERENCES lists (uuid),
  list_uuid UUID NOT NULL,
  name TEXT NOT NULL,
  description TEXT,
  status status NOT NULL DEFAULT 'todo',
  position INTEGER NOT NULL,
  deleted BOOLEAN NOT NULL DEFAULT FALSE,
  created_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
  updated_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
  deadline TIMESTAMPTZ,
  start_date TIMESTAMPTZ,
  finish_date TIMESTAMPTZ,
  UNIQUE (list_uuid, position)
);

CREATE EXTENSION IF NOT EXISTS pgcrypto;

-- Indexes for Performance
CREATE INDEX idx_tasks_list_uuid ON tasks (list_uuid);

CREATE INDEX idx_tasks_status ON tasks (status);

CREATE INDEX idx_tasks_deadline ON tasks (deadline);

-- CREATE OR REPLACE FUNCTION update_updated_at_column()
-- RETURNS TRIGGER AS $$
-- BEGIN
--    NEW.updated_at = NOW();
--    RETURN NEW;
-- END;
-- $$ language 'plpgsql';
-- CREATE TRIGGER update_task_updated_at
-- BEFORE UPDATE ON tasks
-- FOR EACH ROW
-- EXECUTE PROCEDURE update_updated_at_column();
