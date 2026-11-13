use axum::{Json, Router, response::IntoResponse, routing::post};
use serde::{Serialize, Deserialize};
use axum::extract::State;
use crate::state::AppState;

#[derive(Serialize, Deserialize)]
struct Res {
    message: String
} 

async fn signup(State(state): State<AppState>) -> impl IntoResponse {
    Json(Res {
        message: String::from("/signup: In development")
    })
}

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/signup", post(signup))
}