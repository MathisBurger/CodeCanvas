CREATE TABLE assignment_wishes (
    id SERIAL PRIMARY KEY,
    title VARCHAR(255) NOT NULL,
    description TEXT NOT NULL,
    group_id INTEGER NOT NULL REFERENCES groups(id)
);
