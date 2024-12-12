use rand::Rng;
use actix_web::{web, HttpResponse};
use crate::state::AppState;
use crate::templates;

pub async fn index(state: web::Data<AppState>) -> HttpResponse {
    templates::render_page("Welkom bij het Raadspel! Raad het getal tussen 1 en 100:", &state)
}

pub async fn guess(
    state: web::Data<AppState>,
    form: web::Form<std::collections::HashMap<String, String>>,
) -> HttpResponse {
    let guess: u32 = match form.get("guess").and_then(|g| g.parse().ok()) {
        Some(num) => num,
        None => {
            return templates::render_page("Ongeldige invoer! Voer een geldig getal in.", &state);
        }
    };

    let mut secret_number = state.secret_number.lock().unwrap();
    {
        let mut guesses = state.guesses.lock().unwrap();
        guesses.push(guess);
    }
    {
        let mut attempts = state.attempts.lock().unwrap();
        *attempts += 1;
    }

    let feedback = if guess < *secret_number {
        "Te laag! Probeer opnieuw.".to_string()
    } else if guess > *secret_number {
        "Te hoog! Probeer opnieuw.".to_string()
    } else {
        let mut scoreboard = state.scoreboard.lock().unwrap();
        scoreboard.insert(*secret_number, *state.attempts.lock().unwrap());

        *secret_number = rand::thread_rng().gen_range(1..=100);
        *state.guesses.lock().unwrap() = Vec::new();
        *state.attempts.lock().unwrap() = 0;

        "Gefeliciteerd! Je hebt het juiste getal geraden. Een nieuw getal is gegenereerd!"
            .to_string()
    };

    templates::render_page(&feedback, &state)
}

pub async fn health_check() -> HttpResponse {
    HttpResponse::Ok().body("Server is running!")
}
