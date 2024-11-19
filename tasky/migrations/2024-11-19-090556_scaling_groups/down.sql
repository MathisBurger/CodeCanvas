ALTER TABLE groups
ADD COLUMN members INTEGER[] NOT NULL; -- Assuming members was an INTEGER array

-- 2. Populate the members column by aggregating data from group_members
UPDATE groups
SET members = subquery.members_array
FROM (
    SELECT
        group_id,
        ARRAY_AGG(member_id) AS members_array
    FROM
        group_members
    GROUP BY
        group_id
) AS subquery
WHERE groups.id = subquery.group_id;

DROP TABLE group_members;
