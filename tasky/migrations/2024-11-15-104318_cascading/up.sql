ALTER TABLE assignments
DROP CONSTRAINT assignments_group_id_fkey,
ADD CONSTRAINT assignments_group_id_fkey
FOREIGN KEY (group_id) REFERENCES groups(id) ON DELETE CASCADE;

ALTER TABLE assignment_wishes
DROP CONSTRAINT assignment_wishes_group_id_fkey,
ADD CONSTRAINT assignment_wishes_group_id_fkey
FOREIGN KEY (group_id) REFERENCES groups(id) ON DELETE CASCADE;

ALTER TABLE code_comments
DROP CONSTRAINT code_comments_group_id_fkey,
ADD CONSTRAINT code_comments_group_id_fkey
FOREIGN KEY (group_id) REFERENCES groups(id) ON DELETE CASCADE;

ALTER TABLE code_comments
DROP CONSTRAINT code_comments_solution_id_fkey,
ADD CONSTRAINT code_comments_solution_id_fkey
FOREIGN KEY (solution_id) REFERENCES solutions(id) ON DELETE CASCADE;

ALTER TABLE group_join_requests
DROP CONSTRAINT group_join_requests_group_id_fkey,
ADD CONSTRAINT group_join_requests_group_id_fkey
FOREIGN KEY (group_id) REFERENCES groups(id) ON DELETE CASCADE;

ALTER TABLE solutions
DROP CONSTRAINT solutions_group_id_fkey,
ADD CONSTRAINT solutions_group_id_fkey
FOREIGN KEY (group_id) REFERENCES groups(id) ON DELETE CASCADE;

ALTER TABLE solutions
DROP CONSTRAINT solutions_assignment_id_fkey,
ADD CONSTRAINT solutions_assignment_id_fkey
FOREIGN KEY (assignment_id) REFERENCES assignments(id) ON DELETE CASCADE;
