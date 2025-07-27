-- Migrate issues to tasks

INSERT INTO tasks (
    id,
    number,
    repository_id,
    project_id,
    title,
    description,
    labels,
    open,
    is_certified,
    assignee_user_id,
    issue_created_at,
    issue_closed_at,
    created_at,
    updated_at,
    type,
    status,
    url
)
SELECT
    i.id,
    i.number,
    i.repository_id,
    r.project_id,
    i.title,
    i.description,
    i.labels,
    i.open,
    COALESCE(i.certified, false) AS is_certified,
    i.assignee_id AS assignee_user_id,
    i.issue_created_at,
    i.issue_closed_at,
    i.created_at,
    i.updated_at,
    'dev' AS type,
    CASE 
        WHEN i.open = TRUE AND i.assignee_id IS NULL THEN 'open'
        WHEN i.open = TRUE AND i.assignee_id IS NOT NULL THEN 'in-progress'
        WHEN i.open = FALSE THEN 'completed'
    END AS status,
    CONCAT(
        'https://github.com/',
        r.slug,
        '/issues/',
        i.number
    ) AS url
FROM 
    issues i
JOIN 
    repositories r 
ON 
    i.repository_id = r.id;

-- add CONTRIBUTOR role

INSERT INTO users_projects_roles (user_id, role_id)
SELECT DISTINCT assignee_id AS user_id, 2 AS role_id
FROM issues
WHERE assignee_id IS NOT NULL
ON CONFLICT (user_id, project_id, role_id) DO NOTHING;
