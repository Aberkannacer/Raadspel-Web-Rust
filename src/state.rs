use rand::Rng;
use std::collections::HashMap;
use std::sync::Mutex;


pub struct AppState {
    pub secret_number: Mutex<u32>,        // Het geheime nummer
    pub guesses: Mutex<Vec<u32>>,        // Lijst met geraden getallen
    pub attempts: Mutex<u32>,            // Aantal pogingen
    pub scoreboard: Mutex<HashMap<u32, u32>>, // Permanent scorebord
}

impl AppState {
    pub fn new() -> Self {
        Self {
            secret_number: Mutex::new(rand::thread_rng().gen_range(1..=100)),
            guesses: Mutex::new(Vec::new()),
            attempts: Mutex::new(0),
            scoreboard: Mutex::new(HashMap::new()),
        }
    }
}
  