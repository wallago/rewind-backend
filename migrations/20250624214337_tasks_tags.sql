CREATE TABLE tasks_tags (
  task_uuid UUID NOT NULL REFERENCES tasks (uuid) ON DELETE CASCADE,
  tag_uuid UUID NOT NULL REFERENCES tags (uuid) ON DELETE CASCADE,
  PRIMARY KEY (task_uuid, tag_uuid)
);
