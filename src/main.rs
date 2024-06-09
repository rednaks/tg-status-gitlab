mod bot;
mod handlers;
mod status_io;
use std::env;

use axum::{routing::post, Router};

#[tokio::main]
async fn main() {
    let app = Router::new().route("/", post(handlers::handle_notification));

    let listener = tokio::net::TcpListener::bind(format!(
        "{}:{}",
        env::var("HOST").unwrap_or(String::from("0.0.0.0")),
        env::var("PORT").unwrap_or(String::from("9000")),
    ))
    .await
    .unwrap();
    axum::serve(listener, app).await.unwrap();
}
