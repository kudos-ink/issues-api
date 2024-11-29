-- user can create milestones in a project
CREATE TABLE IF NOT EXISTS users_milestones_role (
    id SERIAL PRIMARY KEY,
    user_id INT REFERENCES users(id) NULL,
    project_id INT REFERENCES projects(id) ON DELETE CASCADE NOT NULL
);

-- user can create tasks in a milestone
CREATE TABLE IF NOT EXISTS users_tasks_role (
    id SERIAL PRIMARY KEY,
    user_id INT REFERENCES users(id) NULL,
    milestone_id INT REFERENCES milestones(id) ON DELETE CASCADE NOT NULL
);