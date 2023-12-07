CREATE TABLE comments (
  id          SERIAL NOT NULL PRIMARY KEY,
  creator     SERIAL NOT NULL,
  content     TEXT NOT NULL,
  likes       INTEGER[] NOT NULL DEFAULT {},
  dislikes    INTEGER[] NOT NULL DEFAULT {}
);
