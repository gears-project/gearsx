CREATE TABLE documents (
  id UUID PRIMARY KEY NOT NULL,
  project_id UUID REFERENCES projects(id) NOT NULL,
  doctype TEXT NOT NULL,
  version INTEGER DEFAULT 0 NOT NULL,
  doc jsonb NOT NULL
);
