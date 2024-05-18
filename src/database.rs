use chrono::NaiveDate;
use sqlx::postgres::PgPoolOptions;
use std::{collections::HashMap, time::Duration};

use crate::models::{DbSkill, NewWarrior};
use serde_valid::Validate;

#[derive(Clone)]
pub struct Database {
    pub pool: sqlx::PgPool,
    pub primary_pool: sqlx::PgPool,
    pub skills: HashMap<String, i32>,
}

impl Database {
    pub async fn new() -> Self {
        let db_connection_str = std::env::var("DATABASE_URL")
            .unwrap_or_else(|_| "postgres://postgres:password@localhost".to_string());

        let pool: sqlx::Pool<sqlx::Postgres> = create_pool(db_connection_str).await;

        let primary_db_connection_str =
            "postgres://postgres:pass123@postgres:5432/warriors".to_string();

        let primary_pool = create_pool(primary_db_connection_str).await;
        let mut skills = HashMap::new();

        let skill_list: Vec<DbSkill> = sqlx::query_as("SELECT * FROM skills")
            .fetch_all(&primary_pool)
            .await
            .expect("Failed to fetch skills from the database");

        for skill in skill_list {
            skills.insert(skill.name, skill.id);
        }
        Self {
            pool,
            primary_pool,
            skills,
        }
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

pub async fn create_pool(db_connection_str: String) -> sqlx::PgPool {
    PgPoolOptions::new()
        .max_connections(40000)
        .acquire_timeout(Duration::from_secs(10))
        .connect(&db_connection_str)
        .await
        .expect("can't connect to database")
}
