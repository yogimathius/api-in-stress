pub const GET_WARRIOR: &str = r#"
SELECT 
    warriors.*, 
    array_agg(skills.name) AS fight_skills
FROM 
    warriors
LEFT JOIN 
    warrior_skills ON warriors.id = warrior_skills.warrior_id
LEFT JOIN 
    skills ON warrior_skills.skill_id = skills.id
WHERE 
    warriors.id = $1
GROUP BY 
    warriors.id, warriors.name, warriors.dob;"#;

pub const SEARCH_WARRIORS: &str = r#"
SELECT 
    warriors.*, 
    array_agg(skills.name) AS fight_skills
FROM 
    warriors
LEFT JOIN 
    warrior_skills ON warriors.id = warrior_skills.warrior_id
LEFT JOIN 
    skills ON warrior_skills.skill_id = skills.id
WHERE 
    warriors.name = $1
GROUP BY 
    warriors.id, warriors.name, warriors.dob
LIMIT 
    50;"#;
