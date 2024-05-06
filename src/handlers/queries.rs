pub const GET_WARRIOR: &str = r#"
SELECT 
    warriors.*
FROM 
    warriors
WHERE 
    warriors.id = $1
GROUP BY 
    warriors.id, warriors.name, warriors.dob, warriors.fight_skills;"#;

pub const SEARCH_WARRIORS: &str = r#"
SELECT 
    warriors.*
FROM 
    warriors
WHERE 
    warriors.name ILIKE $1
GROUP BY 
    warriors.id, warriors.name, warriors.dob, warriors.fight_skills
LIMIT 
    50;"#;
