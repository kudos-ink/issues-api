-- basic github repository
CREATE TABLE IF NOT EXISTS projects (
    id SERIAL PRIMARY KEY,
    name TEXT NOT NULL,
    slug TEXT NOT NULL UNIQUE,
    types TEXT [],
    purposes TEXT [],
    stack_levels TEXT [],
    technologies TEXT [],
    avatar VARCHAR(255),
    created_at TIMESTAMP WITH TIME ZONE DEFAULT (now() AT TIME ZONE 'utc') NOT NULL,
    updated_at TIMESTAMP WITH TIME ZONE NULL
);
CREATE TABLE IF NOT EXISTS languages (
    id SERIAL PRIMARY KEY,
    slug TEXT NOT NULL UNIQUE,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT (now() AT TIME ZONE 'utc') NOT NULL,
    updated_at TIMESTAMP WITH TIME ZONE NULL
);
-- basic github repository
CREATE TABLE IF NOT EXISTS repositories (
    id SERIAL PRIMARY KEY,
    slug TEXT NOT NULL UNIQUE,
    name TEXT NOT NULL,
    url TEXT UNIQUE NOT NULL,
    language_slug TEXT NOT NULL,
    project_id INT REFERENCES projects(id) ON DELETE CASCADE NOT NULL,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT (now() AT TIME ZONE 'utc') NOT NULL,
    updated_at TIMESTAMP WITH TIME ZONE NULL
);
-- all the users including maintainers and admins
CREATE TABLE IF NOT EXISTS users (
    id SERIAL PRIMARY KEY,
    username TEXT UNIQUE NOT NULL,
    avatar TEXT NULL,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT (now() AT TIME ZONE 'utc') NOT NULL,
    updated_at TIMESTAMP WITH TIME ZONE NULL
);
-- issues
CREATE TABLE IF NOT EXISTS issues (
    id SERIAL PRIMARY KEY,
    number int NOT NULL,
    title TEXT NOT NULL,
    labels TEXT [],
    open boolean DEFAULT true NOT NULL,
    certified boolean,
    assignee_id INT REFERENCES users(id) NULL,
    repository_id INT REFERENCES repositories(id) ON DELETE CASCADE NOT NULL,
    issue_created_at TIMESTAMP WITH TIME ZONE NOT NULL,
    issue_closed_at TIMESTAMP WITH TIME ZONE NULL,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT (now() AT TIME ZONE 'utc') NOT NULL,
    updated_at TIMESTAMP WITH TIME ZONE NULL,
    CONSTRAINT issue_closed_at_check CHECK (
        issue_closed_at IS NULL OR (
            issue_closed_at > issue_created_at
        )
    ),
    CONSTRAINT issues_repo_number_unique UNIQUE (repository_id, number)
);