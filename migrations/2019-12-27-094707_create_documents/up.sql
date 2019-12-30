CREATE TABLE documents (
  id UUID PRIMARY KEY NOT NULL,
  project_id UUID REFERENCES projects(id) NOT NULL,
  name TEXT NOT NULL,
  doctype TEXT NOT NULL,
  body jsonb NOT NULL
);

  -- created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
  -- updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
-- CREATE TRIGGER set_timestamp
-- BEFORE UPDATE ON documents
-- FOR EACH ROW
-- EXECUTE PROCEDURE trigger_set_timestamp();