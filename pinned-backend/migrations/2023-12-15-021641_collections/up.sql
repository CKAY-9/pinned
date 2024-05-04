CREATE TABLE collections (
  id                      SERIAL NOT NULL PRIMARY KEY,
  name                    TEXT NOT NULL,
  description             TEXT NOT NULL,
  linked_posts            INTEGER[] NOT NULL,
  linked_comments         INTEGER[] NOT NULL,
  recommended_collections INTEGER[] NOT NULL,
  creator                 INTEGER NOT NULL,
  collaborators           INTEGER[] NOT NULL,
  likes                   INTEGER[] NOT NULL,
  dislikes                INTEGER[] NOT NULL
);
