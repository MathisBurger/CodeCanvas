ALTER TABLE group_members
ADD CONSTRAINT fk_group_id
FOREIGN KEY (group_id)
REFERENCES groups(id)
ON DELETE CASCADE;
