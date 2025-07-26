CREATE TABLE public.user_subscriptions (
    id SERIAL PRIMARY KEY,
    github_id BIGINT NOT NULL REFERENCES public.users(github_id),
    purpose TEXT,
    stack_level TEXT,
    technology TEXT,
    created_at TIMESTAMPTZ DEFAULT (now() AT TIME ZONE 'utc') NOT NULL,
    UNIQUE(github_id, purpose),
    UNIQUE(github_id, stack_level),
    UNIQUE(github_id, technology)
); 