-- This file should undo anything in `up.sql`


ALTER TABLE to_do DROP COLUMN user_id;
DROP TABLE users
