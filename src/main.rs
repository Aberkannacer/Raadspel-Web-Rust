mod state;
mod routes;
mod templates;

use actix_files::Files;
use actix_web::{web, App, HttpServer};
use state::AppState;
use std::env;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let data = web::Data::new(AppState::new());
    
    // Krijg PORT van environment variable (voor Heroku) of gebruik 8080 als fallback
    let port = env::var("PORT").unwrap_or_else(|_| "8080".to_string());
    let server_url = format!("0.0.0.0:{}", port);

    println!("\n🚀 Server is running!");
    println!("📱 Server listening on: http://{}", server_url);
    
    HttpServer::new(move || {
        App::new()
            .app_data(data.clone())
            .service(Files::new("/static", "./static"))
            .route("/health", web::get().to(routes::health_check))
            .route("/", web::get().to(routes::index))
            .route("/guess", web::post().to(routes::guess))
    })
    .bind(&server_url)?
    .workers(2)
    .run()
    .await
}
