mod state;
mod routes;
mod templates;

use actix_files::Files;
use actix_web::{web, App, HttpServer, middleware::Logger};
use state::AppState;
use std::env;
use env_logger;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Initialize logger
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("debug"));

    let data = web::Data::new(AppState::new());
    
    let port = env::var("PORT").unwrap_or_else(|_| "8080".to_string());
    let server_url = format!("0.0.0.0:{}", port);

    println!("\n🚀 Server starting on {}", server_url);
    eprintln!("Debug: Initializing server..."); // Extra debug output
    
    HttpServer::new(move || {
        println!("Creating new app instance"); // Extra debug output
        App::new()
            .wrap(Logger::default())
            .wrap(Logger::new("%a %r %s %b %{Referer}i %{User-Agent}i %T"))
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
