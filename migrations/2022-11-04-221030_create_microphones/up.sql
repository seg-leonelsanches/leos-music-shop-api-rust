-- Your SQL goes here
CREATE TABLE microphones (
  id INT NOT NULL PRIMARY KEY,
  model VARCHAR NOT NULL,
  manufacturer VARCHAR NOT NULL,
  description TEXT NOT NULL,
  main_image VARCHAR NULL
)