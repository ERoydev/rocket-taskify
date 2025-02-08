-- 1. Enable the pg_cron extension (if available)
CREATE EXTENSION IF NOT EXISTS pg_cron;

CREATE TABLE IF NOT EXISTS "TASK" (
    id SERIAL PRIMARY KEY,
    title VARCHAR(255) NOT NULL,
    description TEXT NOT NULL,
    priority VARCHAR(50) DEFAULT 'low',
    due_date BIGINT NOT NULL,       -- UNIX timestamp
    is_completed BOOLEAN DEFAULT FALSE,
    is_critical BOOLEAN DEFAULT FALSE
);

CREATE OR REPLACE FUNCTION update_task_priorities() RETURNS void AS $$
DECLARE
    task_id INT;
    task_is_completed BOOLEAN;
    task_is_critical BOOLEAN;
    task_due_date BIGINT;
    task_priority VARCHAR(50);
    now_timestamp BIGINT;
    difference BIGINT;
    ONE_DAY_IN_SECONDS BIGINT := 86400;
    IMMEDIATE_THRESHOLD BIGINT := ONE_DAY_IN_SECONDS; 
    HIGH_THRESHOLD BIGINT := ONE_DAY_IN_SECONDS * 2;    
    MEDIUM_THRESHOLD BIGINT := ONE_DAY_IN_SECONDS * 3;   
BEGIN
    FOR task_id, task_is_completed, task_is_critical, task_due_date IN
        SELECT id, is_completed, is_critical, due_date FROM "TASK"
    LOOP
        task_priority := 'low';

        IF task_is_completed THEN
            task_priority := 'low';  
        ELSIF task_is_critical THEN
            task_priority := 'immediate';  
        ELSE
            SELECT EXTRACT(EPOCH FROM NOW() AT TIME ZONE 'Europe/Sofia') INTO now_timestamp;
            difference := task_due_date - now_timestamp;

            IF difference <= 0 THEN
                task_priority := 'expired';    -- Task is expired
            ELSIF difference <= IMMEDIATE_THRESHOLD THEN
                task_priority := 'immediate';  -- Due within 1 day
            ELSIF difference <= HIGH_THRESHOLD THEN
                task_priority := 'high';       -- Due within 2 days
            ELSIF difference <= MEDIUM_THRESHOLD THEN
                task_priority := 'medium';     -- Due within 3 days
            END IF;
        END IF;

        -- Update the task with the calculated priority
        UPDATE "TASK"
        SET priority = task_priority
        WHERE id = task_id;
    END LOOP;
END;
$$ LANGUAGE plpgsql;

-- 4. Schedule the stored procedure to run every 24 hours using pg_cron.
-- This schedules the job to run at midnight every day.
SELECT cron.schedule(
    'update_task_priorities_daily',  -- Job name
    '0 0 * * *',                     -- Cron expression: at 00:00 (midnight) every day
    'SELECT update_task_priorities();'
);
