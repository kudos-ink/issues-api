-- Drop and replace the trigger function
CREATE OR REPLACE FUNCTION update_task_votes()
RETURNS TRIGGER AS $$
BEGIN
    IF TG_OP = 'INSERT' THEN
        IF NEW.vote > 0 THEN
            UPDATE tasks SET upvotes = upvotes + 1 WHERE id = NEW.task_id;
        ELSIF NEW.vote < 0 THEN
            UPDATE tasks SET downvotes = downvotes + 1 WHERE id = NEW.task_id;
        END IF;

    ELSIF TG_OP = 'DELETE' THEN
        IF OLD.vote > 0 THEN
            UPDATE tasks SET upvotes = upvotes - 1 WHERE id = OLD.task_id;
        ELSIF OLD.vote < 0 THEN
            UPDATE tasks SET downvotes = downvotes - 1 WHERE id = OLD.task_id;
        END IF;

    ELSIF TG_OP = 'UPDATE' THEN
        IF OLD.vote != NEW.vote THEN
            IF OLD.vote > 0 THEN
                UPDATE tasks SET upvotes = upvotes - 1 WHERE id = OLD.task_id;
            ELSIF OLD.vote < 0 THEN
                UPDATE tasks SET downvotes = downvotes - 1 WHERE id = OLD.task_id;
            END IF;

            IF NEW.vote > 0 THEN
                UPDATE tasks SET upvotes = upvotes + 1 WHERE id = NEW.task_id;
            ELSIF NEW.vote < 0 THEN
                UPDATE tasks SET downvotes = downvotes + 1 WHERE id = NEW.task_id;
            END IF;
        END IF;
    END IF;

    RETURN NULL;
END;
$$ LANGUAGE plpgsql;

-- Recreate the trigger to include UPDATE
DROP TRIGGER IF EXISTS trigger_update_task_votes ON tasks_votes;

CREATE TRIGGER trigger_update_task_votes
AFTER INSERT OR DELETE OR UPDATE ON tasks_votes
FOR EACH ROW
EXECUTE FUNCTION update_task_votes();
