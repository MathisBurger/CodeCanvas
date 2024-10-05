ALTER TABLE solutions ADD approved_by_tutor BOOLEAN NOT NULL DEFAULT false;
ALTER TABLE solutions DROP approval_status;
