use axum::{Json, Router, response::IntoResponse, routing::post};
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
struct Res {
    message: String
} 

async fn signup() -> impl IntoResponse {
    Json(Res {
        message: String::from("/signup: In development")
    })
}

async fn login() -> impl IntoResponse {
    Json(Res {
        message: String::from("/login: In development")
    })
}

pub fn router() -> Router {
    Router::new()
        .route("/signup", post(signup))
        .route("/login", post(login))
}