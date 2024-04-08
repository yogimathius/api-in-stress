-- Add migration script here
CREATE TABLE IF NOT EXISTS warriors (
  id SERIAL PRIMARY KEY,
  name VARCHAR NOT NULL,
  dob VARCHAR NOT NULL
);