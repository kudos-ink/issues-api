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
    created_at TIMESTAMP WITH TIME ZONE DEFAULT (now() AT TIME ZONE 'utc')
);
CREATE TABLE IF NOT EXISTS issues (
    id SERIAL PRIMARY KEY,
    issue_number INT,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT (now() AT TIME ZONE 'utc'),
    repository_id INT REFERENCES repositories(id),
    tip_id INT REFERENCES tips(id)
);
CREATE TABLE IF NOT EXISTS tips (
    id SERIAL PRIMARY KEY,
    issue_id INT REFERENCES issues(id),
    url VARCHAR(255) UNIQUE,
    -- TODO: extra fields of tipping
    created_at TIMESTAMP WITH TIME ZONE DEFAULT (now() AT TIME ZONE 'utc')
);
-- Adding a many-to-many relationship table for users and repositories
CREATE TABLE IF NOT EXISTS repository_users (
    repository_id INT REFERENCES repositories(id),
    user_id INT REFERENCES users(id),
    PRIMARY KEY (repository_id, user_id)
);