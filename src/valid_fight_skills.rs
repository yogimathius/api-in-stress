use std::collections::HashMap;

#[derive(Clone)]
pub struct ValidFightSkills {
    skills: HashMap<String, bool>,
}

impl ValidFightSkills {
    pub fn new() -> Self {
        let mut skills = HashMap::new();
        // Initialize skills hashmap with the provided list of skills
        let skill_list = vec![
            "BJJ", "Karate", "Judo", "KungFu", "Capoeira", "Boxing", "Taekwondo", "Aikido",
            "KravMaga", "MuayThai", "KickBoxing", "Pankration", "Wrestling", "Sambo", "Savate",
            "Sumo", "Kendo", "Hapkido", "LutaLivre", "WingChu", "Ninjutsu", "Fencing",
            "ArmWrestling", "SuckerPunch", "44Magnum", "Swordsmanship", "Archery", "Magic",
            "Stealth", "Leadership", "Survival", "Alchemy", "Tactics", "Hand-to-hand Combat",
            "Marksmanship", "Sorcery", "Diplomacy", "Navigation", "Intelligence", "Tracking",
            "Healing", "Engineering", "Acrobatics", "Animal Handling", "Music", "Empathy",
            "Negotiation", "Persuasion",
        ];
        for skill in skill_list {
            skills.insert(String::from(skill), true);
        }
        ValidFightSkills { skills }
    }

    pub fn are_valid_skills(&self, skills: &Vec<std::string::String>) -> bool {
        skills.iter().all(|skill| self.skills.contains_key(skill)) && skills.len() > 0 && skills.len() <= 20
    }    
}
