-- Your SQL goes here
-- Add a status column to track if a comment is active or soft-deleted
ALTER TABLE task_comments
ADD COLUMN status TEXT NOT NULL DEFAULT 'active'
CONSTRAINT comment_status_check CHECK (status IN ('active', 'deleted'));
