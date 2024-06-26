SELECT 'CREATE DATABASE warriors' 
WHERE NOT EXISTS (SELECT FROM pg_database WHERE datname = 'warriors')\gexec

CREATE TABLE IF NOT EXISTS warriors (
  id SERIAL PRIMARY KEY,
  name VARCHAR NOT NULL,
  dob VARCHAR NOT NULL
);

CREATE TABLE IF NOT EXISTS skills (
  id SERIAL PRIMARY KEY,
  name VARCHAR NOT NULL UNIQUE
);

CREATE TABLE IF NOT EXISTS warrior_skills (
  id SERIAL PRIMARY KEY,
  warrior_id INT NOT NULL,
  skill_id INT NOT NULL,
  FOREIGN KEY (warrior_id) REFERENCES warriors(id),
  FOREIGN KEY (skill_id) REFERENCES skills(id)
);

CREATE INDEX idx_skills_name ON skills (name);
CREATE INDEX idx_warrior_skills_warrior_id ON warrior_skills (warrior_id);
CREATE INDEX idx_warrior_skills_skill_id ON warrior_skills (skill_id);