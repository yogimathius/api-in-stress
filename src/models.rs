use sqlx::{FromRow, Encode};

#[derive(Debug, serde::Deserialize, Encode)]
pub struct NewWarrior {
    pub name: String,
    pub dob: String,
}

#[derive(Debug, serde::Serialize, FromRow)]
pub struct Warrior {
    pub id: String,
    pub name: String,
    pub dob: String,
}
