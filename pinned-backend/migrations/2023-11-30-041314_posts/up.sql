CREATE TABLE posts (
  id          SERIAL NOT NULL PRIMARY KEY,
  file_id     TEXT NOT NULL,
  description TEXT NOT NULL DEFAULT "No description provided.",
  creator     SERIAL NOT NULL,
  likes       INTEGER[] NOT NULL DEFAULT {},
  dislikes    INTEGER[] NOT NULL DEFAULT {},
  comments    INTEGER[] NOT NULL DEFAULT {}
);
