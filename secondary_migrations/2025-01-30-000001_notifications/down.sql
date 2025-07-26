DROP TRIGGER IF EXISTS task_notifications_trigger ON public.tasks;
DROP FUNCTION IF EXISTS public.handle_task_notifications();
DROP TABLE IF EXISTS public.notifications; 