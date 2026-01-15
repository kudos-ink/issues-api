
DROP TRIGGER IF EXISTS trigger_set_project_id ON tasks;
DROP FUNCTION IF EXISTS set_project_id_from_repository;
ALTER TABLE tasks
    DROP CONSTRAINT tasks_repository_id_fkey;

ALTER TABLE repositories
    DROP CONSTRAINT unique_id_project_id;
