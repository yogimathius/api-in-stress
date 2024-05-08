use std::collections::HashMap;

use chrono::NaiveDate;
use serde_valid::Validate;

use crate::models::{DbSkill, NewWarrior};

#[derive(Clone)]
pub struct DbFightSkills {
    skills: HashMap<String, i32>,
}

impl DbFightSkills {
    pub async fn new(db_pool: sqlx::PgPool) -> Self {
        let mut skills = HashMap::new();

        let skill_list: Vec<DbSkill> = sqlx::query_as("SELECT * FROM skills")
            .fetch_all(&db_pool)
            .await
            .expect("Failed to fetch skills from the database");

        for skill in skill_list {
            skills.insert(skill.name, skill.id);
        }
        DbFightSkills { skills }
    }

    pub fn get_valid_skills(&self, warrior: &NewWarrior) -> Result<Vec<i32>, String> {
        let _ = warrior.validate().map_err(|e| e.to_string())?;
        let _ = warrior
            .dob
            .parse::<NaiveDate>()
            .map_err(|_| "Invalid date of birth".to_string())?;

        let skill_ids: Result<Vec<_>, _> = warrior
            .fight_skills
            .iter()
            .map(|skill| {
                self.skills
                    .get(skill)
                    .map(|&id| id)
                    .ok_or_else(|| format!("Invalid skill: {}", skill))
            })
            .collect();

        skill_ids
    }
}
