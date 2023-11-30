CREATE TABLE comments (
  id          SERIAL NOT NULL PRIMARY KEY,
  creator     SERIAL NOT NULL,
  content     TEXT,
  likes       INTEGER[],
  dislikes    INTEGER[]
);
