DELETE FROM tasks
WHERE type = 'dev'
  AND id IN (SELECT id FROM issues);