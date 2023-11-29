CREATE TABLE users (
  id          SERIAL NOT NULL PRIMARY KEY,
  username    TEXT,
  avatar      TEXT,
  bio         TEXT,
  token       TEXT,
  collections TEXT[]
);
