ALTER TABLE assignments ADD runner_cpu VARCHAR(5) NOT NULL DEFAULT '.5';
ALTER TABLE assignments ADD runner_memory VARCHAR(5) NOT NULL DEFAULT '20m';
ALTER TABLE assignments ADD runner_timeout VARCHAR(5) NOT NULL DEFAULT '60s';
