CREATE TYPE assignment_language AS ENUM ('java', 'golang');
CREATE TABLE assignments (
    id SERIAL PRIMARY KEY,
    title VARCHAR(100) NOT NULL,
    due_date TIMESTAMP NOT NULL,
    group_id INTEGER NOT NULL REFERENCES groups(id),
    description TEXT NOT NULL,
    language assignment_language NOT NULL
);
