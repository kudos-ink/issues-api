-- Create roles table
CREATE TABLE IF NOT EXISTS roles (
    id SERIAL PRIMARY KEY,
    name TEXT NOT NULL UNIQUE, -- Ensure each role name is unique
    created_at TIMESTAMP WITH TIME ZONE DEFAULT (now() AT TIME ZONE 'utc') NOT NULL,
    updated_at TIMESTAMP WITH TIME ZONE
);

-- Create users_projects_roles table
CREATE TABLE IF NOT EXISTS users_projects_roles (
    id SERIAL PRIMARY KEY,
    user_id INT REFERENCES users(id) ON DELETE CASCADE NOT NULL,
    project_id INT REFERENCES projects(id) ON DELETE CASCADE, -- only for maintainers
    role_id INT REFERENCES roles(id) ON DELETE CASCADE NOT NULL,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT (now() AT TIME ZONE 'utc') NOT NULL,
    -- Ensure a user cannot have duplicate roles within the same project
    CONSTRAINT unique_user_project_role UNIQUE (user_id, project_id, role_id)
);

INSERT INTO 
    roles("id","name") 
VALUES
    (1, 'Admin'),
    (2, 'Contributor'),
    (3, 'Maintainer'),
    (4, 'Ecosystem Architect');