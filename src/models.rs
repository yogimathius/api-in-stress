use sqlx::{Decode, Encode, FromRow};
use serde::{Deserialize, Serialize};

#[derive(Debug, serde::Deserialize)]
pub struct NewWarrior {
    pub name: String,
    pub dob: String,
    pub skills: Vec<String>
}

#[derive(Debug, Deserialize, Serialize, FromRow)]
pub struct Warrior {
    pub id: String,
    pub name: String,
    pub dob: String,
    pub fight_skills: Option<Vec<String>>
}

#[derive(Debug, Deserialize, Serialize, Encode, Decode)]
pub struct WarriorSkill {
    pub name: String
}