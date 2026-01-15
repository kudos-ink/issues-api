CREATE TABLE public.notification_schedule (
    id SERIAL PRIMARY KEY,
    next_run TIMESTAMPTZ NOT NULL,
    last_run TIMESTAMPTZ,
    created_at TIMESTAMPTZ DEFAULT (now() AT TIME ZONE 'utc') NOT NULL,
    updated_at TIMESTAMPTZ DEFAULT (now() AT TIME ZONE 'utc') NOT NULL
);

-- Insert initial schedule
INSERT INTO public.notification_schedule (next_run)
VALUES ((now() AT TIME ZONE 'utc')); 