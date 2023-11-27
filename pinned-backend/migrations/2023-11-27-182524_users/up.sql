CREATE TABLE users (
  id          SERIAL NOT NULL AUTO_INCREMENT PRIMARY KEY,
  username    VARCHAR,
  avatar      VARCHAR,
  bio         VARCHAR,
  token       VARCHAR,
  collections VARCHAR[],
);
