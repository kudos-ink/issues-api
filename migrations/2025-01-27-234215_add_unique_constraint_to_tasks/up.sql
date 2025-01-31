ALTER TABLE tasks
ADD CONSTRAINT tasks_repo_number_unique UNIQUE (repository_id, number);
