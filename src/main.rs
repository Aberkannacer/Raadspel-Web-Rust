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
    
    let port = env::var("PORT").unwrap_or_else(|_| "8080".to_string());
    let server_url = format!("0.0.0.0:{}", port);

    println!("\n🚀 Server starting on {}", server_url);
    
    HttpServer::new(move || {
        App::new()
            .wrap(actix_web::middleware::Logger::default())
            .app_data(data.clone())
            .service(Files::new("/static", "./static").show_files_listing())
            .route("/health", web::get().to(routes::health_check))
            .route("/", web::get().to(routes::index))
            .route("/guess", web::post().to(routes::guess))
    })
    .bind(&server_url)?
    .workers(2)
    .run()
    .await
}
