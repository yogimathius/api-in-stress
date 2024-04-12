SELECT 'CREATE DATABASE warriors' 
WHERE NOT EXISTS (SELECT FROM pg_database WHERE datname = 'warriors')\gexec

\c warriors

CREATE EXTENSION postgres_fdw;

CREATE SERVER db_part1 FOREIGN DATA WRAPPER postgres_fdw
    OPTIONS (host 'db_part1', dbname 'warriors');

CREATE SERVER db_part2 FOREIGN DATA WRAPPER postgres_fdw
    OPTIONS (host 'db_part2', dbname 'warriors');

CREATE USER MAPPING FOR CURRENT_USER SERVER db_part1
    OPTIONS (user 'postgres', password 'pass123');

CREATE USER MAPPING FOR CURRENT_USER SERVER db_part2
    OPTIONS (user 'postgres', password 'pass123');
  
CREATE FOREIGN TABLE warriors_1 (
  id INT,
  name VARCHAR,
  dob VARCHAR
) SERVER db_part1;

CREATE FOREIGN TABLE skills_1 (
  id INT,
  name VARCHAR
) SERVER db_part1;

CREATE FOREIGN TABLE warrior_skills_1 (
  id INT,
  warrior_id INT,
  skill_id INT
) SERVER db_part1;

CREATE FOREIGN TABLE warriors_2 (
  id INT,
  name VARCHAR,
  dob VARCHAR
) SERVER db_part2;

CREATE FOREIGN TABLE skills_2 (
  id INT,
  name VARCHAR
) SERVER db_part2;

CREATE FOREIGN TABLE warrior_skills_2 (
  id INT,
  warrior_id INT,
  skill_id INT
) SERVER db_part2;

CREATE VIEW warriors AS
SELECT * FROM warriors_1
UNION ALL
SELECT * FROM warriors_2;

CREATE VIEW skills AS
SELECT * FROM skills_1
UNION ALL
SELECT * FROM skills_2;

CREATE VIEW warrior_skills AS
SELECT * FROM warrior_skills_1
UNION ALL
SELECT * FROM warrior_skills_2;