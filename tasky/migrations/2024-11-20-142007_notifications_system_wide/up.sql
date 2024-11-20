ALTER TABLE notifications ADD COLUMN show_until TIMESTAMP;
ALTER TABLE notifications ADD COLUMN system_wide BOOLEAN NOT NULL DEFAULT false;
