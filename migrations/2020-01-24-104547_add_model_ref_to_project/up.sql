ALTER TABLE projects
ADD CONSTRAINT project_model
FOREIGN KEY (model_id) REFERENCES documents(id);