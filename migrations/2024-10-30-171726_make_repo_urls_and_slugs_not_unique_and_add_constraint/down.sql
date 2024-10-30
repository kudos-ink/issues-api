ALTER TABLE repositories DROP CONSTRAINT IF EXISTS unique_url_slug_project_id;

ALTER TABLE repositories ADD CONSTRAINT repositories_slug_key UNIQUE (slug);
ALTER TABLE repositories ADD CONSTRAINT repositories_url_key UNIQUE (url);
