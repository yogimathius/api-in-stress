CREATE EXTENSION IF NOT EXISTS "uuid-ossp";

CREATE TABLE IF NOT EXISTS warriors_1 (
  id VARCHAR PRIMARY KEY, 
  name VARCHAR NOT NULL,
  dob VARCHAR NOT NULL
);

CREATE TABLE IF NOT EXISTS skills_1 (
  id SERIAL PRIMARY KEY,
  name VARCHAR NOT NULL
);

CREATE TABLE IF NOT EXISTS warrior_skills_1 (
  id SERIAL PRIMARY KEY,
  warrior_id VARCHAR NOT NULL,
  skill_id INT NOT NULL,
  FOREIGN KEY (warrior_id) REFERENCES warriors_1(id),
  FOREIGN KEY (skill_id) REFERENCES skills_1(id)
);


CREATE INDEX idx_warriors_id ON warriors_1 (id);
CREATE INDEX idx_warrior_skills_warrior_id_skill_id ON warrior_skills_1 (warrior_id, skill_id);
CREATE INDEX idx_skills_id_1 ON skills_1 (id);
