use actix_web::HttpResponse;
use crate::state::AppState;

pub fn render_page(feedback: &str, state: &AppState) -> HttpResponse {
    let guesses = state.guesses.lock().unwrap();
    let attempts = state.attempts.lock().unwrap();
    let scoreboard = state.scoreboard.lock().unwrap();

    let guesses_list = guesses
        .iter()
        .map(|guess| format!("<li>{}</li>", guess))
        .collect::<Vec<String>>()
        .join("");

    let scoreboard_list = scoreboard
        .iter()
        .map(|(secret, attempts)| format!("<li>Nummer: {} - Pogingen: {}</li>", secret, attempts))
        .collect::<Vec<String>>()
        .join("");

    HttpResponse::Ok().body(format!(
        r#"
        <!DOCTYPE html>
        <html>
            <head>
                <title>Raad het Getal</title>
                <link rel="stylesheet" type="text/css" href="/static/styles.css">
            </head>
            <body>
                <h1>Raad het Getal!</h1>
                <p>{}</p>
                <form method="post" action="/guess">
                    <label>Voer je gok in:</label>
                    <input type="number" name="guess" required>
                    <button type="submit">Verzenden</button>
                </form>
                <h2>Huidige Pogingen</h2>
                <p>Aantal pogingen: {}</p>
                <ul>{}</ul>
                <h2>Scorebord</h2>
                <ul>{}</ul>
            </body>
        </html>
        "#,
        feedback, *attempts, guesses_list, scoreboard_list
    ))
}
