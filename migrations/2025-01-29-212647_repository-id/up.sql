
ALTER TABLE tasks
    ADD CONSTRAINT tasks_repository_id_fkey
    FOREIGN KEY (repository_id)
    REFERENCES repositories(id)
    ON DELETE CASCADE;