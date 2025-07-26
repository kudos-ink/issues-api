DELETE FROM tasks
WHERE type = 'dev'
  AND id IN (SELECT id FROM issues);

DELETE FROM users_projects_roles
WHERE role_id = 2
  AND user_id IN (SELECT DISTINCT assignee_id FROM issues WHERE assignee_id IS NOT NULL);
