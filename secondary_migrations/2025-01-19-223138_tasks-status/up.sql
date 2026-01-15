
CREATE OR REPLACE FUNCTION update_task_status() RETURNS TRIGGER AS $$
BEGIN
    NEW.status := CASE 
        WHEN NEW.open = TRUE AND NEW.assignee_user_id IS NULL THEN 'open'
        WHEN NEW.open = TRUE AND NEW.assignee_user_id IS NOT NULL THEN 'in-progress'
        WHEN NEW.open = FALSE THEN 'completed'
    END;
    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

CREATE TRIGGER update_task_status_trigger
BEFORE INSERT OR UPDATE ON tasks
FOR EACH ROW
EXECUTE FUNCTION update_task_status();