use std::collections::HashMap;

use crate::models::DbSkill;

#[derive(Clone)]
pub struct DbFightSkills {
    skills: HashMap<String, i32>,
}

impl DbFightSkills {
    pub async fn new(db_pool: sqlx::PgPool) -> Self {
        let mut skills = HashMap::new();

        // Initialize skills hashmap with the provided list of skills
        let skill_list: Vec<DbSkill> = sqlx::query_as("SELECT * FROM skills")
            .fetch_all(&db_pool)
            .await
            .expect("Failed to fetch skills from the database");

        for skill in skill_list {
            skills.insert(skill.name, skill.id);
        }
        DbFightSkills { skills }
    }

    pub fn get_valid_skills(&self, skills: &Vec<std::string::String>) -> Vec<i32> {
        // get all skill ids that match the skills provided
        skills
            .iter()
            .filter_map(|skill| self.skills.get(skill).map(|id| *id))
            .collect()
    }
}
