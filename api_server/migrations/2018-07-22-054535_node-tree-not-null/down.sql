-- This file should undo anything in `up.sql`

ALTER TABLE node ALTER COLUMN tree DROP NOT NULL;
