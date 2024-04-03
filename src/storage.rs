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

    pub async fn create_warrior(&self, warrior: Warrior) -> &'static str {
        self.warriors.lock().unwrap().push(warrior);
        "Warrior created"
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
        self.warriors.lock().unwrap().clone()
    }

    pub async fn count_warriors(&self) -> usize {
        self.warriors.lock().unwrap().len()
    }
}
