-- basic github repository
CREATE TABLE IF NOT EXISTS projects (
    id SERIAL PRIMARY KEY,
    name VARCHAR(255) NOT NULL,
    slug VARCHAR(255) NOT NULL UNIQUE,
    categories TEXT [],
    purposes TEXT [],
    stack_levels TEXT [],
    technologies TEXT [],
    created_at TIMESTAMP WITH TIME ZONE DEFAULT (now() AT TIME ZONE 'utc') NOT NULL,
    updated_at TIMESTAMP WITH TIME ZONE NULL
);
-- basic github repository
CREATE TABLE IF NOT EXISTS repositories (
    id SERIAL PRIMARY KEY,
    slug VARCHAR(255) NOT NULL UNIQUE,
    name VARCHAR(255) NOT NULL,
    project_id INT REFERENCES projects(id),
    created_at TIMESTAMP WITH TIME ZONE DEFAULT (now() AT TIME ZONE 'utc') NOT NULL,
    updated_at TIMESTAMP WITH TIME ZONE NULL
);