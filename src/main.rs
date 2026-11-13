mod user;
mod db;

use axum::{Router, routing::get};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenvy::dotenv().ok();

    let database_url = std::env::var("DATABASE_URL")
    .expect("DATABASE_URL must be set");

    let db = db::connect(&database_url).await.unwrap();

    let result: i32 = sqlx::query_scalar("SELECT 1")
        .fetch_one(&db)
        .await?;

    let app = Router::new()
        .merge(user::controller::router())
        .route("/test", get(move || async move { result.to_string() }));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();

    axum::serve(listener, app).await.unwrap();

    Ok(())
}
