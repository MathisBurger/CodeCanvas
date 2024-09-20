-- Your SQL goes here
CREATE TABLE groups (
    id SERIAL PRIMARY KEY,
    title VARCHAR NOT NULL,
    members INTEGER[] NOT NULL,
    tutor INTEGER NOT NULL
);