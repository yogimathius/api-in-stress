pub const CREATE_WARRIOR: &str = r#"
WITH inserted_warrior AS (
    INSERT INTO warriors (name, dob)
    VALUES ($1, $2)
    RETURNING id
),
inserted_warrior_skills AS (
    INSERT INTO warrior_skills (warrior_id, skill_id)
    SELECT inserted_warrior.id, s.id
    FROM inserted_warrior
    CROSS JOIN unnest($3::text[]) AS skill_name
    JOIN skills s ON s.name = skill_name
)
SELECT id from inserted_warrior;
"#;

pub const GET_WARRIOR: &str = r#"
SELECT warriors.*,
    (SELECT array_agg(skills.name)
        FROM skills
        INNER JOIN warrior_skills ON skills.id = warrior_skills.skill_id
        WHERE warrior_skills.warrior_id = warriors.id
    ) AS fight_skills
FROM warriors
WHERE id = $1;"#;

pub const SEARCH_WARRIORS: &str = r#"
SELECT warriors.*,
    (SELECT array_agg(skills.name)
        FROM skills
        INNER JOIN warrior_skills ON skills.id = warrior_skills.skill_id
        WHERE warrior_skills.warrior_id = warriors.id
    ) AS fight_skills
FROM warriors  LIMIT 50;"#;