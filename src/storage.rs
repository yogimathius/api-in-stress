use std::sync::{Arc, Mutex};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub(crate) struct Warrior {
    id: String,
    name: String,
    dob: String,
    fight_skills: Vec<String>,
}

// In-memory storage for warriors
#[derive(Default, Debug, Clone)]
pub(crate) struct Storage {
    warriors: Arc<Mutex<Vec<Warrior>>>,
}

impl Storage {
    pub fn new() -> Self {
        Self {
            warriors: Arc::new(Mutex::new(Vec::new())),
        }
    }

    pub async fn create_warrior(&self, warrior: Warrior) -> Option<Warrior> {
        self.warriors.lock().unwrap().push(warrior.clone());

        Some(warrior)
    }

    pub async fn get_warrior(&self, id: String) -> Option<Warrior> {
        self.warriors
            .lock()
            .unwrap()
            .iter()
            .find(|w| w.id == id)
            .cloned()
    }

    pub async fn search_warriors(&self, _term: String) -> Vec<Warrior> {
        let warriors = self.warriors.lock().unwrap();
        let num_warriors = std::cmp::min(warriors.len(), 50);
        warriors[0..num_warriors].to_vec()
    }
    

    pub async fn count_warriors(&self) -> usize {
        println!("warriors length: {:?}", self.warriors.lock().unwrap().len());
        self.warriors.lock().unwrap().len()
    }

    pub async fn initialize_data(&self) {
        let mut warriors = Vec::new();

        for i in 1..=75 {
            let warrior = Warrior {
                id: i.to_string(),
                name: format!("Warrior {}", i),
                dob: format!("19{:02}-{:02}-{:02}", i % 100, (i * 2) % 12 + 1, (i * 3) % 28 + 1),
                fight_skills: vec![format!("Skill {}", i * 2), format!("Skill {}", i * 2 + 1)],
            };
        
            warriors.push(warrior);
        }
        
        
        for warrior in warriors {
            self.create_warrior(warrior).await;
        }
    }
}
