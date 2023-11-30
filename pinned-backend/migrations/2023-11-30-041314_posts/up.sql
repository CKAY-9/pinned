CREATE TABLE posts (
  id          SERIAL NOT NULL PRIMARY KEY,
  file_id     TEXT,
  description TEXT NOT NULL,
  creator     SERIAL NOT NULL,
  likes       INTEGER[],
  dislikes    INTEGER[],
  comments    INTEGER[]
);
