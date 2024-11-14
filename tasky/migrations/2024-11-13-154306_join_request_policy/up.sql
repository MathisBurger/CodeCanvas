CREATE TYPE join_request_policy AS ENUM ('open', 'request', 'closed');
ALTER TABLE groups ADD join_policy join_request_policy NOT NULL DEFAULT 'request';
