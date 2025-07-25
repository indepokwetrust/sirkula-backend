use axum::{Router, routing::post};
use sirkula_backend::{AppState, api::register};
use sqlx::sqlite::SqlitePoolOptions;
use std::net::SocketAddr;

#[tokio::main]
async fn main() {
    let args: Vec<String> = std::env::args().collect();
    let db_uri = format!(
        "sqlite://{}",
        args.get(1).expect("Provide database file path")
    );

    let pool = SqlitePoolOptions::new().connect(&db_uri).await.unwrap();
    let state = AppState::new(pool);

    let api_route = Router::new().route("/entity/register", post(register));
    // .route("/entity/login", post(login));

    let app = Router::new().nest("/api", api_route).with_state(state);

    let addr = SocketAddr::from(([0, 0, 0, 0], 3000));
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    println!("Listening on {}", addr);
    axum::serve(listener, app).await.unwrap();
}
