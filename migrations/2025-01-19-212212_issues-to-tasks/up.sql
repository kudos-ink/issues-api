INSERT INTO tasks (
    id,
    number,
    repository_id,
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
        WHEN i.open THEN 'open'
        ELSE 'completed'
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