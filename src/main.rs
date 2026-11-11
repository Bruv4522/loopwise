use axum::{
    Json, Router, routing::{get, post}
};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
struct Res {
    status: String,
    service: String
}

async fn echo(Json(payload): Json<Res>) -> Json<Res> {
    Json(payload)
}

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/health", get(|| async {
            Json(Res {
                status: String::from("Ok"),
                service: String::from("Loopwise")
            })
        } ))
        .route("/debug/echo", post(echo));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000")
        .await
        .unwrap();
    
    axum::serve(listener, app)
        .await
        .unwrap();
}
