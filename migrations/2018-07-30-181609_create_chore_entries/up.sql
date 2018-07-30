CREATE TABLE chore_entries(
    id          SERIAL  PRIMARY KEY,
    date        DATE    NOT NULL,
    choreid     INTEGER  REFERENCES chores(id),
    userid      INTEGER  REFERENCES users(id)
)
