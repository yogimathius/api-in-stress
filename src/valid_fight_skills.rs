use std::collections::{HashMap, HashSet};

use chrono::NaiveDate;

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

    pub fn get_valid_skills(&self, warrior: &NewWarrior) -> Result<Vec<i32>, &'static str> {
        if let Err(_) = NaiveDate::parse_from_str(&warrior.dob, "%Y-%m-%d") {
            return Err("Invalid date format");
        }

        if !(1..=20).contains(&warrior.fight_skills.len()) {
            return Err("Invalid number of skills");
        }

        let mut unique_skill_ids = HashSet::new();

        for skill in &warrior.fight_skills {
            if let Some(skill_id) = self.skills.get(skill).cloned() {
                if !unique_skill_ids.insert(skill_id) {
                    return Err("Duplicate skills");
                }
            } else {
                return Err("Not all skills are valid");
            }
        }

        Ok(unique_skill_ids.into_iter().collect())
    }
}
