CREATE TABLE IF NOT EXISTS contribution
(
    id BIGINT PRIMARY KEY NOT NULL,
    created_at timestamp with time zone DEFAULT (now() at time zone 'utc')
);