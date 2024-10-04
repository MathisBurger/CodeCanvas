CREATE TABLE solutions (
    id SERIAL PRIMARY KEY,
    submitter_id INTEGER NOT NULL,
    assignment_id INTEGER NOT NULL REFERENCES assignments(id),
    approved_by_tutor BOOLEAN NOT NULL DEFAULT false
);
