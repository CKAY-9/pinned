CREATE TABLE comments (
  id          SERIAL NOT NULL PRIMARY KEY,
  post        INTEGER NOT NULL,
  creator     SERIAL NOT NULL,
  content     TEXT NOT NULL,
  posted      TEXT NOT NULL,
  likes       INTEGER[] NOT NULL,
  dislikes    INTEGER[] NOT NULL
);
