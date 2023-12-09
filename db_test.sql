DROP TABLE IF EXISTS contribution;
CREATE TABLE IF NOT EXISTS contribution
(
    id bigint PRIMARY KEY NOT NULL,
    created_at timestamp with time zone DEFAULT (now() at time zone 'utc')
);