CREATE TABLE assignment_completions (
    assignment_id INTEGER REFERENCES assignments(id) ON DELETE CASCADE,
    member_id INTEGER NOT NULL,
    PRIMARY KEY (assignment_Id, member_id)
);

INSERT INTO assignment_completions (assignment_id, member_id)
SELECT
    a.id AS assignment_id,
    unnest(a.completed_by) AS member_id
FROM
    assignments a;

ALTER TABLE assignments DROP COLUMN completed_by;
