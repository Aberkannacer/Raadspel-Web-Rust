mod state;
mod routes;
mod templates;

use actix_files::Files;
use actix_web::{web, App, HttpServer};
use state::AppState;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let data = web::Data::new(AppState::new());
    let server_url = "127.0.0.1:8080";

    println!("\n🚀 Server is running!");
    println!("📱 Open je browser en ga naar: http://{}", server_url);
    println!("🎮 Veel plezier met het raadspel!\n");

    HttpServer::new(move || {
        App::new()
            .app_data(data.clone())
            .service(Files::new("/static", "./static"))
            .route("/", web::get().to(routes::index))
            .route("/guess", web::post().to(routes::guess))
    })
    .bind(server_url)?
    .run()
    .await
}
