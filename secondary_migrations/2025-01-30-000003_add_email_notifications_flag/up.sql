ALTER TABLE public.users ADD COLUMN email_notifications_enabled BOOLEAN DEFAULT false NOT NULL; 
ALTER TABLE public.users ADD COLUMN email TEXT; 