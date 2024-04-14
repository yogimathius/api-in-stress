use sqlx::{FromRow, Encode};
use serde::{Deserialize, Serialize};

#[derive(Debug, serde::Deserialize, Encode)]
pub struct NewWarrior {
    pub name: String,
    pub dob: String,
}

#[derive(Debug, Deserialize, Serialize, FromRow)]
pub struct Warrior {
    pub id: String,
    pub name: String,
    pub dob: String,
}
