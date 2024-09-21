CREATE TABLE group_join_requests (
    id SERIAL PRIMARY KEY,
    requestor INTEGER NOT NULL,
    group_id INTEGER NOT NULL REFERENCES groups(id)
);
