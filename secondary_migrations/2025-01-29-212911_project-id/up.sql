ALTER TABLE repositories
    ADD CONSTRAINT unique_id_project_id UNIQUE (id, project_id);

CREATE OR REPLACE FUNCTION set_project_id_from_repository()
RETURNS TRIGGER AS $$
BEGIN
    NEW.project_id := (SELECT project_id FROM repositories WHERE id = NEW.repository_id);
    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

CREATE TRIGGER trigger_set_project_id
BEFORE INSERT OR UPDATE ON tasks
FOR EACH ROW
WHEN (NEW.repository_id IS NOT NULL AND NEW.project_id IS NULL)
EXECUTE FUNCTION set_project_id_from_repository();