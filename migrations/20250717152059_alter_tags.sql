ALTER TABLE tags
ADD COLUMN board_uuid UUID NOT NULL REFERENCES boards (uuid) ON DELETE CASCADE,
ADD CONSTRAINT unique_board_name UNIQUE (board_uuid, name);

CREATE INDEX idx_tags_board_uuid ON tags (board_uuid);

CREATE INDEX idx_tags_name ON tags (name);

-- CREATE TABLE tags (
--   uuid UUID PRIMARY KEY DEFAULT gen_random_uuid (),
--   board_uuid UUID NOT NULL REFERENCES boards (uuid) ON DELETE CASCADE,
--   name TEXT NOT NULL UNIQUE,
--   color TEXT NOT NULL DEFAULT '#415a77',
--   UNIQUE (board_uuid, name),
-- );
-- -- Indexes
-- CREATE INDEX idx_tags_board_uuid ON tags (board_uuid);
-- CREATE INDEX idx_tags_name ON tags (name);
