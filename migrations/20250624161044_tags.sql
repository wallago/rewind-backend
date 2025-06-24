CREATE TABLE tags (
  uuid UUID PRIMARY KEY DEFAULT gen_random_uuid (),
  name TEXT NOT NULL UNIQUE,
  color TEXT DEFAULT '#415a77'
);
