-- Your SQL goes here
CREATE TABLE microphones (
  id INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT,
  model VARCHAR NOT NULL,
  manufacturer VARCHAR NOT NULL,
  description TEXT NOT NULL,
  main_image VARCHAR NULL
)