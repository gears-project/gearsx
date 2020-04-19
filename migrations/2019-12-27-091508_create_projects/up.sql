CREATE TABLE projects (
  id UUID PRIMARY KEY NOT NULL,
  name TEXT NOT NULL,
  description TEXT NOT NULL,
  model_id UUID,
  owner UUID NOT NULL,
  created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
  updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE TRIGGER set_projects_timestamp
BEFORE UPDATE ON projects
  FOR EACH ROW
  EXECUTE PROCEDURE trigger_set_timestamp();