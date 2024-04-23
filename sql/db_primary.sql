SELECT 'CREATE DATABASE warriors' 
WHERE NOT EXISTS (SELECT FROM pg_database WHERE datname = 'warriors')\gexec

\c warriors

CREATE EXTENSION IF NOT EXISTS postgres_fdw;

CREATE SERVER db_part1 FOREIGN DATA WRAPPER postgres_fdw
    OPTIONS (host 'db_part1', dbname 'warriors');

CREATE SERVER db_part2 FOREIGN DATA WRAPPER postgres_fdw
    OPTIONS (host 'db_part2', dbname 'warriors');

CREATE USER MAPPING FOR CURRENT_USER SERVER db_part1
    OPTIONS (user 'postgres', password 'pass123');

CREATE USER MAPPING FOR CURRENT_USER SERVER db_part2
    OPTIONS (user 'postgres', password 'pass123');
  
GRANT USAGE ON SCHEMA public TO postgres;

GRANT USAGE ON FOREIGN SERVER db_part1 TO postgres;
GRANT USAGE ON FOREIGN SERVER db_part2 TO postgres;

CREATE FOREIGN TABLE warriors_1 (
  id VARCHAR,  
  name VARCHAR,
  dob VARCHAR
) SERVER db_part1;

CREATE FOREIGN TABLE skills_1 (
  id INT,
  name VARCHAR
) SERVER db_part1;

CREATE FOREIGN TABLE warrior_skills_1 (
  id INT,
  warrior_id VARCHAR,
  skill_id INT
) SERVER db_part1;

CREATE FOREIGN TABLE warriors_2 (
  id VARCHAR,
  name VARCHAR,
  dob VARCHAR
) SERVER db_part2;

CREATE FOREIGN TABLE skills_2 (
  id INT,
  name VARCHAR
) SERVER db_part2;

CREATE FOREIGN TABLE warrior_skills_2 (
  id INT,
  warrior_id VARCHAR,
  skill_id INT
) SERVER db_part2;

CREATE VIEW warriors AS
SELECT * FROM warriors_1
UNION ALL
SELECT * FROM warriors_2;

CREATE VIEW skills AS
SELECT * FROM skills_1;

CREATE VIEW warrior_skills AS
SELECT * FROM warrior_skills_1
UNION ALL
SELECT * FROM warrior_skills_2;

CREATE OR REPLACE FUNCTION public.insert_warrior_round_robin() RETURNS TRIGGER AS $func$
DECLARE
    last_inserted_table VARCHAR := 'warriors_1';
BEGIN
    -- Insert into the last inserted table
    IF last_inserted_table = 'warriors_1' THEN
        INSERT INTO warriors_2 (id, name, dob) VALUES (NEW.id, NEW.name, NEW.dob);
        last_inserted_table := 'warriors_2';
    ELSE
        INSERT INTO warriors_1 (id, name, dob) VALUES (NEW.id, NEW.name, NEW.dob);
        last_inserted_table := 'warriors_1';
    END IF;
    RETURN NEW;
END;
$func$ LANGUAGE plpgsql;

CREATE TRIGGER insert_warrior_trigger
INSTEAD OF INSERT OR UPDATE ON public.warriors
    FOR EACH ROW
    EXECUTE FUNCTION insert_warrior_round_robin();

CREATE OR REPLACE FUNCTION public.insert_warrior_skills_round_robin() RETURNS TRIGGER AS $func$
DECLARE
    last_inserted_table VARCHAR := 'warrior_skills_1';
BEGIN
    -- Insert into the last inserted table
    IF last_inserted_table = 'warrior_skills_1' THEN
        INSERT INTO warrior_skills_2 (id, warrior_id, skill_id) VALUES (NEW.id, NEW.warrior_id, NEW.skill_id);
        last_inserted_table := 'warrior_skills_2';
    ELSE
        INSERT INTO warrior_skills_1 (id, warrior_id, skill_id) VALUES (NEW.id, NEW.warrior_id, NEW.skill_id);
        last_inserted_table := 'warrior_skills_1';
    END IF;
    RETURN NEW;
END;
$func$ LANGUAGE plpgsql;

CREATE TRIGGER insert_warrior_skills_trigger
INSTEAD OF INSERT OR UPDATE ON public.warrior_skills
    FOR EACH ROW
    EXECUTE FUNCTION insert_warrior_skills_round_robin();
