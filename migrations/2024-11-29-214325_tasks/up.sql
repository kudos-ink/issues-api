CREATE TABLE tasks (
    id SERIAL PRIMARY KEY,
    number INT, -- Github Issue number
    repository_id INT,
    title TEXT NOT NULL, -- Title is mandatory
    description TEXT,
    url TEXT,
    labels TEXT[], -- Labels for filtering
    open BOOLEAN NOT NULL DEFAULT TRUE, -- Whether the task is open
    type TEXT NOT NULL, -- "dev", "non-dev", "wish"
    project_id INT REFERENCES projects(id) ON DELETE CASCADE, -- Links tasks to a project
    created_by_user_id INT, -- Who submitted the task
    assignee_user_id INT, -- Directly assigned to a user
    assignee_team_id INT, -- Assigned to a team (if applicable)
    funding_options TEXT[], -- Funding methods (treasury bounty, w3f grant)(TBD)
    contact TEXT, -- Contact information
    skills TEXT[], -- Skills needed (e.g. Marketing, BD, Ops, Talent, Research, Tech, OpenGov)
    bounty INT DEFAULT 0, -- Bounty amount, 0 by default
    approved_by INT[], -- Approvals by trusted users
    approved_at TIMESTAMP WITH TIME ZONE, -- When approval occurred
    status TEXT NOT NULL DEFAULT 'open', -- "open", "in-progress", "completed"
    upvotes INT DEFAULT 0, -- Upvote count
    downvotes INT DEFAULT 0, -- Downvote count
    is_featured BOOLEAN DEFAULT FALSE, -- Whether this task is featured
    is_certified BOOLEAN DEFAULT FALSE, -- Whether the task is certified (actual certified)
    featured_by_user_id INT, -- User who marked it as featured
    issue_created_at TIMESTAMP WITH TIME ZONE, -- for Github issues
    issue_closed_at TIMESTAMP WITH TIME ZONE, -- for Github issues
    created_at TIMESTAMP WITH TIME ZONE DEFAULT (now() AT TIME ZONE 'utc') NOT NULL,
    updated_at TIMESTAMP WITH TIME ZONE
);

CREATE TABLE tasks_votes (
    id SERIAL PRIMARY KEY,
    user_id INT REFERENCES users(id) ON DELETE CASCADE NOT NULL,
    task_id INT REFERENCES tasks(id) ON DELETE CASCADE NOT NULL,
    vote INT NOT NULL,
    CONSTRAINT unique_vote UNIQUE (user_id, task_id)
);

-- Create a function to handle the vote updates
CREATE OR REPLACE FUNCTION update_task_votes()
RETURNS TRIGGER AS $$
BEGIN
    IF TG_OP = 'INSERT' THEN
        IF NEW.vote > 0 THEN
            UPDATE tasks
            SET upvotes = upvotes + 1
            WHERE id = NEW.task_id;
        ELSIF NEW.vote < 0 THEN
            UPDATE tasks
            SET downvotes = downvotes + 1
            WHERE id = NEW.task_id;
        END IF;
    ELSIF TG_OP = 'DELETE' THEN
        IF OLD.vote > 0 THEN
            UPDATE tasks
            SET upvotes = upvotes - 1
            WHERE id = OLD.task_id;
        ELSIF OLD.vote < 0 THEN
            UPDATE tasks
            SET downvotes = downvotes - 1
            WHERE id = OLD.task_id;
        END IF;
    END IF;

    RETURN NULL;
END;
$$ LANGUAGE plpgsql;

-- Create the trigger for the tasks_votes table
CREATE TRIGGER trigger_update_task_votes
AFTER INSERT OR DELETE ON tasks_votes
FOR EACH ROW
EXECUTE FUNCTION update_task_votes();
