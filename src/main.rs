use actix_files::Files;
use actix_web::{web, App, HttpServer};
use std::env;

mod routes;
mod state;
mod templates;

use routes::{health_check, index, guess};
use state::AppState;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Lees de poort uit de omgevingsvariabele of gebruik standaard 8080
    let port = env::var("PORT").unwrap_or_else(|_| "8080".to_string());
    let address = format!("0.0.0.0:{}", port);

    println!("Server wordt gestart op: {}", address);

    let data = web::Data::new(AppState::new());

    HttpServer::new(move || {
        App::new()
            .app_data(data.clone()) // Deel de state tussen routes
            .service(Files::new("/static", "./static")) // Serveer statische bestanden
            .route("/", web::get().to(index)) // Hoofdpagina
            .route("/guess", web::post().to(guess)) // Raad-route
            .route("/health", web::get().to(health_check)) // Health check route
    })
    .bind(&address)? // Bind de server aan het adres
    .run()
    .await
}
