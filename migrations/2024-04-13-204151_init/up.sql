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
CREATE TABLE IF NOT EXISTS languages (
    id SERIAL PRIMARY KEY,
    name VARCHAR(255) UNIQUE,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT (now() AT TIME ZONE 'utc') NOT NULL,
    updated_at TIMESTAMP NULL
);
-- basic github repository
CREATE TABLE IF NOT EXISTS repositories (
    id SERIAL PRIMARY KEY,
    slug VARCHAR(255) NOT NULL UNIQUE,
    name VARCHAR(255) NOT NULL,
    language_id INT REFERENCES languages(id) NOT NULL,
    project_id INT REFERENCES projects(id) NOT NULL,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT (now() AT TIME ZONE 'utc') NOT NULL,
    updated_at TIMESTAMP WITH TIME ZONE NULL
);
-- all the users including maintainers and admins
CREATE TABLE IF NOT EXISTS users (
    id SERIAL PRIMARY KEY,
    username VARCHAR(100) UNIQUE NOT NULL,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT (now() AT TIME ZONE 'utc') NOT NULL,
    updated_at TIMESTAMP WITH TIME ZONE NULL
);
-- issues
CREATE TABLE IF NOT EXISTS issues (
    id SERIAL PRIMARY KEY,
    number int NOT NULL,
    title VARCHAR(100) NOT NULL,
    labels TEXT [],
    open boolean DEFAULT true NOT NULL,
    assignee_id INT REFERENCES users(id) NULL,
    e_tag VARCHAR(100) NOT NULL,
    repository_id INT REFERENCES repositories(id) NOT NULL,
    issue_created_at TIMESTAMP WITH TIME ZONE NOT NULL,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT (now() AT TIME ZONE 'utc') NOT NULL,
    updated_at TIMESTAMP WITH TIME ZONE NULL
);