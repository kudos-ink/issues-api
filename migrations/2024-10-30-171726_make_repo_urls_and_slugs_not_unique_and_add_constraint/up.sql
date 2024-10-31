ALTER TABLE repositories DROP CONSTRAINT IF EXISTS repositories_slug_key;
ALTER TABLE repositories DROP CONSTRAINT IF EXISTS repositories_url_key;

ALTER TABLE repositories ADD CONSTRAINT unique_url_slug_project_id UNIQUE (url, slug, project_id);
