CREATE TABLE IF NOT EXISTS warrior_skills (
  id SERIAL PRIMARY KEY,
  warrior_id INT NOT NULL,
  skill_id INT NOT NULL,
  FOREIGN KEY (warrior_id) REFERENCES warriors(id),
  FOREIGN KEY (skill_id) REFERENCES skills(id)
);