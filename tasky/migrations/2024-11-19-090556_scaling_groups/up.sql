CREATE TABLE group_members (
    group_id INTEGER NOT NULL,
    member_id INTEGER NOT NULL,
    PRIMARY KEY (group_id, member_id)
);

INSERT INTO group_members (group_id, member_id)
SELECT
    g.id AS group_id,
    unnest(g.members) AS member_id
FROM
    groups g;

ALTER TABLE groups DROP COLUMN members;
