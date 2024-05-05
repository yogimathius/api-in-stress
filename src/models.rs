use serde::{Deserialize, Serialize};
use serde_valid::Validate;
use sqlx::{Decode, Encode, FromRow};

#[derive(Debug, Validate, Deserialize, Serialize, Clone)]
pub struct NewWarrior {
    pub name: String,
    pub dob: String,
    #[validate(min_items = 1, message = "At least one skill is required")]
    #[validate(max_items = 20, message = "Maximum of 20 skills allowed")]
    pub fight_skills: Vec<String>,
}

#[derive(Debug, Deserialize, Serialize, FromRow, Clone)]
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
