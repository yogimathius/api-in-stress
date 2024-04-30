use std::collections::HashMap;

use crate::models::DbSkill;

#[derive(Clone)]
pub struct ValidFightSkills {
    skills: HashMap<String, i32>,
}

impl ValidFightSkills {
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
        ValidFightSkills { skills }
    }

    pub fn are_valid_skills(&self, skills: &Vec<std::string::String>) -> bool {
        !skills.is_empty()
            && skills.len() <= 20
            && skills
                .iter()
                .all(|skill| self.skills.contains_key(skill) && skill.len() <= 250)
    }

    pub fn filter_warrior_skills(
        &self,
        skills: &[String], // Changed to slice
    ) -> Vec<i32> {
        skills
            .iter()
            .filter_map(|skill| self.skills.get(skill).map(|id| *id))
            .collect()
    }
}
