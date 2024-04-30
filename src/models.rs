use serde::{Deserialize, Serialize};
use serde_valid::validation::Error;
use serde_valid::Validate;
use sqlx::{Decode, Encode, FromRow};
// use validator::{Validate, ValidationError};

#[derive(Debug, Validate, Deserialize, Serialize, Clone)]
pub struct NewWarrior {
    pub name: String,
    pub dob: String,
    #[validate(min_items = 1)]
    #[validate(max_items = 20)]
    #[validate(custom(validate_skills))]
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

fn validate_skills(fight_skills: &Vec<String>) -> Result<(), Error> {
    let valid_skills = vec![
        "BJJ",
        "Karate",
        "Judo",
        "KungFu",
        "Capoeira",
        "Boxing",
        "Taekwondo",
        "Aikido",
        "KravMaga",
        "MuayThai",
        "KickBoxing",
        "Pankration",
        "Wrestling",
        "Sambo",
        "Savate",
        "Sumo",
        "Kendo",
        "Hapkido",
        "LutaLivre",
        "WingChu",
        "Ninjutsu",
        "Fencing",
        "ArmWrestling",
        "SuckerPunch",
        "44Magnum",
    ];

    // Check if all fight_skills are valid
    for skill in fight_skills {
        if !valid_skills.contains(&skill.as_str()) {
            return Err(Error::Custom("Invalid skill".to_owned()));
        }
    }

    Ok(())
}
