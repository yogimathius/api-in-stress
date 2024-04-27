CREATE EXTENSION IF NOT EXISTS "uuid-ossp";

CREATE TABLE IF NOT EXISTS warriors_2 (
  id VARCHAR PRIMARY KEY, 
  name VARCHAR NOT NULL,
  dob VARCHAR NOT NULL
);

CREATE TABLE IF NOT EXISTS skills_2 (
  id SERIAL PRIMARY KEY,
  name VARCHAR NOT NULL
);

CREATE TABLE IF NOT EXISTS warrior_skills_2 (
  id SERIAL PRIMARY KEY,
  warrior_id VARCHAR NOT NULL,
  skill_id INT NOT NULL,
  FOREIGN KEY (warrior_id) REFERENCES warriors_2(id),
  FOREIGN KEY (skill_id) REFERENCES skills_2(id)
);

CREATE INDEX idx_skills_name ON skills_2 (name);
CREATE INDEX idx_warrior_skills_warrior_id ON warrior_skills_2 (warrior_id);
CREATE INDEX idx_warrior_skills_skill_id ON warrior_skills_2 (skill_id);