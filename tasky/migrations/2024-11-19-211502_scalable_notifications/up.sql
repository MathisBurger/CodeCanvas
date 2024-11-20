CREATE TABLE notification_targets (
    notification_id INTEGER NOT NULL REFERENCES notifications(id),
    user_id INTEGER NOT NULL,
    PRIMARY KEY (notification_id, user_id)
);

INSERT INTO notification_targets (notification_id, user_id)
SELECT
    n.id AS notification_id,
    unnest(n.targeted_users) AS user_id
FROM
    notifications n;

ALTER TABLE notifications DROP COLUMN targeted_users;
