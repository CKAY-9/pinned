CREATE TABLE users (
  id          SERIAL NOT NULL PRIMARY KEY,
  oauth_id    TEXT,
  username    TEXT,
  avatar      TEXT,
  bio         TEXT,
  token       TEXT,
  collections INTEGER[]
);
