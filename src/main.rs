mod user;
mod db;
mod state;

use axum::{Router, routing::get};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenvy::dotenv().ok();

    let database_url = std::env::var("DATABASE_URL")
    .expect("DATABASE_URL must be set");

    let db = db::connect(&database_url).await.unwrap();
    
    let state = state::AppState { db };

    let app = Router::new()
        .merge(user::controller::router())
        .with_state(state);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();

    axum::serve(listener, app).await.unwrap();

    Ok(())
}
