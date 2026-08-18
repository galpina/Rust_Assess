mod data;
mod handlers;
mod models;

use actix_cors::Cors;
use actix_web::{middleware::Logger, web, App, HttpServer};
use data::DataStore;

const DEFAULT_PORT: u16 = 8080;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Read PORT from environment, fall back to 8080
    let port: u16 = std::env::var("PORT")
        .ok()
        .and_then(|p| p.parse().ok())
        .unwrap_or(DEFAULT_PORT);

    // Shared, thread-safe data store
    let store = web::Data::new(DataStore::new());

    println!("Rust backend server starting on http://localhost:{}", port);
    println!("Serving data directly from Rust backend");

    HttpServer::new(move || {
        // Allow all origins (mirrors the Rust backend behaviour)
        let cors = Cors::default()
            .allow_any_origin()
            .allow_any_method()
            .allow_any_header();

        App::new()
            .wrap(cors)
            .wrap(Logger::default())
            .app_data(store.clone())
            // Health
            .route("/health", web::get().to(handlers::handle_health))
            // Users
            .route("/api/users",     web::get().to(handlers::handle_users))
            .route("/api/users/{id}", web::get().to(handlers::handle_user_by_id))
            // Tasks
            .route("/api/tasks", web::get().to(handlers::handle_tasks))
            // Stats
            .route("/api/stats", web::get().to(handlers::handle_stats))
    })
    .bind(("0.0.0.0", port))?
    .run()
    .await
}
