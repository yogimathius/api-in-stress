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
FROM warriors  
WHERE 
    warriors.name ILIKE '%' || $1 || '%'
LIMIT 50;"#;
