use std::collections::HashMap;
use std::sync::Mutex;
use rand::Rng;

pub struct AppState {
    pub secret_number: Mutex<u32>,
    pub guesses: Mutex<Vec<u32>>,
    pub attempts: Mutex<u32>,
    pub scoreboard: Mutex<HashMap<u32, u32>>,
    pub last_feedback: Mutex<String>,
}

impl AppState {
    pub fn new() -> Self {
        Self {
            secret_number: Mutex::new(rand::thread_rng().gen_range(1..=100)),
            guesses: Mutex::new(Vec::new()),
            attempts: Mutex::new(0),
            scoreboard: Mutex::new(HashMap::new()),
            last_feedback: Mutex::new(String::from("Welkom bij het Raadspel! Raad het getal tussen 1 en 100:")),
        }
    }
}
