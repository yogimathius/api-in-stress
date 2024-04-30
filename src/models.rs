use serde::{Deserialize, Serialize};
use sqlx::{Decode, Encode, FromRow};

#[derive(Debug, serde::Deserialize, serde::Serialize, Clone)]
pub struct NewWarrior {
    pub name: String,
    pub dob: String,
    pub fight_skills: Vec<String>,
}

#[derive(Debug, Deserialize, Serialize, FromRow)]
pub struct Warrior {
    pub id: String,
    pub name: String,
    pub dob: String,
    pub fight_skills: Option<Vec<String>>,
}

#[derive(Debug, Deserialize, Serialize, Encode, Decode, FromRow, PartialEq, Eq, Hash, Clone)]
pub struct DbSkill {
    pub id: i32,
    pub name: String,
}

#[derive(Debug, Deserialize, Serialize, Encode, Decode, FromRow, PartialEq, Eq, Hash, Clone)]
pub struct WarriorSkill {
    pub skill_id: i32,
    pub warrior_id: String,
}
