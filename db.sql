CREATE TABLE IF NOT EXISTS users (
    id SERIAL PRIMARY KEY,
    username VARCHAR(100) UNIQUE,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT (now() AT TIME ZONE 'utc')
);
CREATE TABLE IF NOT EXISTS organizations (
    id SERIAL PRIMARY KEY,
    name VARCHAR(255) UNIQUE,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT (now() AT TIME ZONE 'utc')
);
CREATE TABLE IF NOT EXISTS repositories (
    id SERIAL PRIMARY KEY,
    name VARCHAR(255),
    organization_id INT REFERENCES organizations(id),
    -- user_id INT REFERENCES users(id), // TODO: allow users
    created_at TIMESTAMP WITH TIME ZONE DEFAULT (now() AT TIME ZONE 'utc')
);
CREATE TABLE IF NOT EXISTS issues (
    id SERIAL PRIMARY KEY,
    issue_number INT,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT (now() AT TIME ZONE 'utc'),
    updated_at TIMESTAMP,
    repository_id INT REFERENCES repositories(id) -- TODO: add "tipping" status
);