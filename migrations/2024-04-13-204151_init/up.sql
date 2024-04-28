-- some enums
CREATE TYPE tip_status AS ENUM ('set', 'paid', 'rejected');
CREATE TYPE issue_status AS ENUM ('open', 'closed');
CREATE TYPE tip_type AS ENUM ('direct', 'gov');
-- some tables to save metadata associated to the issues, repositories and tips
-- used in the repositories
CREATE TABLE IF NOT EXISTS languages (
    id SERIAL PRIMARY KEY,
    name VARCHAR(255) UNIQUE,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT (now() AT TIME ZONE 'utc') NOT NULL,
    updated_at TIMESTAMP NULL
);
-- used in the issues
CREATE TABLE IF NOT EXISTS labels (
    id SERIAL PRIMARY KEY,
    name VARCHAR(255) UNIQUE,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT (now() AT TIME ZONE 'utc') NOT NULL,
    updated_at TIMESTAMP NULL
);
-- used in the repositories
CREATE TABLE IF NOT EXISTS topics (
    id SERIAL PRIMARY KEY,
    name VARCHAR(255) UNIQUE,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT (now() AT TIME ZONE 'utc') NOT NULL,
    updated_at TIMESTAMP NULL
);
-- used in the tips
CREATE TABLE IF NOT EXISTS blockchains (
    id SERIAL PRIMARY KEY,
    name VARCHAR(255) UNIQUE,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT (now() AT TIME ZONE 'utc') NOT NULL,
    updated_at TIMESTAMP NULL
);
-- filters used by the frontend: languages, interests, etc
CREATE TABLE IF NOT EXISTS filters (
    id SERIAL PRIMARY KEY,
    name VARCHAR(255) UNIQUE,
    emoji TEXT NOT NULL,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT (now() AT TIME ZONE 'utc') NOT NULL,
    updated_at TIMESTAMP NULL
);
-- filters used by the frontend: EVM, documentation, etc.
CREATE TABLE IF NOT EXISTS filter_values (
    id SERIAL PRIMARY KEY,
    filters_id INT REFERENCES filters(id) NOT NULL,
    emoji TEXT NOT NULL,
    name VARCHAR(255) UNIQUE,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT (now() AT TIME ZONE 'utc') NOT NULL,
    updated_at TIMESTAMP NULL
);
-- basic github organization
CREATE TABLE IF NOT EXISTS organizations (
    id SERIAL PRIMARY KEY,
    name VARCHAR(255) UNIQUE,
    icon VARCHAR(255),
    description VARCHAR(255),
    url VARCHAR(255),
    email VARCHAR(255),
    twitter VARCHAR(255),
    e_tag VARCHAR(40) NOT NULL,
    is_verified boolean,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT (now() AT TIME ZONE 'utc') NOT NULL,
    updated_at TIMESTAMP NULL
);
-- basic github repository
CREATE TABLE IF NOT EXISTS repositories (
    id SERIAL PRIMARY KEY,
    name VARCHAR(255),
    url VARCHAR(255),
    icon VARCHAR(255),
    e_tag VARCHAR(40) NOT NULL,
    organization_id INT REFERENCES organizations(id),
    created_at TIMESTAMP WITH TIME ZONE DEFAULT (now() AT TIME ZONE 'utc') NOT NULL,
    updated_at TIMESTAMP NULL
);
-- all the users including maintainers and admins
CREATE TABLE IF NOT EXISTS users (
    id SERIAL PRIMARY KEY,
    username VARCHAR(100) UNIQUE NOT NULL,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT (now() AT TIME ZONE 'utc') NOT NULL
);
-- two types of tips: gov and direct. One tip per issue
CREATE TABLE IF NOT EXISTS tips (
    id SERIAL PRIMARY KEY,
    status tip_status NOT NULL,
    type tip_type NOT NULL,
    amount BIGINT CHECK (amount >= 0) NOT NULL,
    "to" VARCHAR(48) NOT NULL,
    "from" VARCHAR(48) NOT NULL,
    --direct
    transaction VARCHAR(255) NULL,
    blockchain_id INT REFERENCES blockchains(id) NULL,
    -- gov
    url VARCHAR(255) NULL,
    contributor_id INT REFERENCES users(id) NOT NULL,
    --not needed if it's a proposal
    curator_id INT REFERENCES users(id) NULL,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT (now() AT TIME ZONE 'utc') NOT NULL,
    updated_at TIMESTAMP NULL
);
-- issue with repository, user and tip
CREATE TABLE IF NOT EXISTS issues (
    id SERIAL PRIMARY KEY,
    issue_number INT NOT NULL,
    url VARCHAR(255),
    title VARCHAR(255),
    description VARCHAR(255),
    status issue_status NOT NULL,
    has_wishes boolean,
    repository_id INT REFERENCES repositories(id) NOT NULL,
    user_id INT REFERENCES users(id) NULL,
    tip_id INT REFERENCES tips(id) NULL,
    issue_created_at TIMESTAMP NOT NULL,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT (now() AT TIME ZONE 'utc') NOT NULL,
    e_tag VARCHAR(40) NOT NULL,
    updated_at TIMESTAMP NULL
);
-- one repository can have multiple languages
-- one language can have multiple repositories
CREATE TABLE IF NOT EXISTS repositories_languages (
    id SERIAL PRIMARY KEY,
    repositories_id INT REFERENCES repositories(id),
    languages_id INT REFERENCES languages(id)
);
-- one repository can have multiple topics
-- one topic can have multiple repositories
CREATE TABLE IF NOT EXISTS repositories_topics (
    id SERIAL PRIMARY KEY,
    repositories_id INT REFERENCES repositories(id),
    topics_id INT REFERENCES topics(id)
);
-- we need to associate the repository (and then the issue) to some
-- filters that are used by the frontend. 
-- For example, "Interests" -> "EVM" and "Network" -> "Kusama", "Polkadot"
-- one repository can have multiple filters
-- one topic can have multiple repositories
CREATE TABLE IF NOT EXISTS repositories_filters (
    id SERIAL PRIMARY KEY,
    repositories_id INT REFERENCES repositories(id),
    filters_id INT REFERENCES filters(id),
    filter_values_id INT REFERENCES filter_values(id)
);
-- one issue can have multiple labels
-- one label can have multiple issues
CREATE TABLE IF NOT EXISTS issues_labels (
    id SERIAL PRIMARY KEY,
    issue_id INT REFERENCES issues(id),
    labels_id INT REFERENCES labels(id)
);
-- one user can be maintainer in multiple repositories
-- one repository can have multiple maintainers
CREATE TABLE IF NOT EXISTS maintainers (
    id SERIAL PRIMARY KEY,
    repository_id INT REFERENCES repositories(id),
    user_id INT REFERENCES users(id)
);
-- wishes are special issues where comments are fetched
CREATE TABLE IF NOT EXISTS wishes (
    id SERIAL PRIMARY KEY,
    issues_id INT REFERENCES issues(id) NOT NULL,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT (now() AT TIME ZONE 'utc') NOT NULL,
    updated_at TIMESTAMP NULL
);
-- each wish has multiple comments
CREATE TABLE IF NOT EXISTS comments (
    id SERIAL PRIMARY KEY,
    wish_id INT REFERENCES wishes(id) NOT NULL,
    user_id INT REFERENCES users(id) NOT NULL,
    comment VARCHAR(255) UNIQUE,
    positive_votes INT,
    negative_votes INT,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT (now() AT TIME ZONE 'utc') NOT NULL,
    updated_at TIMESTAMP NULL
);