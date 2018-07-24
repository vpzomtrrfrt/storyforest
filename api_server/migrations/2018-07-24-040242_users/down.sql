-- This file should undo anything in `up.sql`

DROP TABLE account;
ALTER TABLE node DROP COLUMN author;
ALTER TABLE tree DROP COLUMN creator;
