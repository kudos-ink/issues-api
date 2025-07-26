-- Your SQL goes here
CREATE TABLE IF NOT EXISTS task_comments (
    id SERIAL PRIMARY KEY,
    content TEXT NOT NULL,
    task_id INT REFERENCES tasks(id) ON DELETE CASCADE NOT NULL,
    user_id INT REFERENCES users(id) ON DELETE CASCADE NOT NULL,
    parent_comment_id INT REFERENCES task_comments(id) ON DELETE CASCADE, -- For threaded replies
    created_at TIMESTAMP WITH TIME ZONE DEFAULT (now() AT TIME ZONE 'utc') NOT NULL,
    updated_at TIMESTAMP WITH TIME ZONE NULL
);

-- Add an index on task_id for faster comment lookups
CREATE INDEX idx_task_comments_task_id ON task_comments(task_id);