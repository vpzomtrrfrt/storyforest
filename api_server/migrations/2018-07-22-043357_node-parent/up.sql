-- Your SQL goes here

ALTER TABLE node ADD COLUMN parent INTEGER REFERENCES node(id);
