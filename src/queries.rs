pub const CREATE_WARRIOR: &str = r#"
WITH inserted_warrior AS (
    INSERT INTO warriors (name, dob)
    VALUES ($1, $2)
    RETURNING id
),
inserted_skills AS (
    INSERT INTO skills (name)
    SELECT skill_name
    FROM unnest(($3::text[])) AS skill_name
    ON CONFLICT (name) DO NOTHING
    RETURNING id, name
),
inserted_warrior_skills AS (
    INSERT INTO warrior_skills (warrior_id, skill_id)
    SELECT inserted_warrior.id, COALESCE(existing_skills.id, new_skill.id)
    FROM inserted_warrior
    CROSS JOIN inserted_skills
    LEFT JOIN skills existing_skills ON existing_skills.name = inserted_skills.name
    LEFT JOIN (
        SELECT id, name
        FROM inserted_skills
    ) AS new_skill ON true
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