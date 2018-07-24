-- Your SQL goes here

CREATE TABLE account (
	id SERIAL PRIMARY KEY,
	name VARCHAR(20) NOT NULL,
	passhash VARCHAR(60)
);

ALTER TABLE node ADD COLUMN author INTEGER REFERENCES account(id);
ALTER TABLE tree ADD COLUMN creator INTEGER REFERENCES account(id);
