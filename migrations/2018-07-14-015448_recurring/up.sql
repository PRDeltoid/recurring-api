-- Your SQL goes here
CREATE TABLE chore_entries (
	id integer PRIMARY KEY,
	date date NOT NULL,
	choreid integer REFERENCES chores(id) NOT NULL,
	userid  integer REFERENCES users(id) NOT NULL
);

CREATE TABLE chores (
	id integer PRIMARY KEY,
	name text NOT NULL,
	interval_time integer NOT NULL,
	userid  integer REFERENCES users(id) NOT NULL
);

CREATE TABLE users (
	id integer PRIMARY KEY,
	name text NOT NULL,
);