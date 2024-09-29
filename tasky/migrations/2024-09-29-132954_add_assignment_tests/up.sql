ALTER TABLE assignments ADD completed_by INTEGER ARRAY NOT NULL DEFAULT '{}';
ALTER TABLE assignments ADD file_structure JSONB;
