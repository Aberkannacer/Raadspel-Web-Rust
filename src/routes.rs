use actix_web::{web, HttpResponse, http::header};
use crate::state::AppState;
use crate::templates;
use rand::Rng;


pub async fn index(state: web::Data<AppState>) -> HttpResponse {
    let feedback = state.last_feedback.lock().unwrap().clone();
    templates::render_page(&feedback, &state)
}

pub async fn guess(
    state: web::Data<AppState>,
    form: web::Form<std::collections::HashMap<String, String>>,
) -> HttpResponse {
    if !form.contains_key("guess") {
        return HttpResponse::SeeOther()
            .insert_header((header::LOCATION, "/"))
            .finish();
    }

    let guess: u32 = match form.get("guess").and_then(|g| g.parse().ok()) {
        Some(num) => num,
        None => {
            return HttpResponse::SeeOther()
                .insert_header((header::LOCATION, "/"))
                .finish();
        }
    };

    if guess < 1 || guess > 100 {
        return HttpResponse::SeeOther()
            .insert_header((header::LOCATION, "/"))
            .finish();
    }

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
        "Te laag! Probeer opnieuw."
    } else if guess > *secret_number {
        "Te hoog! Probeer opnieuw."
    } else {
        let mut scoreboard = state.scoreboard.lock().unwrap();
        scoreboard.insert(*secret_number, *state.attempts.lock().unwrap());

        *secret_number = rand::thread_rng().gen_range(1..=100);
        *state.guesses.lock().unwrap() = Vec::new();
        *state.attempts.lock().unwrap() = 0;

        "Gefeliciteerd! Je hebt het juiste getal geraden. Een nieuw getal is gegenereerd!"
    };

    let mut last_feedback = state.last_feedback.lock().unwrap();
    *last_feedback = feedback.to_string();

    HttpResponse::SeeOther()
        .insert_header((header::LOCATION, "/"))
        .finish()
}

pub async fn health_check() -> HttpResponse {
    HttpResponse::Ok().body("Server is running!")
}
