CREATE TABLE users (
  id          SERIAL NOT NULL PRIMARY KEY,
  oauth_id    TEXT NOT NULL,
  username    TEXT NOT NULL,
  avatar      TEXT NOT NULL,
  bio         TEXT NOT NULL,
  token       TEXT NOT NULL,
  collections INTEGER[] NOT NULL
);
