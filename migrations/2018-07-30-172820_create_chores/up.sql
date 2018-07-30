CREATE TABLE chores(
    id          SERIAL  PRIMARY KEY,
    name        TEXT    NOT NULL,
    interval    INTEGER   NOT NULL,
    userid      INTEGER  REFERENCES users(id)
)
