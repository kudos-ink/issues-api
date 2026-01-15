CREATE TABLE public.notifications (
    id SERIAL PRIMARY KEY,
    github_id BIGINT NOT NULL REFERENCES public.users(github_id),
    task_id INTEGER NOT NULL REFERENCES public.tasks(id),
    seen BOOLEAN DEFAULT FALSE,
    created_at TIMESTAMPTZ DEFAULT (now() AT TIME ZONE 'utc') NOT NULL,
    UNIQUE(github_id, task_id)
); 

-- Trigger function to insert notifications for matching user subscriptions
CREATE OR REPLACE FUNCTION public.handle_task_notifications()
RETURNS TRIGGER AS $$
BEGIN
    INSERT INTO public.notifications (github_id, task_id)
    SELECT DISTINCT us.github_id, NEW.id
    FROM public.user_subscriptions us
    LEFT JOIN public.repositories r ON r.id = NEW.repository_id
    JOIN public.projects p ON p.id = COALESCE(NEW.project_id, r.project_id)
    WHERE
        (us.purpose IS NOT NULL AND us.purpose = ANY(p.purposes))
        OR (us.stack_level IS NOT NULL AND us.stack_level = ANY(p.stack_levels))
        OR (us.technology IS NOT NULL AND us.technology = ANY(p.technologies))
    ON CONFLICT DO NOTHING;
    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

-- Create trigger to run after a new task is inserted
CREATE TRIGGER task_notifications_trigger
    AFTER INSERT ON public.tasks
    FOR EACH ROW
    EXECUTE FUNCTION public.handle_task_notifications(); 