-- Drop columns
ALTER TABLE assignments
DROP COLUMN IF EXISTS created_at,
DROP COLUMN IF EXISTS updated_at;

ALTER TABLE assignment_wishes
DROP COLUMN IF EXISTS created_at,
DROP COLUMN IF EXISTS updated_at;

ALTER TABLE code_comments
DROP COLUMN IF EXISTS created_at,
DROP COLUMN IF EXISTS updated_at;

ALTER TABLE groups
DROP COLUMN IF EXISTS created_at,
DROP COLUMN IF EXISTS updated_at;

ALTER TABLE group_join_requests
DROP COLUMN IF EXISTS created_at,
DROP COLUMN IF EXISTS updated_at;

ALTER TABLE notifications
DROP COLUMN IF EXISTS created_at,
DROP COLUMN IF EXISTS updated_at;

ALTER TABLE solutions
DROP COLUMN IF EXISTS created_at,
DROP COLUMN IF EXISTS updated_at;

-- Drop triggers
DROP TRIGGER IF EXISTS set_timestamp ON assignments;
DROP TRIGGER IF EXISTS set_timestamp ON assignment_wishes;
DROP TRIGGER IF EXISTS set_timestamp ON code_comments;
DROP TRIGGER IF EXISTS set_timestamp ON groups;
DROP TRIGGER IF EXISTS set_timestamp ON group_join_requests;
DROP TRIGGER IF EXISTS set_timestamp ON notifications;
DROP TRIGGER IF EXISTS set_timestamp ON solutions;

-- Drop function
DROP FUNCTION IF EXISTS update_timestamp();
