CREATE TABLE code_comments (
    id SERIAL PRIMARY KEY,
    title VARCHAR(255) NOT NULL,
    content TEXT NOT NULL,
    commentor INTEGER NOT NULL,
    group_id INTEGER NOT NULL REFERENCES groups(id),
    solution_id INTEGER NOT NULL REFERENCES solutions(id)
);
