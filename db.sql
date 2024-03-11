CREATE TYPE tip_status AS ENUM ('set', 'paid', 'rejected');
CREATE TYPE tip_type AS ENUM ('direct', 'gov');
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
-- Moved the tips table definition before the issues table
CREATE TABLE IF NOT EXISTS tips (
    id SERIAL PRIMARY KEY,
    status tip_status NOT NULL,
    type tip_type NOT NULL,
    amount BIGINT CHECK (amount >= 0),
    "to" VARCHAR(48),
    "from" VARCHAR(48),
    contributor_id INT REFERENCES users(id),
    curator_id INT REFERENCES users(id),
    created_at TIMESTAMP WITH TIME ZONE DEFAULT (now() AT TIME ZONE 'utc'),
    updated_at TIMESTAMP
);
CREATE TABLE IF NOT EXISTS issues (
    id SERIAL PRIMARY KEY,
    issue_number INT,
    repository_id INT REFERENCES repositories(id),
    tip_id INT REFERENCES tips(id),
    created_at TIMESTAMP WITH TIME ZONE DEFAULT (now() AT TIME ZONE 'utc')
);
-- Adding a many-to-many relationship table for users and repositories
CREATE TABLE IF NOT EXISTS repository_users (
    repository_id INT REFERENCES repositories(id),
    user_id INT REFERENCES users(id),
    PRIMARY KEY (repository_id, user_id)
);