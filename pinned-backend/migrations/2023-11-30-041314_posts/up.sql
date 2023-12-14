CREATE TABLE posts (
  id          SERIAL NOT NULL PRIMARY KEY,
  title       TEXT NOT NULL,
  file_id     TEXT NOT NULL,
  description TEXT NOT NULL,
  posted      TEXT NOT NULL,
  creator     SERIAL NOT NULL,
  likes       INTEGER[] NOT NULL, 
  dislikes    INTEGER[] NOT NULL,
  comments    INTEGER[] NOT NULL
);
