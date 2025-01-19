CREATE OR REPLACE FUNCTION assign_role_to_user()
RETURNS TRIGGER AS $$
BEGIN
  INSERT INTO users_projects_roles (user_id, role_id)
  SELECT NEW.assignee_user_id, 2
  WHERE NOT EXISTS (
    SELECT 1
    FROM users_projects_roles
    WHERE user_id = NEW.assignee_user_id
      AND role_id = 2
  );
  RETURN NEW;
END;
$$ LANGUAGE plpgsql;

CREATE TRIGGER trigger_assign_role_to_user
AFTER INSERT OR UPDATE OF assignee_user_id ON tasks
FOR EACH ROW
EXECUTE FUNCTION assign_role_to_user();
